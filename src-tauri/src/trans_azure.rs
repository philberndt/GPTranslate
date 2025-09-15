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

    fn is_reasoning_model(&self) -> bool {
        // Automatic detection based on known reasoning model name patterns
        // Supports custom Azure deployment names that include underlying model name
        let deployment_name = if !self.config.azure_deployment_name.is_empty() {
            self.config.azure_deployment_name.as_str()
        } else {
            ""
        };
        let model_name = self.config.model.as_str();

        // Check both deployment name and model name
        let is_reasoning =
            is_reasoning_model_name(deployment_name) || is_reasoning_model_name(model_name);

        log::info!(
            "Azure - Reasoning model detection: deployment='{}', model='{}', is_reasoning={}",
            deployment_name,
            model_name,
            is_reasoning
        );

        is_reasoning
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
            let status = response.status();
            let error_text = response.text().await?;
            log::error!(
                "Azure OpenAI API request failed: Status: {}, Error: {}",
                status,
                error_text
            );
            return Err(anyhow::anyhow!(
                "Azure OpenAI API request failed ({}): {}",
                status,
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

        // IMPORTANT: For reasoning models, the content is often just the JSON text itself
        // First try direct JSON parsing
        let parsed: Value = match serde_json::from_str(&cleaned_content) {
            Ok(json) => {
                log::info!("Successfully parsed JSON response directly");
                json
            }
            Err(parse_error) => {
                log::warn!("Failed to parse as JSON directly: {}", parse_error);

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

        // Extract detected language from parsed JSON with better error handling
        let detected_language = if let Some(lang) = parsed["detected_language"].as_str() {
            if !lang.is_empty() && lang.to_lowercase() != "unknown" {
                log::info!(
                    "✅ Successfully extracted detected_language from JSON: {}",
                    lang
                );
                lang.to_string()
            } else {
                log::warn!(
                    "⚠️ detected_language field found but is empty or 'unknown': {}",
                    lang
                );
                "unknown".to_string()
            }
        } else {
            log::error!(
                "❌ No 'detected_language' field found in JSON. Available keys: {:?}",
                parsed.as_object().map(|obj| obj.keys().collect::<Vec<_>>())
            );
            "unknown".to_string()
        };
        let translated_text = if let Some(text) = parsed["translated_text"].as_str() {
            if !text.is_empty() {
                log::info!(
                    "✅ Successfully extracted translated_text from JSON (length: {})",
                    text.len()
                );
                // Properly handle escaped newlines that Azure might return
                text.replace("\\n", "\n")
                    .replace("\\r\\n", "\n")
                    .replace("\\r", "\n")
                    .replace("\\t", "\t")
            } else {
                log::error!("❌ translated_text field found but is empty");
                "translation failed".to_string()
            }
        } else {
            log::error!(
                "❌ No 'translated_text' field found in JSON. Available keys: {:?}",
                parsed.as_object().map(|obj| obj.keys().collect::<Vec<_>>())
            );
            // If translated_text field is missing, check if the whole response is just text
            if parsed.is_string() {
                let text = parsed.as_str().unwrap_or("translation failed");
                log::info!(
                    "📝 Using whole response as translated text (length: {})",
                    text.len()
                );
                text.replace("\\n", "\n")
                    .replace("\\r\\n", "\n")
                    .replace("\\r", "\n")
                    .replace("\\t", "\t")
            } else {
                "translation failed".to_string()
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

        log::info!("Detected language (provider reported): {}", detected_language);
        log::info!("Configured (effective) target language: {}", self.config.target_language);
        Ok(TranslationResult { detected_language, translated_text, target_language: self.config.target_language.clone() })
    }
}

#[async_trait]
impl TranslationProvider for AzureOpenAITranslationService {
    async fn translate(&self, text: &str) -> Result<TranslationResult> {
        let cleaned_text = clean_text_for_translation(text);
        log::info!("Cleaned text for translation: {}", cleaned_text);

        if self.config.model.trim().is_empty() && self.config.azure_deployment_name.trim().is_empty() {
            log::error!("AzureOpenAITranslationService: model/deployment name is empty; cannot proceed");
            return Err(anyhow::anyhow!("Model / deployment not configured for Azure OpenAI provider"));
        }

        // Check if this is an alternatives request (custom_prompt contains "alternatives")
        let is_alternatives_request = self.config.custom_prompt.contains("alternatives");

        let (user_prompt, system_prompt) = if is_alternatives_request {
            // For alternatives requests, use the custom prompt directly as the system prompt
            log::info!("Using alternatives prompt");
            (
                format!("\"{}\"", cleaned_text),
                self.config.custom_prompt.clone(),
            )
        } else {
            // For regular translations, use the normal logic
            let user_prompt = format!("Text to translate into {}: \"{}\"", self.config.target_language, cleaned_text);
            let smart_prompt = create_smart_prompt(&self.config, None);
            log::info!("Using smart prompt (pre-resolved target '{}')", self.config.target_language);
            (user_prompt, smart_prompt)
        };

        // Determine which token parameter to use based on model
        let model_name = if !self.config.azure_deployment_name.is_empty() {
            &self.config.azure_deployment_name
        } else {
            &self.config.model
        };

        // Check if this is a reasoning model
        let is_reasoning_model = self.is_reasoning_model();

        log::info!(
            "Azure - Model name detected: {}, is_reasoning_model: {}",
            model_name,
            is_reasoning_model
        );
        log::info!(
            "Azure - Deployment name: '{}', Model config: '{}'",
            self.config.azure_deployment_name,
            self.config.model
        );

        // Role selection based on reasoning model type:
        // - GPT-5 models: Use "system" role (they support system messages)
        // - O-series models (o1, o3, o4-mini): Use "developer" role
        // - Non-reasoning models: Use "system" role
        let system_role = if is_reasoning_model
            && (model_name.starts_with("o1")
                || model_name.starts_with("o3")
                || model_name.starts_with("o4-mini"))
        {
            "developer"
        } else {
            "system"
        };
        let system_content = if is_alternatives_request {
            // For alternatives requests, use the system prompt directly with minimal formatting
            if is_reasoning_model {
                format!("Please respond with valid JSON.\n\n{}", system_prompt)
            } else {
                system_prompt
            }
        } else if is_reasoning_model {
            // For reasoning models, use simplified instructions
            if model_name.starts_with("gpt-5") {
                // GPT-5 specific: even more simplified
                format!(
                    "{}\n\nPlease respond with valid JSON containing 'detected_language' and 'translated_text' fields.",
                    system_prompt
                )
            } else {
                // Other reasoning models
                format!(
                    "Please respond with valid JSON.\n\n{}\n\nRespond with JSON containing 'detected_language' and 'translated_text' fields.",
                    system_prompt
                )
            }
        } else {
            // For non-reasoning models, use detailed instructions
            format!(
                "{}\n\nIMPORTANT FORMATTING RULES:\n- Always respond with valid JSON containing 'detected_language' and 'translated_text' fields\n- Preserve line breaks and paragraph structure in the translation\n- Use actual newline characters (\\n) in the JSON string value, not escaped \\\\n\n- Do NOT escape newlines in the JSON response - use literal newlines\n\nExample response format:\n{{\n  \"detected_language\": \"English\",\n  \"translated_text\": \"Line 1\\nLine 2\\n\\nNew paragraph\"\n}}",
                system_prompt
            )
        };

        let mut request_body = json!({
            "messages": [
                {
                    "role": system_role,
                    "content": system_content
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });

        // Use appropriate token parameter based on model type
        if is_reasoning_model {
            // GPT-5 models support "minimal" reasoning effort which is optimal
            if model_name.starts_with("gpt-5") {
                request_body["max_completion_tokens"] = json!(1000);
                request_body["reasoning_effort"] = json!("minimal");
                log::info!(
                    "Azure - GPT-5 specific config: max_completion_tokens=1000, reasoning_effort=minimal"
                );
            } else if model_name.starts_with("gpt-4.1") {
                // GPT-4.1 should use low reasoning effort
                request_body["max_completion_tokens"] = json!(800);
                request_body["reasoning_effort"] = json!("low");
                log::info!(
                    "Azure - GPT-4.1 specific config: max_completion_tokens=800, reasoning_effort=low"
                );
            } else {
                // Other reasoning models (o1, o3, etc.) - use config or default to low
                request_body["max_completion_tokens"] = json!(800);
                let effort = self.config.reasoning_effort.as_deref().unwrap_or("low");
                request_body["reasoning_effort"] = json!(effort);
                log::info!("Azure - Using reasoning effort: {}", effort);
            }

            // Add JSON mode for reasoning models to ensure proper JSON response
            request_body["response_format"] = json!({
                "type": "json_object"
            });
            log::info!("Azure - Added JSON mode for reasoning model");

            // Note: Reasoning models don't support temperature, top_p, presence_penalty, frequency_penalty, logprobs, top_logprobs, logit_bias parameters
        } else {
            request_body["max_tokens"] = json!(800);
            request_body["temperature"] = json!(0.3);
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
        log::info!(
            "Azure API Response: {}",
            serde_json::to_string_pretty(&response).unwrap_or_default()
        );

        // Additional debugging for GPT-5 models
        if model_name.starts_with("gpt-5") {
            log::error!(
                "GPT-5 DEBUG - Full response structure: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            );
            if let Some(choices) = response["choices"].as_array() {
                log::error!("GPT-5 DEBUG - Choices length: {}", choices.len());
                if !choices.is_empty() {
                    log::error!(
                        "GPT-5 DEBUG - First choice: {}",
                        serde_json::to_string_pretty(&choices[0]).unwrap_or_default()
                    );
                }
            }
        }

        // Better error handling for response structure
        let choices = response["choices"].as_array().ok_or_else(|| {
            anyhow::anyhow!(
                "No 'choices' array in response. Full response: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            )
        })?;

        if choices.is_empty() {
            return Err(anyhow::anyhow!(
                "Empty 'choices' array in response. Full response: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            ));
        }

        let message = &choices[0]["message"];
        if message.is_null() {
            return Err(anyhow::anyhow!(
                "No 'message' object in first choice. Full response: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            ));
        }

        let content = message["content"].as_str().ok_or_else(|| {
            anyhow::anyhow!(
                "No 'content' field in message or content is not a string. Message: {}",
                serde_json::to_string_pretty(message).unwrap_or_default()
            )
        })?;

        log::info!("Azure API Response Content: {}", content);

        // Additional debugging for empty content
        if content.is_empty() {
            log::error!(
                "EMPTY CONTENT DEBUG - Model: {}, Content length: {}",
                model_name,
                content.len()
            );
            log::error!(
                "EMPTY CONTENT DEBUG - Full message object: {}",
                serde_json::to_string_pretty(message).unwrap_or_default()
            );
        }

        // Check if this is an alternatives request
        let is_alternatives_request = self.config.custom_prompt.contains("alternatives");
        if is_alternatives_request {
            // For alternatives requests, return the raw content as translated_text
            // The calling code will parse the JSON alternatives from it
            log::info!("Returning raw alternatives response");
            Ok(TranslationResult {
                detected_language: "unknown".to_string(),
                translated_text: content.to_string(),
                target_language: "alternatives".to_string(),
            })
        } else {
            self.parse_response_content(content)
        }
    }
}

fn is_reasoning_model_name(name: &str) -> bool {
    let n = name.to_lowercase();
    // Known Azure OpenAI reasoning model families and variants based on Microsoft documentation
    // GPT-5 series: gpt-5, gpt-5-mini, gpt-5-nano, gpt-5-chat, gpt-5-thinking
    n.starts_with("gpt-5") ||
    n.starts_with("gpt5") ||
    // O-series models: o1, o1-mini, o3, o3-mini, o3-pro, o4-mini
    n.starts_with("o1") ||
    n.starts_with("o3") ||
    n.starts_with("o4-mini") ||
    // Codex reasoning model
    n.starts_with("codex-mini")
}
