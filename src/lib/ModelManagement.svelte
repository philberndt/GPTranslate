<script lang="ts">
    interface ModelConfig {
        name: string;
        display_name: string;
        provider: string;
        is_enabled: boolean;
        description?: string;
    }
    interface Props {
        config: any;
        availableModels: Record<string, ModelConfig[]>;
        onConfigChange: (updates: any) => void;
        onModelAdd: (provider: string, model: ModelConfig) => void;
        onModelRemove: (provider: string, modelIndex: number) => void;
        onModelToggle: (provider: string, modelIndex: number) => void;
    }

    let {
        config,
        availableModels,
        onConfigChange,
        onModelAdd,
        onModelRemove,
        onModelToggle,
    }: Props = $props();

    let newModelName = $state("");
    let newModelDisplayName = $state("");
    let newModelDescription = $state("");
    let selectedProvider = $state("openai");

    function addModel() {
        if (!newModelName.trim() || !newModelDisplayName.trim()) return;

        const newModel: ModelConfig = {
            name: newModelName.trim(),
            display_name: newModelDisplayName.trim(),
            provider: selectedProvider,
            is_enabled: true,
            description: newModelDescription.trim() || undefined,
        };

        onModelAdd(selectedProvider, newModel);

        // Clear form
        newModelName = "";
        newModelDisplayName = "";
        newModelDescription = "";
    }

    function getEnabledModelsForProvider(provider: string): ModelConfig[] {
        return availableModels[provider]?.filter((m) => m.is_enabled) || [];
    }
</script>

<!-- Model Management Section -->
<section class="settings-section">
    <h3><i class="bi bi-cpu"></i>Model Management</h3>

    <!-- Add New Model -->
    <div class="model-form">
        <h4>Add New Model</h4>
        <div class="form-row">
            <div class="form-group">
                <label for="provider-select">Provider</label>
                <select id="provider-select" bind:value={selectedProvider}>
                    <option value="openai">OpenAI</option>
                    <option value="azure_openai">Azure OpenAI</option>
                    <option value="ollama">Ollama</option>
                </select>
            </div>
            <div class="form-group">
                <label for="model-name">Model Name</label>
                <input
                    id="model-name"
                    type="text"
                    bind:value={newModelName}
                    placeholder="e.g., gpt-4o-mini"
                />
            </div>
        </div>
        <div class="form-row">
            <div class="form-group">
                <label for="model-display">Display Name</label>
                <input
                    id="model-display"
                    type="text"
                    bind:value={newModelDisplayName}
                    placeholder="e.g., GPT-4o Mini"
                />
            </div>
            <div class="form-group">
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
            class="add-model-btn"
            onclick={addModel}
            disabled={!newModelName.trim() || !newModelDisplayName.trim()}
        >
            <i class="bi bi-plus-circle"></i>
            Add Model
        </button>
    </div>

    <!-- Model Lists -->
    {#each Object.entries(availableModels) as [provider, models]}
        {#if models && models.length > 0}
            <div class="provider-models">
                <h4>
                    {provider === "openai"
                        ? "OpenAI"
                        : provider === "azure_openai"
                          ? "Azure OpenAI"
                          : provider === "ollama"
                            ? "Ollama"
                            : provider} Models
                    <span class="model-count"
                        >({getEnabledModelsForProvider(provider)
                            .length}/{models.length} enabled)</span
                    >
                </h4>
                <div class="model-list">
                    {#each models as model, index}
                        <div
                            class="model-item {model.is_enabled
                                ? 'enabled'
                                : 'disabled'}"
                        >
                            <div class="model-info">
                                <div class="model-name">
                                    {model.display_name}
                                </div>
                                <div class="model-details">
                                    <span class="model-id">{model.name}</span>
                                    {#if model.description}
                                        <span class="model-description"
                                            >{model.description}</span
                                        >
                                    {/if}
                                </div>
                            </div>
                            <div class="model-actions">
                                <button
                                    class="toggle-btn {model.is_enabled
                                        ? 'enabled'
                                        : 'disabled'}"
                                    onclick={() =>
                                        onModelToggle(provider, index)}
                                    title={model.is_enabled
                                        ? "Disable model"
                                        : "Enable model"}
                                    aria-label={model.is_enabled
                                        ? "Disable model"
                                        : "Enable model"}
                                >
                                    <i
                                        class="bi {model.is_enabled
                                            ? 'bi-toggle-on'
                                            : 'bi-toggle-off'}"
                                    ></i>
                                </button>
                                <button
                                    class="remove-btn"
                                    onclick={() =>
                                        onModelRemove(provider, index)}
                                    title="Remove model"
                                    aria-label="Remove model"
                                >
                                    <i class="bi bi-trash"></i>
                                </button>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    {/each}
</section>

<style>
    .settings-section {
        margin: 24px 0;
        min-width: 0;
    }

    .settings-section h3 {
        font-size: 1.1rem;
        color: #333;
        margin: 0 0 16px 0;
        display: flex;
        align-items: center;
        gap: 8px;
        padding-bottom: 8px;
        border-bottom: 1px solid #e0e0e0;
    }

    .model-form {
        background: #f8f9fa;
        border: 1px solid #e0e0e0;
        border-radius: 8px;
        padding: 16px;
        margin-bottom: 24px;
    }
    .model-form h4 {
        margin: 0 0 12px 0;
        font-size: 0.95rem;
        color: #333;
        font-weight: 600;
    }

    .form-row {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 12px;
        margin-bottom: 12px;
    }

    .form-group {
        min-width: 0;
    }

    .form-group label {
        display: block;
        font-weight: 500;
        margin-bottom: 4px;
        color: #333;
        font-size: 13px;
    }

    .form-group input,
    .form-group select {
        width: 100%;
        padding: 8px 10px;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-family: inherit;
        font-size: 13px;
        transition: border-color 0.2s;
        box-sizing: border-box;
    }

    .form-group input:focus,
    .form-group select:focus {
        outline: none;
        border-color: #379df1;
        box-shadow: 0 0 0 2px rgba(55, 157, 241, 0.1);
    }

    .add-model-btn {
        background: #379df1;
        color: white;
        border: none;
        padding: 8px 16px;
        border-radius: 6px;
        font-size: 13px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .add-model-btn:hover:not(:disabled) {
        background: #2980e6;
    }

    .add-model-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .provider-models {
        margin-bottom: 20px;
    }

    .provider-models h4 {
        font-size: 0.95rem;
        color: #333;
        margin: 0 0 8px 0;
        display: flex;
        align-items: center;
        gap: 8px;
        font-weight: 600;
    }

    .model-count {
        font-size: 0.8rem;
        color: #666;
        font-weight: normal;
    }

    .model-list {
        border: 1px solid #e0e0e0;
        border-radius: 6px;
        overflow: hidden;
    }

    .model-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 16px;
        border-bottom: 1px solid #e0e0e0;
        transition: background-color 0.2s;
    }

    .model-item:last-child {
        border-bottom: none;
    }

    .model-item.enabled {
        background: #fff;
    }

    .model-item.disabled {
        background: #f8f9fa;
        opacity: 0.7;
    }

    .model-info {
        flex: 1;
        min-width: 0;
    }

    .model-name {
        font-weight: 500;
        color: #333;
        margin-bottom: 2px;
    }

    .model-details {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .model-id {
        font-size: 0.85rem;
        color: #666;
        font-family: "Consolas", "Monaco", "Courier New", monospace;
    }

    .model-description {
        font-size: 0.8rem;
        color: #888;
        font-style: italic;
    }

    .model-actions {
        display: flex;
        gap: 8px;
        align-items: center;
    }

    .toggle-btn,
    .remove-btn {
        background: none;
        border: none;
        padding: 4px;
        cursor: pointer;
        border-radius: 4px;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 16px;
    }

    .toggle-btn.enabled {
        color: #28a745;
    }

    .toggle-btn.disabled {
        color: #6c757d;
    }

    .remove-btn {
        color: #dc3545;
    }

    .toggle-btn:hover,
    .remove-btn:hover {
        background: rgba(0, 0, 0, 0.05);
    }

    @media (max-width: 768px) {
        .form-row {
            grid-template-columns: 1fr;
        }

        .model-item {
            flex-direction: column;
            align-items: flex-start;
            gap: 8px;
        }

        .model-actions {
            align-self: flex-end;
        }
    }

    /* Dark mode */
    @media (prefers-color-scheme: dark) {
        .settings-section h3 {
            color: #f6f6f6;
            border-color: #444;
        }

        .model-form {
            background: #383838;
            border-color: #444;
        }

        .model-form h4,
        .provider-models h4 {
            color: #f6f6f6;
        }

        .form-group label {
            color: #f6f6f6;
        }

        .form-group input,
        .form-group select {
            background: #3a3a3a;
            border-color: #555;
            color: #f6f6f6;
        }

        .form-group input:focus,
        .form-group select:focus {
            border-color: #379df1;
            background: #404040;
        }

        .model-count {
            color: #ccc;
        }

        .model-list {
            border-color: #444;
        }

        .model-item {
            border-color: #444;
        }

        .model-item.enabled {
            background: #2d2d2d;
        }

        .model-item.disabled {
            background: #383838;
        }

        .model-name {
            color: #f6f6f6;
        }

        .model-id {
            color: #ccc;
        }

        .model-description {
            color: #aaa;
        }

        .toggle-btn:hover,
        .remove-btn:hover {
            background: rgba(255, 255, 255, 0.1);
        }
    }
</style>
