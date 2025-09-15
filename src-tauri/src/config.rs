use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModelConfig {
    pub name: String,
    pub display_name: String,
    pub provider: String,
    pub is_enabled: bool,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub api_provider: String, // "openai", "azure_openai", "azure_translator", or "ollama"
    pub openai_api_key: String,
    pub azure_endpoint: String,
    pub azure_api_key: String,
    pub azure_api_version: String,
    pub azure_deployment_name: String,
    pub azure_translator_endpoint: String,
    pub azure_translator_api_key: String,
    pub azure_translator_region: String,
    pub ollama_url: Option<String>,
    pub model: String, // Current selected model
    pub available_models: HashMap<String, Vec<ModelConfig>>, // Provider -> Models
    pub target_language: String, // User-specified target language (e.g., "Spanish", "French", "German")
    pub alternative_target_language: String, // Used when detected language is same as target language
    pub favorite_languages: Vec<String>,     // User's favorite language codes for quick access
    pub user_source_language: Option<String>, // Manual source language override (None = auto-detect)
    pub auto_start: bool,
    pub hotkey: String,
    pub theme: String,
    pub minimize_to_tray: bool,
    pub custom_prompt: String,
    pub reasoning_effort: Option<String>, // "minimal", "low", "medium", "high"
    pub alternatives_fallback_provider: Option<String>,
    pub auto_translate_enabled: bool,
    pub auto_translate_debounce_ms: u32,
    pub auto_translate_on_paste: bool,
    pub auto_translate_while_typing: bool,
}

impl Default for Config {
    fn default() -> Self {
        let mut available_models = HashMap::new();

        // Start with empty model arrays - users should configure their own models
        available_models.insert("openai".to_string(), vec![]);
        available_models.insert("azure_openai".to_string(), vec![]);
        available_models.insert("azure_translator".to_string(), vec![]);
        available_models.insert("ollama".to_string(), vec![]);

        Self {
            api_provider: "".to_string(),
            openai_api_key: "".to_string(),
            azure_endpoint: "".to_string(),
            azure_api_key: "".to_string(),
            azure_api_version: "".to_string(),
            azure_deployment_name: "".to_string(),
            azure_translator_endpoint: "https://api.cognitive.microsofttranslator.com".to_string(),
            azure_translator_api_key: "".to_string(),
            azure_translator_region: "".to_string(),
            ollama_url: Some("".to_string()),
            model: "".to_string(),
            available_models,
            target_language: "English".to_string(),
            alternative_target_language: "Spanish".to_string(),
            favorite_languages: vec!["en".to_string(), "es".to_string(), "fr".to_string(), "de".to_string()],
            user_source_language: None,
            auto_start: false,
            hotkey: "Ctrl+Q".to_string(),
            theme: "auto".to_string(),
            minimize_to_tray: true,
            custom_prompt: "Translate the given text from {detected_language} to {target_language} accurately while preserving the meaning, tone, and nuance of the original content.\n\n# Additional Details\n- Ensure the translation retains the context, cultural meaning, tone, formal/informal style, and any idiomatic expressions.\n- Do **not** alter names, technical terms, or specific formatting unless required for grammatical correctness in the target language.\n- If the detected language is the same as the target language, choose the most appropriate alternative language for translation.\n\n# Output Format\nThe translation output should be provided as valid JSON containing 'detected_language' and 'translated_text' fields.\n\n# Notes\n- Ensure punctuation and capitalization match the norms of the target language.\n- When encountering idiomatic expressions, adapt them to equivalent phrases in the target language rather than direct word-for-word translation.\n- For ambiguous content, aim for the most contextually appropriate meaning.\n- Take into consideration the whole text and what it is about.".to_string(),
            reasoning_effort: Some("medium".to_string()),
            alternatives_fallback_provider: None,
            auto_translate_enabled: true,
            auto_translate_debounce_ms: 500,
            auto_translate_on_paste: true,
            auto_translate_while_typing: true,
        }
    }
}

impl Config {
    pub fn parse_alternatives_fallback(&self) -> Option<(String, String)> {
        let raw = self.alternatives_fallback_provider.as_deref()?.trim();
        if raw.is_empty() {
            return None;
        }

        let (provider, mut model) = match raw.split_once(':') {
            Some((p, m)) => (p.trim().to_lowercase(), m.trim().to_string()),
            None => (raw.to_lowercase(), String::new()),
        };

        // Validate provider
        match provider.as_str() {
            "openai" | "azure_openai" | "ollama" => {}
            other => {
                log::warn!("Unknown alternatives fallback provider '{}'", other);
                return None;
            }
        }

        // If model not specified, try to resolve a sensible default
        if model.is_empty() {
            // Prefer a currently configured, enabled model for that provider
            if let Some(models) = self.available_models.get(&provider) {
                if let Some(first_enabled) = models.iter().find(|m| m.is_enabled) {
                    model = first_enabled.name.clone();
                } else if let Some(first_any) = models.first() {
                    model = first_any.name.clone();
                }
            }

            // Azure OpenAI: if still empty, use deployment name
            if model.is_empty()
                && provider == "azure_openai"
                && !self.azure_deployment_name.is_empty()
            {
                model = self.azure_deployment_name.clone();
            }

            // If still empty and the current global model matches the provider, use it
            if model.is_empty() && self.api_provider == provider && !self.model.is_empty() {
                model = self.model.clone();
            }
        } else {
            // Model was provided; if it's an Azure OpenAI provider and model looks empty,
            // fall back to deployment name
            if provider == "azure_openai"
                && model.is_empty()
                && !self.azure_deployment_name.is_empty()
            {
                model = self.azure_deployment_name.clone();
            }
        }

        if model.is_empty() {
            log::warn!(
                "Could not resolve a model for alternatives fallback provider '{}'. Check Settings → Model Management.",
                provider
            );
            return None;
        }

        Some((provider, model))
    }

    /// Ensures that for Azure OpenAI, the deployment name matches the model name
    pub fn ensure_azure_deployment_consistency(&mut self) {
        if self.api_provider == "azure_openai"
            && !self.model.is_empty()
            && self.azure_deployment_name != self.model
        {
            log::info!(
                "Updating Azure deployment name from '{}' to '{}' to match model",
                self.azure_deployment_name,
                self.model
            );
            self.azure_deployment_name = self.model.clone();
        }
    }

    pub fn get_config_dir() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let config_dir = home_dir.join(".config").join("gptranslate");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        Ok(config_dir)
    }

    pub fn get_config_path() -> Result<PathBuf> {
        Ok(Self::get_config_dir()?.join("gptranslate.json"))
    }
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;

            // Try to parse the config, and if it fails due to missing fields, migrate it
            match serde_json::from_str::<Config>(&content) {
                Ok(config) => Ok(config),
                Err(_) => {
                    // Try to parse as a generic Value first to preserve existing settings
                    if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&content) {
                        // Add missing fields with defaults
                        if value.get("custom_prompt").is_none() {
                            value["custom_prompt"] = serde_json::Value::String(
                                "Translate the given text from {detected_language} to {target_language} accurately while preserving the meaning, tone, and nuance of the original content.\n\n# Additional Details\n- Ensure the translation retains the context, cultural meaning, tone, formal/informal style, and any idiomatic expressions.\n- Do **not** alter names, technical terms, or specific formatting unless required for grammatical correctness in the target language.\n- If the detected language is the same as the target language, choose the most appropriate alternative language for translation.\n\n# Output Format\nThe translation output should be provided as valid JSON containing 'detected_language' and 'translated_text' fields.\n\n# Notes\n- Ensure punctuation and capitalization match the norms of the target language.\n- When encountering idiomatic expressions, adapt them to equivalent phrases in the target language rather than direct word-for-word translation.\n- For ambiguous content, aim for the most contextually appropriate meaning.\n- Take into consideration the whole text and what it is about.".to_string()
                            );
                        } // Add alternative_target_language if missing
                        if value.get("alternative_target_language").is_none() {
                            value["alternative_target_language"] =
                                serde_json::Value::String("Norwegian".to_string());
                        }

                        // Add favorite_languages if missing
                        if value.get("favorite_languages").is_none() {
                            value["favorite_languages"] =
                                serde_json::json!(["en", "es", "fr", "de"]);
                        }

                        // Add user_source_language if missing
                        if value.get("user_source_language").is_none() {
                            value["user_source_language"] = serde_json::Value::Null;
                        }

                        // Add ollama_url if missing
                        if value.get("ollama_url").is_none() {
                            value["ollama_url"] =
                                serde_json::Value::String("http://localhost:11434".to_string());
                        } // Add available_models if missing with empty arrays
                        if value.get("available_models").is_none() {
                            let mut available_models = serde_json::Map::new();
                            available_models.insert("openai".to_string(), serde_json::json!([]));
                            available_models
                                .insert("azure_openai".to_string(), serde_json::json!([]));
                            available_models
                                .insert("azure_translator".to_string(), serde_json::json!([]));
                            available_models.insert("ollama".to_string(), serde_json::json!([]));
                            value["available_models"] = serde_json::Value::Object(available_models);
                        }

                        // Add azure_translator fields if missing
                        if value.get("azure_translator_endpoint").is_none() {
                            value["azure_translator_endpoint"] = serde_json::Value::String(
                                "https://api.cognitive.microsofttranslator.com".to_string(),
                            );
                        }
                        if value.get("azure_translator_api_key").is_none() {
                            value["azure_translator_api_key"] =
                                serde_json::Value::String("".to_string());
                        }
                        if value.get("azure_translator_region").is_none() {
                            value["azure_translator_region"] =
                                serde_json::Value::String("".to_string());
                        } // Set empty model if it was using old defaults
                        if let Some(model) = value.get("model")
                            && model.as_str() == Some("gpt-4o-mini")
                        {
                            value["model"] = serde_json::Value::String("".to_string());
                        }

                        // Remove old source_language field if it exists
                        if value.get("source_language").is_some() {
                            value.as_object_mut().unwrap().remove("source_language");
                        }

                        // Add reasoning_effort if missing
                        if value.get("reasoning_effort").is_none() {
                            value["reasoning_effort"] =
                                serde_json::Value::String("medium".to_string());
                        }

                        // Add alternatives_fallback_provider if missing
                        if value.get("alternatives_fallback_provider").is_none() {
                            value["alternatives_fallback_provider"] = serde_json::Value::Null;
                        }

                        // Add automatic translation fields if missing
                        if value.get("auto_translate_enabled").is_none() {
                            value["auto_translate_enabled"] = serde_json::Value::Bool(true);
                        }
                        if value.get("auto_translate_debounce_ms").is_none() {
                            value["auto_translate_debounce_ms"] =
                                serde_json::Value::Number(serde_json::Number::from(500));
                        }
                        if value.get("auto_translate_on_paste").is_none() {
                            value["auto_translate_on_paste"] = serde_json::Value::Bool(true);
                        }
                        if value.get("auto_translate_while_typing").is_none() {
                            value["auto_translate_while_typing"] = serde_json::Value::Bool(true);
                        }

                        // Ensure target_language has a sensible default if it was "auto"
                        if let Some(target_lang) = value.get("target_language") {
                            if target_lang.as_str() == Some("auto") {
                                value["target_language"] =
                                    serde_json::Value::String("English".to_string());
                            }
                        } else {
                            value["target_language"] =
                                serde_json::Value::String("English".to_string());
                        }

                        // Try to parse again with the migrated config
                        let migrated_config: Config = serde_json::from_value(value)?;
                        migrated_config.save()?; // Save the migrated config
                        Ok(migrated_config)
                    } else {
                        // If all else fails, use default config
                        let default_config = Self::default();
                        default_config.save()?;
                        Ok(default_config)
                    }
                }
            }
        } else {
            let default_config = Self::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}
