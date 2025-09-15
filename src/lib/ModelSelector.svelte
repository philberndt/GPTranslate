<script lang="ts">
  import { invoke } from "./tauri"

  interface ModelConfig {
    name: string
    display_name: string
    provider: string
    is_enabled: boolean
    description?: string
  }

  interface Config {
    api_provider: string
    model: string
    available_models: Record<string, ModelConfig[]>
    openai_api_key: string
    azure_api_key: string
    azure_translator_api_key: string
    azure_translator_endpoint: string
    azure_translator_region: string
    ollama_url: string
    azure_deployment_name?: string
  }
  interface Props {
    config: Config | null
    onModelChange?: (model: string, provider: string) => void
  }

  let { config, onModelChange }: Props = $props()

  // Derive selectedModel from config changes
  let selectedModel = $derived.by(() => {
    if (!config) return ""
    
    let newSelectedModel = ""
    // Special case for Azure AI Translator - no model needed
    if (config.api_provider === "azure_translator") {
      newSelectedModel = ""
    } else if (config.model && config.model.trim()) {
      newSelectedModel = config.model
    } else if (
      config.api_provider === "azure_openai" &&
      config.azure_deployment_name
    ) {
      newSelectedModel = config.azure_deployment_name
    } else {
      const firstEnabled = (
        config.available_models?.[config.api_provider] || []
      ).find((m) => m.is_enabled)
      if (firstEnabled) newSelectedModel = firstEnabled.name
    }
    return newSelectedModel
  })

  // Derive selectionValue from config and selectedModel
  let selectionValue = $derived.by(() => {
    if (!config) return ""
    if (config.api_provider === "azure_translator") return "azure_translator::"

    const key =
      selectedModel ||
      config.model ||
      (config.api_provider === "azure_openai" ?
        config.azure_deployment_name || ""
      : "")

    return `${config.api_provider}::${key || ""}`
  })

  function isProviderConfigured(provider: string): boolean {
    if (!config) return false

    switch (provider) {
      case "openai":
        return config.openai_api_key.length > 0
      case "azure_openai":
        return config.azure_api_key.length > 0
      case "azure_translator":
        return !!(
          config.azure_translator_api_key &&
          config.azure_translator_api_key.length > 0 &&
          config.azure_translator_endpoint &&
          config.azure_translator_endpoint.length > 0
        )
      case "ollama":
        return config.ollama_url?.length > 0
      default:
        return false
    }
  }
  async function selectModel(provider: string, modelName: string) {

    try {
      if (config) {
        // Update selectedModel immediately for visual feedback
        selectedModel = modelName

        // Create a new config object to avoid mutating props
        const newConfig = {
          ...config,
          api_provider: provider,
          model: modelName,
          // Keep azure_deployment_name in sync with model for Azure
          azure_deployment_name:
            provider === "azure_openai" ? modelName : (
              config.azure_deployment_name || ""
            ),
        }

        await invoke("save_config", { newConfig: newConfig })

        // Call onModelChange to let parent handle config updates
        if (onModelChange) {
          onModelChange(modelName, provider)
        }

        // closed state no longer needed for native select
      }
    } catch (e) {
      console.error("Failed to save model selection:", e)
    }
  }

  function getEnabledModelsForProvider(provider: string): ModelConfig[] {
    if (!config?.available_models) return []
    return config.available_models[provider]?.filter((m) => m.is_enabled) || []
  }

  function hasAnyModels(): boolean {
    if (!config?.available_models) return false
    // Check if Azure AI Translator is configured
    if (isProviderConfigured("azure_translator")) return true
    // Only count models that are enabled
    return Object.values(config.available_models).some((models) =>
      models.some((m) => m.is_enabled)
    )
  }
  function readableProviderName(provider: string): string {
    if (provider === "azure_openai") return "Azure OpenAI"
    if (provider === "azure_translator") return "Azure AI Translator"
    return provider.replace("_", " ").replace(/\b\w/g, (l) => l.toUpperCase())
  }

  function handleSelectChange(value: string) {
    const [provider, modelName = ""] = value.split("::")
    // Guard: if provider missing, ignore
    if (!provider) return
    selectModel(provider, modelName)
  }
</script>

{#if config}
  <select
    class="select select-ghost select-sm"
    value={selectionValue}
    onchange={(e) => handleSelectChange((e.target as HTMLSelectElement).value)}
    aria-label="Select AI model"
    title="Select AI model"
  >
    {#if hasAnyModels()}
      {#if isProviderConfigured("azure_translator")}
        <optgroup label={readableProviderName("azure_translator")}>
          <option value="azure_translator::">Azure AI Translator</option>
        </optgroup>
      {/if}
      {#each Object.keys(config.available_models) as provider (provider)}
        {#if getEnabledModelsForProvider(provider).length > 0}
          <optgroup label={readableProviderName(provider)}>
            {#each getEnabledModelsForProvider(provider) as model (model.name)}
              <option value={`${provider}::${model.name}`}>
                {model.display_name}
              </option>
            {/each}
          </optgroup>
        {/if}
      {/each}
    {:else}
      <option disabled selected>No models configured</option>
    {/if}
  </select>
{:else}
  <select class="select select-ghost select-sm" disabled>
    <option>Loading models...</option>
  </select>
{/if}

<!-- Custom CSS goes in /src/styles.css */ -->
