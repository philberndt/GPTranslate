use crate::config::Config;
use crate::provider_factory::create_provider;
use crate::trans_azure::AzureOpenAITranslationService;
use crate::trans_ollama::OllamaTranslationService;
use crate::trans_openai::OpenAITranslationService;
use anyhow::Result;
use async_trait::async_trait;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref IN_FLIGHT_REQUESTS: Arc<Mutex<HashMap<String, std::time::Instant>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub detected_language: String,
    pub translated_text: String,
    pub target_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeTranslationsResult {
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub original_text: String,
    pub translated_text: String,
    pub detected_language: String,
    pub target_language: String,
}

#[derive(Debug)]
pub enum Error {
    DuplicateRequest,
    ApiError(anyhow::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateRequest => write!(f, "Duplicate request"),
            Error::ApiError(e) => write!(f, "API error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::ApiError(error)
    }
}

#[async_trait]
pub trait TranslationProvider {
    async fn translate(&self, text: &str) -> Result<TranslationResult>;
}

pub struct TranslationService {
    provider: Box<dyn TranslationProvider + Send + Sync>,
}

impl TranslationService {
    pub fn new(config: Config) -> Self {
        Self {
            provider: create_provider(config),
        }
    }

    pub async fn detect_and_translate(&self, text: &str) -> Result<TranslationResult> {
        // Create a more unique request key that includes current timestamp to prevent issues
        // with legitimate duplicate requests (e.g., user retrying the same text)
        let request_key = format!(
            "{}-{}",
            text.len(),
            text.chars().take(50).collect::<String>()
        );

        let now = std::time::Instant::now();

        {
            let mut requests = IN_FLIGHT_REQUESTS.lock().unwrap();

            // Clean up old requests (older than 5 seconds)
            requests.retain(|_, &mut timestamp| now.duration_since(timestamp).as_secs() < 5);

            // Check if there's a recent request for the same content
            if let Some(&request_time) = requests.get(&request_key) {
                let time_diff = now.duration_since(request_time);
                // Only consider it a duplicate if it's within 500ms
                if time_diff.as_millis() < 500 {
                    log::info!(
                        "Duplicate translation request detected within 500ms, skipping API call"
                    );
                    return Err(anyhow::anyhow!("Duplicate request detected"));
                }
            }

            requests.insert(request_key.clone(), now);
        }

        let result = self.provider.translate(text).await;

        {
            let mut requests = IN_FLIGHT_REQUESTS.lock().unwrap();
            requests.remove(&request_key);
        }

        result
    }
}

pub fn clean_text_for_translation(text: &str) -> String {
    // Improve text cleaning to preserve paragraph structure
    // Instead of filtering out empty lines, preserve them as paragraph breaks
    text.lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn create_smart_prompt(config: &Config, _source_override: Option<&str>) -> String {
    let prompt = format!(
        "{}\n\n# Translation Rules\n- Primary target language: {}\n- Alternative target language: {}\n\n**IMPORTANT**: \n- If the detected source language is the same as the primary target language ({}), then translate to the alternative target language ({}) instead.\n- If the detected source language is different from the primary target language ({}), then translate to the primary target language ({}).\n\nExample:\n- If text is in {} and primary target is {}, translate to {}\n- If text is in any other language and primary target is {}, translate to {}",
        config.custom_prompt,
        config.target_language,
        config.alternative_target_language,
        config.target_language,
        config.alternative_target_language,
        config.target_language,
        config.target_language,
        config.target_language,
        config.target_language,
        config.alternative_target_language,
        config.target_language,
        config.target_language
    );

    log::info!("Generated smart prompt: {}", prompt);
    prompt
}

pub async fn translate_text(
    text: String,
    config: tauri::State<'_, crate::AppState>,
) -> Result<TranslationResponse, Error> {
    log::info!("translate_text called with text: {}", text);

    let config_guard = config.config.lock().await;
    let config_clone = config_guard.clone();
    drop(config_guard);

    log::info!(
        "Config loaded - API Provider: {}, Model: {}, Target: {}, Alternative: {}, Custom Prompt: {}",
        config_clone.api_provider,
        config_clone.model,
        config_clone.target_language,
        config_clone.alternative_target_language,
        config_clone.custom_prompt
    );

    let service = TranslationService::new(config_clone.clone());
    match service.detect_and_translate(&text).await {
        Ok(result) => {
            log::info!("Translation completed successfully");
            log::info!("Detected language: {}", result.detected_language);
            log::info!(
                "Translated text length: {} characters",
                result.translated_text.len()
            );

            // Determine the actual target language that was used for translation
            let detected_lower = result.detected_language.to_lowercase();
            let target_lower = config_clone.target_language.to_lowercase();

            log::info!(
                "Language comparison: detected='{}', target='{}'",
                detected_lower,
                target_lower
            );
            let expected_target_language = if detected_lower == target_lower {
                log::info!(
                    "Detected language '{}' matches target language '{}' - using alternative target language '{}'",
                    result.detected_language,
                    config_clone.target_language,
                    config_clone.alternative_target_language
                );
                // If detected language matches target language, we used the alternative
                config_clone.alternative_target_language.clone()
            } else {
                log::info!(
                    "Detected language '{}' does not match target language '{}' - using primary target language '{}'",
                    result.detected_language,
                    config_clone.target_language,
                    config_clone.target_language
                );
                // Otherwise, we used the configured target language
                config_clone.target_language.clone()
            };

            Ok(TranslationResponse {
                original_text: text,
                translated_text: result.translated_text,
                detected_language: result.detected_language,
                target_language: expected_target_language,
            })
        }
        Err(e) => {
            log::error!("Translation failed: {}", e);
            if e.to_string().contains("Duplicate request detected") {
                Err(Error::DuplicateRequest)
            } else {
                Err(Error::ApiError(e))
            }
        }
    }
}

/// Generate alternative translations for a given text (debug version with detailed output)
pub async fn get_alternative_translations_debug(
    selected_text: String,
    target_language: String,
    config: tauri::State<'_, crate::AppState>,
) -> Result<serde_json::Value, Error> {
    let mut debug_info = serde_json::Map::new();
    debug_info.insert("selected_text".to_string(), serde_json::Value::String(selected_text.clone()));
    debug_info.insert("target_language".to_string(), serde_json::Value::String(target_language.clone()));

    let config_guard = config.config.lock().await;
    let config_clone = config_guard.clone();
    drop(config_guard);

    // Create a special prompt for alternative translations
    let alternatives_prompt = format!(
        "Provide up to 5 different word choices or synonyms for the following text when translated into {}. Return ONLY the alternative words/phrases as a JSON array under the key 'alternatives'. Do not include the original word. If no alternatives exist, return an empty array. Do not include any other text or explanation.\n\nText: \"{}\"\n\nRespond in JSON format like this:\n{{\"alternatives\": [\"Alternative 1\", \"Alternative 2\", \"Alternative 3\"]}}",
        target_language, selected_text
    );

    debug_info.insert("prompt".to_string(), serde_json::Value::String(alternatives_prompt.clone()));
    debug_info.insert("api_provider".to_string(), serde_json::Value::String(config_clone.api_provider.clone()));
    debug_info.insert("model".to_string(), serde_json::Value::String(config_clone.model.clone()));

    // Create a service with the alternatives prompt
    let alternatives_service: Box<dyn TranslationProvider + Send + Sync> = if config_clone
        .api_provider
        == "azure_translator"
    {
        // Azure Translator can't generate alternatives, use fallback
        if let Some((provider, model)) = config_clone.parse_alternatives_fallback() {
            log::info!(
                "Using fallback provider '{}' with model '{}' for alternatives",
                provider,
                model
            );
            let mut alt_config = config_clone.clone();
            alt_config.api_provider = provider.clone();
            alt_config.model = model.clone();
            alt_config.custom_prompt = alternatives_prompt;
            
            // For Azure OpenAI, ensure deployment name matches model name
            if provider == "azure_openai" {
                log::info!("Setting Azure deployment name to: {}", model);
                alt_config.azure_deployment_name = model.clone();
            }
            alt_config.ensure_azure_deployment_consistency();
            
            debug_info.insert("using_fallback".to_string(), serde_json::Value::Bool(true));
            debug_info.insert("fallback_provider".to_string(), serde_json::Value::String(provider.clone()));
            debug_info.insert("fallback_model".to_string(), serde_json::Value::String(model.clone()));
            debug_info.insert("azure_deployment_name".to_string(), serde_json::Value::String(alt_config.azure_deployment_name.clone()));
            
            // Ensure Azure deployment consistency before creating service
            alt_config.ensure_azure_deployment_consistency();
            
            match alt_config.api_provider.as_str() {
                "openai" => Box::new(OpenAITranslationService::new(alt_config)),
                "azure_openai" => Box::new(AzureOpenAITranslationService::new(alt_config)),
                "ollama" => Box::new(OllamaTranslationService::new(alt_config)),
                _ => {
                    debug_info.insert("fallback_error".to_string(), serde_json::Value::String("Unknown provider, defaulting to OpenAI".to_string()));
                    alt_config.api_provider = "openai".to_string();
                    Box::new(OpenAITranslationService::new(alt_config))
                }
            }
        } else {
            debug_info.insert("error".to_string(), serde_json::Value::String("Azure Translator cannot generate alternatives. Please configure a fallback provider in settings.".to_string()));
            return Ok(serde_json::Value::Object(debug_info));
        }
    } else {
        // Use the current provider for alternatives with custom prompt
        debug_info.insert("using_fallback".to_string(), serde_json::Value::Bool(false));
        let mut alt_config = config_clone.clone();
        alt_config.custom_prompt = alternatives_prompt;
        
        // Ensure Azure deployment consistency before creating service
        alt_config.ensure_azure_deployment_consistency();
        
        match alt_config.api_provider.as_str() {
            "openai" => Box::new(OpenAITranslationService::new(alt_config)),
            "azure_openai" => Box::new(AzureOpenAITranslationService::new(alt_config)),
            "ollama" => Box::new(OllamaTranslationService::new(alt_config)),
            _ => {
                debug_info.insert("provider_error".to_string(), serde_json::Value::String("Unknown API provider, defaulting to OpenAI".to_string()));
                alt_config.api_provider = "openai".to_string();
                Box::new(OpenAITranslationService::new(alt_config))
            }
        }
    };

    // Use the service to get alternatives
    match alternatives_service.translate(&selected_text).await {
        Ok(result) => {
            let response_text = result.translated_text.trim();
            debug_info.insert("raw_response".to_string(), serde_json::Value::String(response_text.to_string()));
            debug_info.insert("response_length".to_string(), serde_json::Value::Number(serde_json::Number::from(response_text.len())));

            // Try to parse as JSON first
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response_text) {
                debug_info.insert("json_parse_success".to_string(), serde_json::Value::Bool(true));
                debug_info.insert("parsed_json".to_string(), parsed.clone());
                
                if let Some(alternatives_array) = parsed.get("alternatives").and_then(|a| a.as_array()) {
                    let raw_alternatives: Vec<String> = alternatives_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    debug_info.insert("raw_alternatives".to_string(), serde_json::Value::Array(raw_alternatives.iter().map(|s| serde_json::Value::String(s.clone())).collect()));
                    
                    let alternatives: Vec<String> = raw_alternatives
                        .into_iter()
                        .filter(|alt| alt.trim().to_lowercase() != selected_text.trim().to_lowercase())
                        .collect();
                    debug_info.insert("filtered_alternatives".to_string(), serde_json::Value::Array(alternatives.iter().map(|s| serde_json::Value::String(s.clone())).collect()));
                    debug_info.insert("final_count".to_string(), serde_json::Value::Number(serde_json::Number::from(alternatives.len())));
                } else {
                    debug_info.insert("alternatives_array_found".to_string(), serde_json::Value::Bool(false));
                }
            } else {
                debug_info.insert("json_parse_success".to_string(), serde_json::Value::Bool(false));
                debug_info.insert("json_parse_error".to_string(), serde_json::Value::String("Failed to parse as JSON".to_string()));
            }

            Ok(serde_json::Value::Object(debug_info))
        }
        Err(e) => {
            debug_info.insert("service_error".to_string(), serde_json::Value::String(e.to_string()));
            Ok(serde_json::Value::Object(debug_info))
        }
    }
}

/// Generate alternative translations for a given text
pub async fn get_alternative_translations(
    selected_text: String,
    target_language: String,
    config: tauri::State<'_, crate::AppState>,
) -> Result<AlternativeTranslationsResult, Error> {
    log::info!(
        "get_alternative_translations called with text: '{}' target: '{}'",
        selected_text,
        target_language
    );

    let config_guard = config.config.lock().await;
    let config_clone = config_guard.clone();
    drop(config_guard);

    // Create a special prompt for alternative translations
    let alternatives_prompt = format!(
        "Provide up to 5 different word choices or synonyms for the following text when translated into {}. Return ONLY the alternative words/phrases as a JSON array under the key 'alternatives'. Do not include the original word. If no alternatives exist, return an empty array. Do not include any other text or explanation.\n\nText: \"{}\"\n\nRespond in JSON format like this:\n{{\"alternatives\": [\"Alternative 1\", \"Alternative 2\", \"Alternative 3\"]}}",
        target_language, selected_text
    );

    // Create a service with the alternatives prompt
    let alternatives_service: Box<dyn TranslationProvider + Send + Sync> = if config_clone
        .api_provider
        == "azure_translator"
    {
        // Azure Translator can't generate alternatives, use fallback
        if let Some((provider, model)) = config_clone.parse_alternatives_fallback() {
            log::info!(
                "Using fallback provider '{}' with model '{}' for alternatives",
                provider,
                model
            );
            let mut alt_config = config_clone.clone();
            alt_config.api_provider = provider.clone();
            alt_config.model = model.clone();
            alt_config.custom_prompt = alternatives_prompt;
            
            // For Azure OpenAI, ensure deployment name matches model name
            if provider == "azure_openai" {
                log::info!("Setting Azure deployment name to: {}", model);
                alt_config.azure_deployment_name = model.clone();
            }
            alt_config.ensure_azure_deployment_consistency();
            
            match alt_config.api_provider.as_str() {
                "openai" => Box::new(OpenAITranslationService::new(alt_config)),
                "azure_openai" => Box::new(AzureOpenAITranslationService::new(alt_config)),
                "ollama" => Box::new(OllamaTranslationService::new(alt_config)),
                _ => {
                    log::warn!(
                        "Unknown alternatives provider '{}', defaulting to OpenAI",
                        provider
                    );
                    alt_config.api_provider = "openai".to_string();
                    Box::new(OpenAITranslationService::new(alt_config))
                }
            }
        } else {
            return Err(Error::ApiError(anyhow::anyhow!(
                "Azure Translator cannot generate alternatives. Please configure a fallback provider in settings."
            )));
        }
    } else {
        // Use the current provider for alternatives with custom prompt
        log::info!(
            "Using current provider '{}' for alternatives",
            config_clone.api_provider
        );
        let mut alt_config = config_clone.clone();
        alt_config.custom_prompt = alternatives_prompt;
        
        // Ensure Azure deployment consistency before creating service
        alt_config.ensure_azure_deployment_consistency();
        
        match alt_config.api_provider.as_str() {
            "openai" => Box::new(OpenAITranslationService::new(alt_config)),
            "azure_openai" => Box::new(AzureOpenAITranslationService::new(alt_config)),
            "ollama" => Box::new(OllamaTranslationService::new(alt_config)),
            _ => {
                log::warn!(
                    "Unknown API provider '{}', defaulting to OpenAI",
                    alt_config.api_provider
                );
                alt_config.api_provider = "openai".to_string();
                Box::new(OpenAITranslationService::new(alt_config))
            }
        }
    };

    // Use the service to get alternatives
    match alternatives_service.translate(&selected_text).await {
        Ok(result) => {
            let response_text = result.translated_text.trim();
            log::info!("Raw AI response for alternatives: '{}'", response_text);

            // Try to parse as JSON first
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response_text) {
                if let Some(alternatives_array) = parsed.get("alternatives").and_then(|a| a.as_array()) {
                    let alternatives: Vec<String> = alternatives_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .filter(|alt| alt.trim().to_lowercase() != selected_text.trim().to_lowercase())
                        .collect();

                    if !alternatives.is_empty() {
                        log::info!("Successfully parsed {} alternatives", alternatives.len());
                        return Ok(AlternativeTranslationsResult { alternatives });
                    }
                }
            }

            // Try to fix and re-parse JSON
            let fixed_json = response_text
                .replace("\\n", "\n")
                .replace("\n", " ")
                .replace("  ", " ")
                .trim()
                .to_string();

            let completed_json = if !fixed_json.ends_with('}') && !fixed_json.ends_with(']') {
                if fixed_json.contains("\"alternatives\": [") && !fixed_json.ends_with(']') {
                    format!("{}]}}", fixed_json.trim_end_matches(','))
                } else {
                    fixed_json
                }
            } else {
                fixed_json
            };

            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&completed_json) {
                if let Some(alternatives_array) = parsed.get("alternatives").and_then(|a| a.as_array()) {
                    let alternatives: Vec<String> = alternatives_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .filter(|alt| alt.trim().to_lowercase() != selected_text.trim().to_lowercase())
                        .collect();

                    if !alternatives.is_empty() {
                        log::info!("Successfully parsed {} alternatives from fixed JSON", alternatives.len());
                        return Ok(AlternativeTranslationsResult { alternatives });
                    }
                }
            }

            // Fallback: extract lines as alternatives
            let alternatives: Vec<String> = response_text
                .lines()
                .map(|line| line.trim())
                .filter(|line| {
                    !line.is_empty() 
                        && !line.starts_with(['#', '-', '{', '}', '[', ']'])
                        && !line.contains("detected_language")
                        && !line.contains("translated_text")
                        && !line.contains("alternatives")
                        && !line.ends_with([':',','])
                        && *line != "translation failed"
                })
                .map(|line| {
                    line.trim_start_matches(['1', '2', '3', '4', '5', '.', ')', '-', '*'])
                        .trim()
                        .trim_matches(['"', ','])
                        .to_string()
                })
                .filter(|line| {
                    !line.is_empty() 
                        && line.len() > 1
                        && !line.chars().all(|c| c.is_ascii_punctuation())
                        && line != "translation failed"
                        && line.trim().to_lowercase() != selected_text.trim().to_lowercase()
                })
                .take(5)
                .collect();

            if !alternatives.is_empty() {
                log::info!("Extracted {} alternatives from text lines", alternatives.len());
                Ok(AlternativeTranslationsResult { alternatives })
            } else {
                log::warn!("No alternatives found in response");
                Err(Error::ApiError(anyhow::anyhow!(
                    "No alternatives found in response"
                )))
            }
        }
        Err(e) => {
            log::error!("Failed to get alternative translations: {}", e);
            Err(Error::ApiError(e))
        }
    }
}
