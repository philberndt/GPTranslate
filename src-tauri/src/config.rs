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
    pub api_provider: String, // "openai", "azure_openai", or "ollama"
    pub openai_api_key: String,
    pub azure_endpoint: String,
    pub azure_api_key: String,
    pub azure_api_version: String,
    pub azure_deployment_name: String,
    pub ollama_url: Option<String>,
    pub model: String, // Current selected model
    pub available_models: HashMap<String, Vec<ModelConfig>>, // Provider -> Models
    pub target_language: String, // User-specified target language (e.g., "Spanish", "French", "German")
    pub alternative_target_language: String, // Used when detected language is same as target language
    pub auto_start: bool,
    pub hotkey: String,
    pub theme: String,
    pub minimize_to_tray: bool,
    pub custom_prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut available_models = HashMap::new();

        // Start with empty model arrays - users should configure their own models
        available_models.insert("openai".to_string(), vec![]);
        available_models.insert("azure_openai".to_string(), vec![]);
        available_models.insert("ollama".to_string(), vec![]);

        Self {
            api_provider: "".to_string(),
            openai_api_key: "".to_string(),
            azure_endpoint: "".to_string(),
            azure_api_key: "".to_string(),
            azure_api_version: "".to_string(),
            azure_deployment_name: "".to_string(),
            ollama_url: Some("http://localhost:11434".to_string()),
            model: "".to_string(),
            available_models,
            target_language: "English".to_string(),
            alternative_target_language: "Norwegian".to_string(),
            auto_start: true,
            hotkey: "Ctrl+Alt+C".to_string(),
            theme: "auto".to_string(),
            minimize_to_tray: true,
            custom_prompt: "Translate the given text from {detected_language} to {target_language} accurately while preserving the meaning, tone, and nuance of the original content.\n\n# Additional Details\n- Ensure the translation retains the context, cultural meaning, tone, formal/informal style, and any idiomatic expressions.\n- Do **not** alter names, technical terms, or specific formatting unless required for grammatical correctness in the target language.\n- If the detected language is the same as the target language, choose the most appropriate alternative language for translation.\n\n# Output Format\nThe translation output should be provided as valid JSON containing 'detected_language' and 'translated_text' fields.\n\n# Notes\n- Ensure punctuation and capitalization match the norms of the target language.\n- When encountering idiomatic expressions, adapt them to equivalent phrases in the target language rather than direct word-for-word translation.\n- For ambiguous content, aim for the most contextually appropriate meaning.\n- Take into consideration the whole text and what it is about.".to_string(),
        }
    }
}

impl Config {
    pub fn get_config_dir() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let config_dir = home_dir.join(".gptranslate");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        Ok(config_dir)
    }

    pub fn get_config_path() -> Result<PathBuf> {
        Ok(Self::get_config_dir()?.join("config.json"))
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
                            available_models.insert("ollama".to_string(), serde_json::json!([]));
                            value["available_models"] = serde_json::Value::Object(available_models);
                        } // Set empty model if it was using old defaults
                        if let Some(model) = value.get("model") {
                            if model.as_str() == Some("gpt-4o-mini") {
                                value["model"] = serde_json::Value::String("".to_string());
                            }
                        }

                        // Remove old source_language field if it exists
                        if value.get("source_language").is_some() {
                            value.as_object_mut().unwrap().remove("source_language");
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
