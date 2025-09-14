export interface Config {
  api_provider: string;
  openai_api_key: string;
  azure_api_key: string;
  azure_endpoint: string;
  azure_translator_api_key: string;
  azure_translator_endpoint: string;
  ollama_url?: string;
}

/**
 * Check if a specific API provider is configured
 */
export function isProviderConfigured(
  config: Config | null,
  provider: string,
): boolean {
  if (!config) return false;

  switch (provider) {
    case "openai":
      return config.openai_api_key?.length > 0;
    case "azure_openai":
      return (
        config.azure_api_key?.length > 0 && config.azure_endpoint?.length > 0
      );
    case "azure_translator":
      return !!(
        config.azure_translator_api_key &&
        config.azure_translator_api_key.length > 0 &&
        config.azure_translator_endpoint &&
        config.azure_translator_endpoint.length > 0
      );
    case "ollama":
      return !!(config.ollama_url && config.ollama_url.length > 0);
    default:
      return false;
  }
}

/**
 * Check if any API provider is configured
 */
export function isAnyProviderConfigured(config: Config | null): boolean {
  if (!config) return false;

  const providers = ["openai", "azure_openai", "azure_translator", "ollama"];
  return providers.some((provider) => isProviderConfigured(config, provider));
}

/**
 * Get list of configured providers
 */
export function getConfiguredProviders(config: Config | null): string[] {
  if (!config) return [];

  const providers = ["openai", "azure_openai", "azure_translator", "ollama"];
  return providers.filter((provider) => isProviderConfigured(config, provider));
}
