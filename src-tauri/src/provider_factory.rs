use crate::config::Config;
use crate::trans_openai::OpenAITranslationService;
use crate::trans_azure::AzureOpenAITranslationService;
use crate::trans_azure_translator::AzureTranslatorService;
use crate::trans_ollama::OllamaTranslationService;
use crate::translation::TranslationProvider;

/// Central place to construct providers so adding new ones touches fewer files.
pub fn create_provider(config: Config) -> Box<dyn TranslationProvider + Send + Sync> {
    match config.api_provider.as_str() {
        "openai" => Box::new(OpenAITranslationService::new(config)),
        "azure_openai" => Box::new(AzureOpenAITranslationService::new(config)),
        "azure_translator" => Box::new(AzureTranslatorService::new(config)),
        "ollama" => Box::new(OllamaTranslationService::new(config)),
        other => {
            log::warn!("Unknown API provider '{}' , defaulting to OpenAI", other);
            let mut fallback = config.clone();
            fallback.api_provider = "openai".into();
            Box::new(OpenAITranslationService::new(fallback))
        }
    }
}
