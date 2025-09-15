use crate::config::Config;
use crate::translation::{TranslationProvider, TranslationResult, clean_text_for_translation};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

pub struct AzureTranslatorService {
    client: reqwest::Client,
    config: Config,
}

impl AzureTranslatorService {
    pub fn new(config: Config) -> Self {
        log::info!(
            "Creating AzureTranslatorService with endpoint: {}",
            config.azure_translator_endpoint
        );
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    async fn call_azure_translator(
        &self,
        text: &str,
        to_language: &str,
        from_language: Option<&str>,
    ) -> Result<Value> {
        let url = format!(
            "{}/translate?api-version=3.0&to={}{}",
            self.config.azure_translator_endpoint.trim_end_matches('/'),
            to_language,
            from_language
                .map(|lang| format!("&from={}", lang))
                .unwrap_or_default()
        );

        let request_body = json!([{
            "Text": text
        }]);

        log::info!("Making Azure Translator request to: {}", url);
        log::info!(
            "Request body: {}",
            serde_json::to_string_pretty(&request_body).unwrap_or_default()
        );

        let mut request = self
            .client
            .post(&url)
            .header(
                "Ocp-Apim-Subscription-Key",
                &self.config.azure_translator_api_key,
            )
            .header("Content-Type", "application/json; charset=UTF-8");

        // Add region header if specified (required for multi-service or regional resources)
        if !self.config.azure_translator_region.is_empty() {
            request = request.header(
                "Ocp-Apim-Subscription-Region",
                &self.config.azure_translator_region,
            );
        }

        let response = request.json(&request_body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            log::error!(
                "Azure Translator API request failed: Status: {}, Error: {}",
                status,
                error_text
            );
            return Err(anyhow::anyhow!(
                "Azure Translator API request failed ({}): {}",
                status,
                error_text
            ));
        }

        Ok(response.json().await?)
    }

    fn parse_translator_response(&self, response: Value) -> Result<TranslationResult> {
        log::info!(
            "Azure Translator Response: {}",
            serde_json::to_string_pretty(&response).unwrap_or_default()
        );

        // Azure Translator returns an array with translation results
        let results = response
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format: expected array"))?;

        if results.is_empty() {
            return Err(anyhow::anyhow!("Empty response from Azure Translator"));
        }

        let result = &results[0];

        // Extract detected language information
        let detected_language = if let Some(detected) = result.get("detectedLanguage") {
            detected
                .get("language")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string()
        } else {
            "unknown".to_string()
        };

        // Extract translation text
        let translations = result
            .get("translations")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("No translations found in response"))?;

        if translations.is_empty() {
            return Err(anyhow::anyhow!("Empty translations array"));
        }

        let translation = &translations[0];
        let translated_text = translation
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("No translation text found"))?
            .to_string();

        let target_lang = translation
            .get("to")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        log::info!("Detected language: {}", detected_language);
        log::info!("Target language: {}", target_lang);
        log::info!(
            "Alternative target language: {}",
            self.config.alternative_target_language
        );

        // The actual target language is determined by what we requested from the API
        // and should match the target_lang returned in the response
        let actual_target_language = self.map_language_code_to_name(&target_lang);

        log::info!(
            "Translated text (first 100 chars): {}",
            if translated_text.len() > 100 {
                format!("{}...", &translated_text[..100])
            } else {
                translated_text.clone()
            }
        );

        Ok(TranslationResult {
            detected_language: self.map_language_code_to_name(&detected_language),
            translated_text,
            target_language: actual_target_language,
        })
    }

    fn map_language_code_to_name(&self, code: &str) -> String {
        // Map common language codes to full names for consistency
        match code.to_lowercase().as_str() {
            "en" => "English".to_string(),
            "es" => "Spanish".to_string(),
            "fr" => "French".to_string(),
            "de" => "German".to_string(),
            "it" => "Italian".to_string(),
            "pt" => "Portuguese".to_string(),
            "ru" => "Russian".to_string(),
            "ja" => "Japanese".to_string(),
            "ko" => "Korean".to_string(),
            "zh" | "zh-hans" => "Chinese".to_string(),
            "zh-hant" => "Chinese (Traditional)".to_string(),
            "ar" => "Arabic".to_string(),
            "hi" => "Hindi".to_string(),
            "nl" => "Dutch".to_string(),
            "sv" => "Swedish".to_string(),
            "no" | "nb" => "Norwegian".to_string(),
            "da" => "Danish".to_string(),
            "fi" => "Finnish".to_string(),
            "pl" => "Polish".to_string(),
            "tr" => "Turkish".to_string(),
            "cs" => "Czech".to_string(),
            "hu" => "Hungarian".to_string(),
            _ => code.to_string(), // Return the code as-is if not mapped
        }
    }

    fn map_language_name_to_code(&self, name: &str) -> String {
        // Map language names to Azure Translator language codes
        match name.to_lowercase().as_str() {
            "english" => "en".to_string(),
            "spanish" => "es".to_string(),
            "french" => "fr".to_string(),
            "german" => "de".to_string(),
            "italian" => "it".to_string(),
            "portuguese" => "pt".to_string(),
            "russian" => "ru".to_string(),
            "japanese" => "ja".to_string(),
            "korean" => "ko".to_string(),
            "chinese" | "chinese (simplified)" => "zh-Hans".to_string(),
            "chinese (traditional)" => "zh-Hant".to_string(),
            "arabic" => "ar".to_string(),
            "hindi" => "hi".to_string(),
            "dutch" => "nl".to_string(),
            "swedish" => "sv".to_string(),
            "norwegian" => "nb".to_string(),
            "danish" => "da".to_string(),
            "finnish" => "fi".to_string(),
            "polish" => "pl".to_string(),
            "turkish" => "tr".to_string(),
            "czech" => "cs".to_string(),
            "hungarian" => "hu".to_string(),
            _ => name.to_string(), // Return the name as-is if not mapped
        }
    }
}

#[async_trait]
impl TranslationProvider for AzureTranslatorService {
    async fn translate(&self, text: &str) -> Result<TranslationResult> {
        let cleaned_text = clean_text_for_translation(text);
        log::info!("Cleaned text for translation: {}", cleaned_text);

        // Check if user specified a source language override
        let source_language_code = self
            .config
            .user_source_language
            .as_ref()
            .map(|lang| self.map_language_name_to_code(lang));

        // First, detect the language if not specified by user
        let detected_language = if source_language_code.is_some() {
            source_language_code.clone().unwrap()
        } else {
            // Make a detection-only request to Azure Translator
            let detect_response = self
                .call_azure_translator(&cleaned_text, "en", None) // Use 'en' as dummy target for detection
                .await?;
            
            let results = detect_response
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Invalid response format for detection"))?;
            
            if results.is_empty() {
                return Err(anyhow::anyhow!("Empty response from Azure Translator for detection"));
            }
            
            let result = &results[0];
            if let Some(detected) = result.get("detectedLanguage") {
                detected
                    .get("language")
                    .and_then(|v| v.as_str())
                    .unwrap_or("en")
                    .to_string()
            } else {
                "en".to_string()
            }
        };

        log::info!("Detected language code: {}", detected_language);

        // Determine the actual target language based on smart switching logic
        let target_language_code = {
            let detected_lower = detected_language.to_lowercase();
            let primary_target_code = self.map_language_name_to_code(&self.config.target_language);
            let primary_target_lower = primary_target_code.to_lowercase();

            if detected_lower == primary_target_lower {
                log::info!(
                    "Smart switching: detected '{}' matches primary target '{}', using alternative target '{}'",
                    detected_language,
                    primary_target_code,
                    self.config.alternative_target_language
                );
                self.map_language_name_to_code(&self.config.alternative_target_language)
            } else {
                log::info!(
                    "Normal translation: detected '{}' != primary target '{}', using primary target",
                    detected_language,
                    primary_target_code
                );
                primary_target_code
            }
        };

        log::info!("Final target language code: {}", target_language_code);
        if let Some(ref source_code) = source_language_code {
            log::info!("Source language code (user specified): {}", source_code);
        } else {
            log::info!("Source language: auto-detect");
        }

        let response = self
            .call_azure_translator(
                &cleaned_text,
                &target_language_code,
                source_language_code.as_deref(),
            )
            .await?;

        self.parse_translator_response(response)
    }
}
