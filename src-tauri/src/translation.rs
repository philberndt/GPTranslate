use crate::config::Config;
use crate::provider_factory::create_provider;
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
    pub fn new(config: Config) -> Self { Self { provider: create_provider(config) } }

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

/// Generate alternative translations for a given text
pub async fn get_alternative_translations(
    selected_text: String,
    target_language: String,
    config: tauri::State<'_, crate::AppState>,
) -> Result<AlternativeTranslationsResult, Error> {
    log::info!("get_alternative_translations called with text: '{}' target: '{}'", selected_text, target_language);

    let config_guard = config.config.lock().await;
    let config_clone = config_guard.clone();
    drop(config_guard);

    // Create a special prompt for alternative translations
    let alternatives_prompt = format!(
        "Generate 3-5 alternative ways to express the following text in {}: \"{}\"\n\n# Instructions\n- Provide different phrasings that convey the same meaning\n- Focus on different word choices, sentence structures, or stylistic variations\n- Maintain the same tone and formality level\n- Each alternative should be distinct but equivalent in meaning\n- Return only the alternatives as a JSON array of strings\n\n# Output Format\nReturn valid JSON in this exact format:\n{{\"alternatives\": [\"alternative 1\", \"alternative 2\", \"alternative 3\", \"alternative 4\", \"alternative 5\"]}}",
        target_language,
        selected_text
    );

    // Create a service with the alternatives prompt
    let alternatives_service: Box<dyn TranslationProvider + Send + Sync> = if config_clone.api_provider == "azure_translator" {
        // Azure Translator can't generate alternatives, use fallback
        if let Some((provider, model)) = config_clone.parse_alternatives_fallback() {
            log::info!("Using fallback provider '{}' with model '{}' for alternatives", provider, model);
            let mut alt_config = config_clone.clone();
            alt_config.api_provider = provider.clone();
            alt_config.model = model.clone();
            alt_config.custom_prompt = alternatives_prompt;
            match alt_config.api_provider.as_str() {
                "openai" => Box::new(OpenAITranslationService::new(alt_config)),
                "azure_openai" => Box::new(AzureOpenAITranslationService::new(alt_config)),
                "ollama" => Box::new(OllamaTranslationService::new(alt_config)),
                _ => {
                    log::warn!("Unknown alternatives provider '{}', defaulting to OpenAI", provider);
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
        log::info!("Using current provider '{}' for alternatives", config_clone.api_provider);
        let mut alt_config = config_clone.clone();
        alt_config.custom_prompt = alternatives_prompt;
        match alt_config.api_provider.as_str() {
            "openai" => Box::new(OpenAITranslationService::new(alt_config)),
            "azure_openai" => Box::new(AzureOpenAITranslationService::new(alt_config)),
            "ollama" => Box::new(OllamaTranslationService::new(alt_config)),
            _ => {
                log::warn!("Unknown API provider '{}', defaulting to OpenAI", alt_config.api_provider);
                alt_config.api_provider = "openai".to_string();
                Box::new(OpenAITranslationService::new(alt_config))
            }
        }
    };

    // Use the service to get alternatives
    match alternatives_service.translate(&selected_text).await {
        Ok(result) => {
            // Parse the response to extract alternatives
            let response_text = result.translated_text.trim();
            
            // Try to parse as JSON first
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response_text) {
                if let Some(alternatives_array) = parsed.get("alternatives").and_then(|a| a.as_array()) {
                    let alternatives: Vec<String> = alternatives_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    
                    if !alternatives.is_empty() {
                        log::info!("Successfully parsed {} alternatives", alternatives.len());
                        return Ok(AlternativeTranslationsResult { alternatives });
                    }
                }
            }
            
            // Fallback: try to extract lines as alternatives
            let alternatives: Vec<String> = response_text
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with('-'))
                .map(|line| {
                    // Clean up common prefixes
                    line.trim_start_matches(['1', '2', '3', '4', '5', '.', ')', '-', '*'])
                        .trim()
                        .trim_matches('"')
                        .to_string()
                })
                .filter(|line| !line.is_empty())
                .take(5)
                .collect();

            if !alternatives.is_empty() {
                log::info!("Extracted {} alternatives from text lines", alternatives.len());
                Ok(AlternativeTranslationsResult { alternatives })
            } else {
                Err(Error::ApiError(anyhow::anyhow!("No alternatives found in response")))
            }
        }
        Err(e) => {
            log::error!("Failed to get alternative translations: {}", e);
            Err(Error::ApiError(e))
        }
    }
}
