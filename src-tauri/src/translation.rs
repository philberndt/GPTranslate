use crate::config::Config;
use crate::trans_openai::OpenAITranslationService;
use crate::trans_azure::AzureOpenAITranslationService;
use crate::trans_ollama::OllamaTranslationService;
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
        let provider: Box<dyn TranslationProvider + Send + Sync> = match config.api_provider.as_str() {
            "openai" => Box::new(OpenAITranslationService::new(config)),
            "azure_openai" => Box::new(AzureOpenAITranslationService::new(config)),
            "ollama" => Box::new(OllamaTranslationService::new(config)),
            _ => {
                log::warn!("Unknown API provider '{}', defaulting to OpenAI", config.api_provider);
                Box::new(OpenAITranslationService::new(config))
            }
        };

        Self { provider }
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
    text
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn create_smart_prompt(config: &Config) -> String {
    format!(
        "{}\n\n# Alternative Language Logic\n- Primary target language: {}\n- Alternative target language: {}\n- If the detected language matches the primary target language, translate to the alternative target language instead.\n- If the detected language is different from the primary target language, translate to the primary target language.",
        config.custom_prompt,
        config.target_language,
        config.alternative_target_language
    )
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
        "Config loaded, custom_prompt: {}",
        config_clone.custom_prompt
    );

    let service = TranslationService::new(config_clone.clone());
    match service.detect_and_translate(&text).await {
        Ok(result) => {
            // Use the configured target language from config
            let target_language = config_clone.target_language.clone();

            Ok(TranslationResponse {
                original_text: text,
                translated_text: result.translated_text,
                detected_language: result.detected_language,
                target_language,
            })
        }
        Err(e) => {
            if e.to_string().contains("Duplicate request detected") {
                Err(Error::DuplicateRequest)
            } else {
                Err(Error::ApiError(e))
            }
        }
    }
}
