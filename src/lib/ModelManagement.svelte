<script lang="ts">
  import { PlusIcon } from "heroicons-svelte/24/outline"
  // Types
  export interface ModelConfig {
    name: string
    display_name: string
    provider: string
    is_enabled: boolean
    description?: string
  }

  // Props
  let { availableModels, onModelAdd, onModelRemove, onModelToggle } = $props<{
    availableModels: Record<string, ModelConfig[]>
    onModelAdd: (provider: string, model: ModelConfig) => void
    onModelRemove: (provider: string, modelIndex: number) => void
    onModelToggle: (provider: string, modelIndex: number) => void
  }>()

  // Local state
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
    return (
      availableModels[provider]?.filter((m: ModelConfig) => m.is_enabled) || []
    )
  }

  // Typed entries for template usage
  const providerEntries = $derived(
    Object.entries(availableModels) as [string, ModelConfig[]][]
  )
</script>

<div class="space-y-6">
  <!-- Add New Model -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h5 class="card-title flex items-center gap-2">
        <PlusIcon class="w-4 h-4" />
        Add New Model
      </h5>

      <div class="grid md:grid-cols-2 gap-4">
        <div class="form-control w-full">
          <label class="label" for="provider-select">
            <span class="label-text font-medium">Provider</span>
          </label>
          <select
            id="provider-select"
            class="select select-bordered bg-base-200 w-full block"
            bind:value={selectedProvider}
          >
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
            class="input input-bordered bg-base-200 min-w-0"
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
            class="input input-bordered bg-base-200 min-w-0"
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
            class="input input-bordered bg-base-200 min-w-0"
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
  {#each providerEntries as [provider, models] (provider)}
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

          <div
            class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-1"
          >
            {#each models as model, index (model.name)}
              <div class="card card-compact card-border rounded-md min-w-0">
                <div class="card-body p-2 gap-1">
                  <div
                    class="text-xs font-medium truncate"
                    title={model.display_name}
                  >
                    {model.display_name}
                  </div>
                  <div class="text-[11px] flex items-center gap-2 min-w-0">
                    <code
                      class="px-1.5 py-0.5 rounded bg-base-300/50 truncate max-w-full text-primary"
                      title={model.name}>{model.name}</code
                    >
                    {#if model.is_enabled}
                      <span class="badge badge-success badge-xs">Enabled</span>
                    {:else}
                      <span class="badge badge-outline badge-xs">Disabled</span>
                    {/if}
                  </div>
                  {#if model.description}
                    <div
                      class="text-[11px] text-base-content/70 truncate"
                      title={model.description}
                    >
                      {model.description}
                    </div>
                  {/if}
                  <div class="card-actions justify-between mt-1">
                    <button
                      type="button"
                      class="btn btn-soft btn-xs"
                      class:btn-success={model.is_enabled}
                      class:btn-outline={!model.is_enabled}
                      onclick={() => onModelToggle(provider, index)}
                      title={model.is_enabled ? "Disable model" : (
                        "Enable model"
                      )}
                      aria-label={model.is_enabled ? "Disable model" : (
                        "Enable model"
                      )}
                    >
                      {model.is_enabled ? "Enabled" : "Disabled"}
                    </button>
                    <button
                      type="button"
                      class="btn btn-ghost btn-error btn-xs"
                      onclick={() => onModelRemove(provider, index)}
                      title="Remove model"
                      aria-label="Remove model"
                    >
                      Remove
                    </button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  {/each}
</div>
