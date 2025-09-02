<script lang="ts">
  interface ModelConfig {
    name: string
    display_name: string
    provider: string
    is_enabled: boolean
    description?: string
  }
  interface Props {
    config: any
    availableModels: Record<string, ModelConfig[]>
    onConfigChange: (updates: any) => void
    onModelAdd: (provider: string, model: ModelConfig) => void
    onModelRemove: (provider: string, modelIndex: number) => void
    onModelToggle: (provider: string, modelIndex: number) => void
  }
  let {
    // config,
    availableModels,
    // onConfigChange,
    onModelAdd,
    onModelRemove,
    onModelToggle,
  }: Props = $props()

  let newModelName = $state("")
  let newModelDisplayName = $state("")
  let newModelDescription = $state("")
  let selectedProvider = $state("openai")

  function addModel() {
    if (!newModelName.trim() || !newModelDisplayName.trim()) return

    const newModel: ModelConfig = {
      name: newModelName.trim(),
      display_name: newModelDisplayName.trim(),
      provider: selectedProvider,
      is_enabled: true,
      description: newModelDescription.trim() || undefined,
    }

    onModelAdd(selectedProvider, newModel)

    // Clear form
    newModelName = ""
    newModelDisplayName = ""
    newModelDescription = ""
  }

  function getEnabledModelsForProvider(provider: string): ModelConfig[] {
    return availableModels[provider]?.filter((m) => m.is_enabled) || []
  }
</script>

<!-- Model Management Section -->
<div>
  <h4>Model Management</h4>

  <!-- Add New Model -->
  <div>
    <h5>Add New Model</h5>
    <div>
      <div>
        <label for="provider-select">Provider</label>
        <select id="provider-select" bind:value={selectedProvider}>
          <option value="openai">OpenAI</option>
          <option value="azure_openai">Azure OpenAI</option>
          <option value="ollama">Ollama</option>
        </select>
      </div>
      <div>
        <label for="model-name">Model Name</label>
        <input
          id="model-name"
          type="text"
          bind:value={newModelName}
          placeholder="e.g., gpt-4o-mini"
        />
      </div>
    </div>
    <div>
      <div>
        <label for="model-display">Display Name</label>
        <input
          id="model-display"
          type="text"
          bind:value={newModelDisplayName}
          placeholder="e.g., GPT-4o Mini"
        />
      </div>
      <div>
        <label for="model-description">Description (optional)</label>
        <input
          id="model-description"
          type="text"
          bind:value={newModelDescription}
          placeholder="e.g., Fast and cost-effective model"
        />
      </div>
    </div>
    <button
      onclick={addModel}
      disabled={!newModelName.trim() || !newModelDisplayName.trim()}
    >
      Add Model
    </button>
  </div>

  <hr />

  <!-- Model Lists -->
  {#each Object.entries(availableModels) as [provider, models] (provider)}
    {#if models && models.length > 0 && provider !== "azure_translator"}
      <div>
        <div>
          <h5>
            {provider === "openai" ? "OpenAI"
            : provider === "azure_openai" ? "Azure OpenAI"
            : provider === "ollama" ? "Ollama"
            : provider} Models
          </h5>
          <span
            >({getEnabledModelsForProvider(provider).length}/{models.length} enabled)</span
          >
        </div>

        <div>
          {#each models as model, index (model.name)}
            <div>
              <div>
                <div>
                  {model.display_name}
                </div>
                <div>
                  <code class={model.is_enabled ? "text-primary" : "text-muted"}
                    >{model.name}</code
                  >
                  {#if model.description}
                    <span>{model.description}</span>
                  {/if}
                </div>
              </div>
              <div>
                <button
                  type="button"
                  onclick={() => onModelToggle(provider, index)}
                  title={model.is_enabled ? "Disable model" : "Enable model"}
                  aria-label={model.is_enabled ? "Disable model" : (
                    "Enable model"
                  )}
                >
                </button>
                <button
                  type="button"
                  onclick={() => onModelRemove(provider, index)}
                  title="Remove model"
                  aria-label="Remove model"
                >
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/each}
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
