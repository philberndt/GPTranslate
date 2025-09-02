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
<div class="">
  <h4 class=""><i class=""></i>Model Management</h4>

  <!-- Add New Model -->
  <div class="">
    <h5 class="">
      <i class=""></i>
      Add New Model
    </h5>
    <div class="">
      <div class="">
        <label for="provider-select" class="">Provider</label>
        <select id="provider-select" class="" bind:value={selectedProvider}>
          <option value="openai">OpenAI</option>
          <option value="azure_openai">Azure OpenAI</option>
          <option value="ollama">Ollama</option>
        </select>
      </div>
      <div class="">
        <label for="model-name" class="">Model Name</label>
        <input
          id="model-name"
          type="text"
          class=""
          bind:value={newModelName}
          placeholder="e.g., gpt-4o-mini"
        />
      </div>
    </div>
    <div class="">
      <div class="">
        <label for="model-display" class="">Display Name</label>
        <input
          id="model-display"
          type="text"
          class=""
          bind:value={newModelDisplayName}
          placeholder="e.g., GPT-4o Mini"
        />
      </div>
      <div class="">
        <label for="model-description" class="">Description (optional)</label>
        <input
          id="model-description"
          type="text"
          class=""
          bind:value={newModelDescription}
          placeholder="e.g., Fast and cost-effective model"
        />
      </div>
    </div>
    <button
      class=""
      onclick={addModel}
      disabled={!newModelName.trim() || !newModelDisplayName.trim()}
    >
      <i class=""></i>
      Add Model
    </button>
  </div>

  <hr class="" />

  <!-- Model Lists -->
  {#each Object.entries(availableModels) as [provider, models] (provider)}
    {#if models && models.length > 0 && provider !== "azure_translator"}
      <div class="">
        <div class="">
          <h5 class="">
            <i class=""></i>
            {provider === "openai" ? "OpenAI"
            : provider === "azure_openai" ? "Azure OpenAI"
            : provider === "ollama" ? "Ollama"
            : provider} Models
          </h5>
          <span class=""
            >({getEnabledModelsForProvider(provider).length}/{models.length} enabled)</span
          >
        </div>

        <div class="">
          {#each models as model, index (model.name)}
            <div class="">
              <div class="">
                <div class="">
                  {model.display_name}
                </div>
                <div class="">
                  <code class={model.is_enabled ? "text-primary" : "text-muted"}
                    >{model.name}</code
                  >
                  {#if model.description}
                    <span class="">{model.description}</span>
                  {/if}
                </div>
              </div>
              <div class="">
                <button
                  type="button"
                  class=""
                  onclick={() => onModelToggle(provider, index)}
                  title={model.is_enabled ? "Disable model" : "Enable model"}
                  aria-label={model.is_enabled ? "Disable model" : (
                    "Enable model"
                  )}
                >
                  <i class=""></i>
                </button>
                <button
                  type="button"
                  class=""
                  onclick={() => onModelRemove(provider, index)}
                  title="Remove model"
                  aria-label="Remove model"
                >
                  <i class=""></i>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/each}
</div>

<style>
  /* CSS goes in /src/styles.css */
</style>
