use crate::config::Config;
use crate::translation::{
    TranslationProvider, TranslationResult, clean_text_for_translation, create_smart_prompt,
};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

pub struct AzureOpenAITranslationService {
    client: reqwest::Client,
    config: Config,
}

impl AzureOpenAITranslationService {
    pub fn new(config: Config) -> Self {
        log::info!(
            "Creating AzureOpenAITranslationService with endpoint: {}",
            config.azure_endpoint
        );
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    async fn call_azure_openai(&self, request_body: Value) -> Result<Value> {
        // Determine endpoint type based on hostname
        let is_models_endpoint = self.config.azure_endpoint.contains("services.ai.azure.com");

        let url = if is_models_endpoint {
            // Models API endpoint format
            format!(
                "{}/models/chat/completions?api-version={}",
                self.config.azure_endpoint.trim_end_matches('/'),
                self.config.azure_api_version
            )
        } else {
            // Cognitive Services endpoint format
            format!(
                "{}/openai/deployments/{}/chat/completions?api-version={}",
                self.config.azure_endpoint.trim_end_matches('/'),
                self.config.azure_deployment_name,
                self.config.azure_api_version
            )
        };

        log::info!("Making Azure OpenAI request to: {}", url);
        log::info!(
            "Endpoint type: {}",
            if is_models_endpoint {
                "Models API"
            } else {
                "Cognitive Services"
            }
        );
        log::info!(
            "Request body: {}",
            serde_json::to_string_pretty(&request_body).unwrap_or_default()
        );

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.config.azure_api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            log::error!("Azure OpenAI API request failed: {}", error_text);
            return Err(anyhow::anyhow!(
                "Azure OpenAI API request failed: {}",
                error_text
            ));
        }

        Ok(response.json().await?)
    }

    fn parse_response_content(&self, content: &str) -> Result<TranslationResult> {
        log::info!("API Response content: {}", content);

        // Clean the content by removing control characters that can break JSON parsing
        let cleaned_content = content
            .chars()
            .filter(|c| !c.is_control() || matches!(*c, '\n' | '\r' | '\t'))
            .collect::<String>();

        // Log if we removed any control characters
        if cleaned_content != content {
            log::warn!("Removed control characters from API response");
            log::info!("Cleaned content: {}", cleaned_content);
        }

        // Try to parse as JSON, but handle cases where the AI might have returned plain text
        let parsed: Value = match serde_json::from_str(&cleaned_content) {
            Ok(json) => {
                log::info!("Successfully parsed JSON response");
                json
            }
            Err(parse_error) => {
                log::warn!("Failed to parse as JSON: {}", parse_error);

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
                        log::info!("Attempting to parse extracted JSON: {}", json_str);

                        match serde_json::from_str::<Value>(json_str) {
                            Ok(json) => {
                                log::info!("Successfully parsed extracted JSON");
                                json
                            }
                            Err(e) => {
                                log::warn!("Failed to parse extracted JSON: {}", e);
                                json!({
                                    "detected_language": "unknown",
                                    "translated_text": cleaned_content
                                })
                            }
                        }
                    } else {
                        log::warn!("Could not find matching closing brace");
                        json!({
                            "detected_language": "unknown",
                            "translated_text": cleaned_content
                        })
                    }
                } else {
                    log::warn!("No JSON structure found in response");
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
                // Properly handle escaped newlines that Azure might return
                text.replace("\\n", "\n")
                    .replace("\\r\\n", "\n")
                    .replace("\\r", "\n")
                    .replace("\\t", "\t")
            }
            None => {
                // If translated_text field is missing, check if the whole response is just text
                if parsed.is_string() {
                    let text = parsed.as_str().unwrap_or("translation failed");
                    text.replace("\\n", "\n")
                        .replace("\\r\\n", "\n")
                        .replace("\\r", "\n")
                        .replace("\\t", "\t")
                } else {
                    "translation failed".to_string()
                }
            }
        };

        log::info!("Detected language: {}", detected_language);
        log::info!("Target language: {}", self.config.target_language);
        log::info!(
            "Alternative target language: {}",
            self.config.alternative_target_language
        ); // Log if alternative language logic should have been applied
        let detected_lower = detected_language.to_lowercase();
        let target_lower = self.config.target_language.to_lowercase();

        log::info!(
            "Azure - Language analysis: detected='{}', target='{}', alternative='{}'",
            detected_language,
            self.config.target_language,
            self.config.alternative_target_language
        );

        if detected_lower == target_lower {
            log::info!(
                "Azure - Alternative language logic SHOULD apply: detected '{}' matches target '{}', should translate to '{}'",
                detected_language,
                self.config.target_language,
                self.config.alternative_target_language
            );
        } else {
            log::info!(
                "Azure - Primary target logic SHOULD apply: detected '{}' != target '{}', should translate to '{}'",
                detected_language,
                self.config.target_language,
                self.config.target_language
            );
        }
        log::info!(
            "Translated text (first 100 chars): {}",
            if translated_text.len() > 100 {
                format!("{}...", &translated_text[..100])
            } else {
                translated_text.clone()
            }
        );

        // Determine the actual target language used for translation
        let actual_target_language = if detected_lower == target_lower {
            // Alternative language logic was applied
            self.config.alternative_target_language.clone()
        } else {
            // Primary target language was used
            self.config.target_language.clone()
        };

        log::info!("Returning target_language: {}", actual_target_language);

        Ok(TranslationResult {
            detected_language,
            translated_text,
            target_language: actual_target_language,
        })
    }
}

#[async_trait]
impl TranslationProvider for AzureOpenAITranslationService {
    async fn translate(&self, text: &str) -> Result<TranslationResult> {
        let cleaned_text = clean_text_for_translation(text);
        log::info!("Cleaned text for translation: {}", cleaned_text);

        let user_prompt = format!("Text to translate: \"{}\"", cleaned_text);
        let smart_prompt = create_smart_prompt(&self.config);
        log::info!("Using smart prompt with alternative language logic");

        // Determine which token parameter to use based on model
        let model_name = if !self.config.azure_deployment_name.is_empty() {
            &self.config.azure_deployment_name
        } else {
            &self.config.model
        };

        // Check if this is a reasoning model (o1, o3 series)
        let is_reasoning_model = model_name.starts_with("o1") || model_name.starts_with("o3");

        log::info!(
            "Model name detected: {}, is_reasoning_model: {}",
            model_name,
            is_reasoning_model
        );

        let system_role = if is_reasoning_model {
            "developer"
        } else {
            "system"
        };
        let mut request_body = json!({
            "messages": [
                {
                    "role": system_role,
                    "content": [
                        {
                            "type": "text",
                            "text": format!("{}\n\nIMPORTANT FORMATTING RULES:\n- Always respond with valid JSON containing 'detected_language' and 'translated_text' fields\n- Preserve line breaks and paragraph structure in the translation\n- Use actual newline characters (\\n) in the JSON string value, not escaped \\\\n\n- Do NOT escape newlines in the JSON response - use literal newlines\n\nExample response format:\n{{\n  \"detected_language\": \"English\",\n  \"translated_text\": \"Line 1\\nLine 2\\n\\nNew paragraph\"\n}}", smart_prompt)
                        }
                    ]
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });

        // Add temperature only for non-reasoning models
        if !is_reasoning_model {
            request_body["temperature"] = json!(0.3);
        }

        // Use appropriate token parameter based on model type
        if is_reasoning_model {
            request_body["max_completion_tokens"] = json!(800);
        } else {
            request_body["max_tokens"] = json!(800);
        }

        // For Azure Models API endpoints, we need to include the model parameter
        // For Cognitive Services endpoints, the model is specified in the URL path
        let is_models_endpoint = self.config.azure_endpoint.contains("services.ai.azure.com");
        if is_models_endpoint && !self.config.azure_deployment_name.is_empty() {
            request_body["model"] = json!(self.config.azure_deployment_name);
            log::info!(
                "Using Azure Models API model: {}",
                self.config.azure_deployment_name
            );
        }

        let response = self.call_azure_openai(request_body).await?;
        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No content in response"))?;

        self.parse_response_content(content)
    }
}
