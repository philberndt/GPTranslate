use crate::config::Config;
use crate::translation::{
    TranslationProvider, TranslationResult, clean_text_for_translation, create_smart_prompt,
};
use anyhow::Result;
use async_trait::async_trait;
use ollama_rs::{
    Ollama,
    generation::completion::{GenerationResponse, request::GenerationRequest},
};
use serde_json::{Value, json};

pub struct OllamaTranslationService {
    client: Ollama,
    config: Config,
}

impl OllamaTranslationService {
    pub fn new(config: Config) -> Self {
        // Default to localhost:11434 if no ollama_url is configured
        let ollama_url = config
            .ollama_url
            .as_deref()
            .unwrap_or("http://localhost:11434");

        log::info!(
            "Creating OllamaTranslationService with URL: {}, model: {}",
            ollama_url,
            config.model
        );

        // Parse the URL to extract host and port
        let (host, port) = if let Ok(url) = url::Url::parse(ollama_url) {
            let host = format!(
                "{}://{}",
                url.scheme(),
                url.host_str().unwrap_or("localhost")
            );
            let port = url.port().unwrap_or(11434);
            (host, port)
        } else {
            // Fallback parsing for just host:port
            if let Some(host_port) = ollama_url.strip_prefix("http://") {
                if let Some((host, port_str)) = host_port.split_once(':') {
                    let port = port_str.parse().unwrap_or(11434);
                    (format!("http://{}", host), port)
                } else {
                    (format!("http://{}", host_port), 11434)
                }
            } else {
                ("http://localhost".to_string(), 11434)
            }
        };

        log::info!("Parsed Ollama host: {}, port: {}", host, port);

        let client = Ollama::new(host, port);

        Self { client, config }
    }

    fn parse_response_content(&self, content: &str) -> Result<TranslationResult> {
        log::info!("Ollama Response content: {}", content);

        // Clean the content by removing control characters that can break JSON parsing
        let cleaned_content = content
            .chars()
            .filter(|c| !c.is_control() || matches!(*c, '\n' | '\r' | '\t'))
            .collect::<String>();

        // Log if we removed any control characters
        if cleaned_content != content {
            log::warn!("Removed control characters from Ollama response");
            log::info!("Cleaned content: {}", cleaned_content);
        }

        // Try to parse as JSON, but handle cases where the AI might have returned plain text
        let parsed: Value = match serde_json::from_str(&cleaned_content) {
            Ok(json) => {
                log::info!("Successfully parsed JSON response from Ollama");
                json
            }
            Err(parse_error) => {
                log::warn!("Failed to parse Ollama response as JSON: {}", parse_error);

                // Try to find and extract valid JSON from the response
                if let Some(start_idx) = cleaned_content.find('{') {
                    // Find the matching closing brace by counting braces
                    let mut brace_count = 0;
                    let mut end_idx = None;

                    for (i, c) in cleaned_content[start_idx..].char_indices() {
                        if c == '{' {
                            brace_count += 1;
                        } else if c == '}' {
                            brace_count -= 1;
                            if brace_count == 0 {
                                end_idx = Some(start_idx + i + 1); // +1 to include the closing brace
                                break;
                            }
                        }
                    }

                    if let Some(end_idx) = end_idx {
                        let json_str = &cleaned_content[start_idx..end_idx];
                        log::info!(
                            "Attempting to parse extracted JSON from Ollama: {}",
                            json_str
                        );

                        match serde_json::from_str::<Value>(json_str) {
                            Ok(json) => {
                                log::info!("Successfully parsed extracted JSON from Ollama");
                                json
                            }
                            Err(e) => {
                                log::warn!("Failed to parse extracted JSON from Ollama: {}", e);
                                json!({
                                    "detected_language": "unknown",
                                    "translated_text": cleaned_content
                                })
                            }
                        }
                    } else {
                        log::warn!("Could not find matching closing brace in Ollama response");
                        json!({
                            "detected_language": "unknown",
                            "translated_text": cleaned_content
                        })
                    }
                } else {
                    log::warn!("No JSON structure found in Ollama response");
                    json!({
                        "detected_language": "unknown",
                        "translated_text": cleaned_content
                    })
                }
            }
        };

        // Extract detected language and translated text from parsed JSON
        let detected_language = match parsed["detected_language"].as_str() {
            Some(lang) if !lang.is_empty() => lang.to_string(),
            _ => {
                // If detected_language field is missing or empty, try to look deeper in JSON structure
                if let Some(lang) = parsed.get("detected_language").and_then(|v| v.as_str()) {
                    lang.to_string()
                } else {
                    "unknown".to_string()
                }
            }
        };
        let translated_text = match parsed["translated_text"].as_str() {
            Some(text) => {
                // Unescape any escaped newlines that Ollama might have returned
                text.replace("\\n", "\n")
                    .replace("/n", "\n") // Also handle the literal /n issue
                    .replace("\\r\\n", "\n")
                    .replace("\\r", "\n")
            }
            None => {
                // If translated_text field is missing, check if the whole response is just text
                if parsed.is_string() {
                    let text = parsed.as_str().unwrap_or("translation failed");
                    text.replace("\\n", "\n")
                        .replace("/n", "\n") // Also handle the literal /n issue
                        .replace("\\r\\n", "\n")
                        .replace("\\r", "\n")
                } else {
                    "translation failed".to_string()
                }
            }
        };
        log::info!(
            "Detected language (provider reported): {}",
            detected_language
        );
        log::info!(
            "Configured (effective) target language: {}",
            self.config.target_language
        );

        log::info!(
            "Translated text (first 100 chars): {}",
            if translated_text.len() > 100 {
                format!("{}...", &translated_text[..100])
            } else {
                translated_text.clone()
            }
        );

        Ok(TranslationResult {
            detected_language,
            translated_text,
            target_language: self.config.target_language.clone(),
        })
    }
}

#[async_trait]
impl TranslationProvider for OllamaTranslationService {
    async fn translate(&self, text: &str) -> Result<TranslationResult> {
        let cleaned_text = clean_text_for_translation(text);
        log::info!("Cleaned text for Ollama translation: {}", cleaned_text);

        if self.config.model.trim().is_empty() {
            log::error!("OllamaTranslationService: model is empty; cannot proceed");
            return Err(anyhow::anyhow!("Model not configured for Ollama provider"));
        }

        // Check if this is an alternatives request (custom_prompt contains "alternatives")
        let is_alternatives_request = self.config.custom_prompt.contains("alternatives");

        let full_prompt = if is_alternatives_request {
            // For alternatives requests, use the custom prompt directly
            log::info!("Using alternatives prompt for Ollama");
            format!("{}\n\n\"{}\"", self.config.custom_prompt, cleaned_text)
        } else {
            // For regular translations, use the normal logic
            let user_prompt = format!(
                "Text to translate into {}: \"{}\"",
                self.config.target_language, cleaned_text
            );
            let smart_prompt = create_smart_prompt(&self.config, None);
            format!(
                "{}\n\nAlways respond with valid JSON containing 'detected_language' and 'translated_text' fields. Preserve line breaks and formatting in the translated text.\n\n{}",
                smart_prompt, user_prompt
            )
        };

        log::info!("Using Ollama model: {}", self.config.model);
        log::info!("Full prompt for Ollama: {}", full_prompt);

        // NOTE: GenerationOptions API removed/changed in ollama-rs 0.3.x; using basic request.
        // If advanced tuning is needed, update to the new ModelOptions once identified.
        let request = GenerationRequest::new(self.config.model.clone(), full_prompt);

        let response: GenerationResponse = self
            .client
            .generate(request)
            .await
            .map_err(|e| anyhow::anyhow!("Ollama generation failed: {}", e))?;

        let content = response.response;
        if content.is_empty() {
            return Err(anyhow::anyhow!("Empty response from Ollama"));
        }

        // Check if this is an alternatives request
        let is_alternatives_request = self.config.custom_prompt.contains("alternatives");
        if is_alternatives_request {
            // For alternatives requests, return the raw content as translated_text
            // The calling code will parse the JSON alternatives from it
            log::info!("Returning raw alternatives response from Ollama");
            Ok(TranslationResult {
                detected_language: "unknown".to_string(),
                translated_text: content,
                target_language: "alternatives".to_string(),
            })
        } else {
            self.parse_response_content(&content)
        }
    }
}
