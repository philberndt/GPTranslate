use crate::config::Config;
use crate::translation::{
    TranslationProvider, TranslationResult, clean_text_for_translation, create_smart_prompt,
};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

pub struct OpenAITranslationService {
    client: reqwest::Client,
    config: Config,
}

impl OpenAITranslationService {
    pub fn new(config: Config) -> Self {
        log::info!(
            "Creating OpenAITranslationService with model: {}",
            config.model
        );
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    fn is_reasoning_model(&self) -> bool {
        // Automatic detection based on known reasoning model name patterns
        is_reasoning_model_name(self.config.model.as_str())
    }

    async fn call_openai(&self, request_body: Value) -> Result<Value> {
        let url = "https://api.openai.com/v1/chat/completions";

        log::info!("Making OpenAI request to: {}", url);
        log::info!(
            "Request body: {}",
            serde_json::to_string_pretty(&request_body).unwrap_or_default()
        );

        let response = self
            .client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.openai_api_key),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            log::error!("OpenAI API request failed: {}", error_text);
            return Err(anyhow::anyhow!("OpenAI API request failed: {}", error_text));
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
                // Properly handle escaped newlines that OpenAI might return
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
        );

        // Determine the actual target language used for translation
        let detected_lower = detected_language.to_lowercase();
        let target_lower = self.config.target_language.to_lowercase();

        let actual_target_language = if detected_lower == target_lower {
            // Alternative language logic was applied
            log::info!(
                "OpenAI - Alternative language logic SHOULD apply: detected '{}' matches target '{}', should translate to '{}'",
                detected_language,
                self.config.target_language,
                self.config.alternative_target_language
            );
            self.config.alternative_target_language.clone()
        } else {
            // Primary target language was used
            log::info!(
                "OpenAI - Primary target logic SHOULD apply: detected '{}' != target '{}', should translate to '{}'",
                detected_language,
                self.config.target_language,
                self.config.target_language
            );
            self.config.target_language.clone()
        };

        log::info!("Returning target_language: {}", actual_target_language);

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
            target_language: actual_target_language,
        })
    }
}

#[async_trait]
impl TranslationProvider for OpenAITranslationService {
    async fn translate(&self, text: &str) -> Result<TranslationResult> {
        let cleaned_text = clean_text_for_translation(text);
        log::info!("Cleaned text for translation: {}", cleaned_text);

        let user_prompt = format!("Text to translate: \"{}\"", cleaned_text);
        let smart_prompt = create_smart_prompt(&self.config, None);

        log::info!("Using smart prompt with alternative language logic");

        // Check if current model is a reasoning model
        let is_reasoning_model = self.is_reasoning_model();
        log::info!(
            "Model {} is reasoning: {}",
            self.config.model,
            is_reasoning_model
        );

        // Role selection based on reasoning model type:
        // - GPT-5 models: Use "system" role (they support system messages)
        // - O-series models (o1, o3, o4-mini): Use "developer" role
        // - Non-reasoning models: Use "system" role
        let model_name = &self.config.model;
        let system_role = if is_reasoning_model
            && (model_name.starts_with("o1")
                || model_name.starts_with("o3")
                || model_name.starts_with("o4-mini"))
        {
            "developer"
        } else {
            "system"
        };
        let system_content = if is_reasoning_model {
            // For reasoning models, use much simpler instructions with formatting re-enabled
            format!("Formatting re-enabled - please respond with valid JSON.\n\n{}\n\nRespond with JSON containing 'detected_language' and 'translated_text' fields.", smart_prompt)
        } else {
            // For non-reasoning models, use detailed instructions
            format!(
                "{}\n\nIMPORTANT FORMATTING RULES:\n- Always respond with valid JSON containing 'detected_language' and 'translated_text' fields\n- Preserve line breaks and paragraph structure in the translation\n- Use actual newline characters (\\n) in the JSON string value, not escaped \\\\n\n- Do NOT escape newlines in the JSON response - use literal newlines\n\nExample response format:\n{{\n  \"detected_language\": \"English\",\n  \"translated_text\": \"Line 1\\nLine 2\\n\\nNew paragraph\"\n}}",
                smart_prompt
            )
        };

        let mut request_body = json!({
            "model": self.config.model,
            "messages": [
                {
                    "role": system_role,
                    "content": system_content
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ],
        });

        // Use appropriate token parameter based on model type
        if is_reasoning_model {
            request_body["max_completion_tokens"] = json!(800);
            // Add reasoning object for reasoning models
            let effort = self.config.reasoning_effort.as_deref().unwrap_or("low");
            request_body["reasoning"] = json!({
                "effort": effort
            });
            log::info!("OpenAI - Using reasoning effort: {}", effort);
            
            // Add verbosity parameter for GPT-5 models
            if self.config.model.starts_with("gpt-5") {
                request_body["verbosity"] = json!("medium");
                log::info!("OpenAI - Using verbosity: medium for GPT-5 model");
            }
            
            // Note: Reasoning models don't support temperature, top_p, presence_penalty, frequency_penalty, logprobs, top_logprobs, logit_bias parameters
        } else {
            request_body["max_tokens"] = json!(800);
            request_body["temperature"] = json!(0.3);
        }

        log::info!("Using OpenAI model: {}", self.config.model);

        let response = self.call_openai(request_body).await?;
        
        // Better error handling for response structure
        let choices = response["choices"].as_array()
            .ok_or_else(|| anyhow::anyhow!("No 'choices' array in response. Full response: {}", serde_json::to_string_pretty(&response).unwrap_or_default()))?;
            
        if choices.is_empty() {
            return Err(anyhow::anyhow!("Empty 'choices' array in response. Full response: {}", serde_json::to_string_pretty(&response).unwrap_or_default()));
        }
        
        let message = &choices[0]["message"];
        if message.is_null() {
            return Err(anyhow::anyhow!("No 'message' object in first choice. Full response: {}", serde_json::to_string_pretty(&response).unwrap_or_default()));
        }
        
        let content = message["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No 'content' field in message or content is not a string. Message: {}", serde_json::to_string_pretty(message).unwrap_or_default()))?;

        self.parse_response_content(content)
    }
}

fn is_reasoning_model_name(name: &str) -> bool {
    let n = name.to_lowercase();
    // Known OpenAI reasoning model families and variants based on Microsoft documentation
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
