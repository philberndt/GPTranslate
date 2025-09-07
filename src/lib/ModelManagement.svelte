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
<div class="space-y-6">
  <h4 class="text-lg font-semibold text-base-content">Model Management</h4>

  <!-- Add New Model -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h5 class="card-title">Add New Model</h5>
      
      <div class="grid md:grid-cols-2 gap-4">
        <div class="form-control w-full">
          <label class="label" for="provider-select">
            <span class="label-text font-medium">Provider</span>
          </label>
          <select id="provider-select" class="select select-bordered bg-base-200" bind:value={selectedProvider}>
            <option value="openai">OpenAI</option>
            <option value="azure_openai">Azure OpenAI</option>
            <option value="ollama">Ollama</option>
          </select>
        </div>
        <div class="form-control w-full">
          <label class="label" for="model-name">
            <span class="label-text font-medium">Model Name</span>
          </label>
          <input
            id="model-name"
            type="text"
            class="input input-bordered bg-base-200"
            bind:value={newModelName}
            placeholder="e.g., gpt-4o-mini"
          />
        </div>
      </div>
      
      <div class="grid md:grid-cols-2 gap-4">
        <div class="form-control w-full">
          <label class="label" for="model-display">
            <span class="label-text font-medium">Display Name</span>
          </label>
          <input
            id="model-display"
            type="text"
            class="input input-bordered bg-base-200"
            bind:value={newModelDisplayName}
            placeholder="e.g., GPT-4o Mini"
          />
        </div>
        <div class="form-control w-full">
          <label class="label" for="model-description">
            <span class="label-text font-medium">Description (optional)</span>
          </label>
          <input
            id="model-description"
            type="text"
            class="input input-bordered bg-base-200"
            bind:value={newModelDescription}
            placeholder="e.g., Fast and cost-effective model"
          />
        </div>
      </div>
      
      <div class="card-actions justify-end mt-4">
        <button
          class="btn btn-primary"
          onclick={addModel}
          disabled={!newModelName.trim() || !newModelDisplayName.trim()}
        >
          Add Model
        </button>
      </div>
    </div>
  </div>

  <div class="divider"></div>

  <!-- Model Lists -->
  {#each Object.entries(availableModels) as [provider, models] (provider)}
    {#if models && models.length > 0 && provider !== "azure_translator"}
      <div class="card bg-base-100 border border-base-300/50">
        <div class="card-body">
          <div class="flex items-center justify-between mb-4">
            <h5 class="card-title text-base">
              {provider === "openai" ? "OpenAI"
              : provider === "azure_openai" ? "Azure OpenAI"
              : provider === "ollama" ? "Ollama"
              : provider} Models
            </h5>
            <span class="badge badge-outline badge-sm">
              {getEnabledModelsForProvider(provider).length}/{models.length} enabled
            </span>
          </div>

          <div class="space-y-3">
            {#each models as model, index (model.name)}
              <div class="flex items-center justify-between p-3 bg-base-200 rounded-lg border border-base-300/30">
                <div class="flex-1">
                  <div class="font-medium text-base-content">
                    {model.display_name}
                  </div>
                  <div class="text-sm space-y-1">
                    <code class="text-xs px-2 py-1 rounded bg-base-300/50 {model.is_enabled ? 'text-primary' : 'text-base-content/60'}"
                      >{model.name}</code>
                    {#if model.description}
                      <div class="text-base-content/70">{model.description}</div>
                    {/if}
                  </div>
                </div>
                <div class="flex items-center gap-2 ml-4">
                  <button
                    type="button"
                    class="btn btn-sm {model.is_enabled ? 'btn-success' : 'btn-outline'}"
                    onclick={() => onModelToggle(provider, index)}
                    title={model.is_enabled ? "Disable model" : "Enable model"}
                    aria-label={model.is_enabled ? "Disable model" : "Enable model"}
                  >
                    {model.is_enabled ? "Enabled" : "Disabled"}
                  </button>
                  <button
                    type="button"
                    class="btn btn-sm btn-error btn-outline"
                    onclick={() => onModelRemove(provider, index)}
                    title="Remove model"
                    aria-label="Remove model"
                  >
                    Remove
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  {/each}
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
