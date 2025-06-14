<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    // import { onMount } from "svelte"; // TODO: Implement if needed

    interface ModelConfig {
        name: string;
        display_name: string;
        provider: string;
        is_enabled: boolean;
        description?: string;
    }

    interface Config {
        api_provider: string;
        model: string;
        available_models: Record<string, ModelConfig[]>;
        openai_api_key: string;
        azure_api_key: string;
        ollama_url: string;
    }
    interface Props {
        config: Config | null;
        onModelChange?: (model: string, provider: string) => void;
    }

    let { config, onModelChange }: Props = $props();

    let isOpen = $state(false);
    let selectedModel = $state("");

    // Update selectedModel when config changes
    $effect(() => {
        if (config) {
            selectedModel = config.model;
        }
    });

    function getProviderDisplayName(provider: string): string {
        switch (provider) {
            case "openai":
                return "OpenAI";
            case "azure_openai":
                return "Azure OpenAI";
            case "ollama":
                return "Ollama";
            default:
                return provider;
        }
    }
    function getProviderIcon(provider: string) {
        switch (provider) {
            case "openai":
                return `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" fill="currentColor" viewBox="0 0 16 16">
                    <path d="M14.949 6.547a3.94 3.94 0 0 0-.348-3.273 4.11 4.11 0 0 0-4.4-1.934A4.1 4.1 0 0 0 8.423.2 4.15 4.15 0 0 0 6.305.086a4.1 4.1 0 0 0-1.891.948 4.04 4.04 0 0 0-1.158 1.753 4.1 4.1 0 0 0-1.563.679A4 4 0 0 0 .554 4.72a3.99 3.99 0 0 0 .502 4.731 3.94 3.94 0 0 0 .346 3.274 4.11 4.11 0 0 0 4.402 1.933c.382.425.852.764 1.377.995.526.231 1.095.35 1.67.346 1.78.002 3.358-1.132 3.901-2.804a4.1 4.1 0 0 0 1.563-.68 4 4 0 0 0 1.14-1.253 3.99 3.99 0 0 0-.506-4.716m-6.097 8.406a3.05 3.05 0 0 1-1.945-.694l.096-.054 3.23-1.838a.53.53 0 0 0 .265-.455v-4.49l1.366.778q.02.011.025.035v3.722c-.003 1.653-1.361 2.992-3.037 2.996m-6.53-2.75a2.95 2.95 0 0 1-.36-2.01l.095.057L5.29 12.09a.53.53 0 0 0 .527 0l3.949-2.246v1.555a.05.05 0 0 1-.022.041L6.473 13.3c-1.454.826-3.311.335-4.15-1.098m-.85-6.94A3.02 3.02 0 0 1 3.07 3.949v3.785a.51.51 0 0 0 .262.451l3.93 2.237-1.366.779a.05.05 0 0 1-.048 0L2.585 9.342a2.98 2.98 0 0 1-1.113-4.094zm11.216 2.571L8.747 5.576l1.362-.776a.05.05 0 0 1 .048 0l3.265 1.86a3 3 0 0 1 1.173 1.207 2.96 2.96 0 0 1-.27 3.2 3.05 3.05 0 0 1-1.36.997V8.279a.52.52 0 0 0-.276-.445m1.36-2.015-.097-.057-3.226-1.855a.53.53 0 0 0-.53 0L6.249 6.153V4.598a.04.04 0 0 1 .019-.04L9.533 2.7a3.07 3.07 0 0 1 3.257.139c.474.325.843.778 1.066 1.303.223.526.289 1.103.191 1.664zM5.503 8.575 4.139 7.8a.05.05 0 0 1-.026-.037V4.049c0-.57.166-1.127.476-1.607s.752-.864 1.275-1.105a3.08 3.08 0 0 1 3.234.41l-.096.054-3.23 1.838a.53.53 0 0 0-.265.455zm.742-1.577 1.758-1 1.762 1v2l-1.755 1-1.762-1z"/>
                </svg>`;
            case "azure_openai":
                return `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" fill="currentColor" viewBox="0 0 16 16">
                    <path d="M7.462 0H0v7.19h7.462zM16 0H8.538v7.19H16zM7.462 8.211H0V16h7.462zm8.538 0H8.538V16H16z"/>
                </svg>`;
            case "ollama":
                return `<svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" fill-rule="evenodd" style="flex:none;line-height:1" viewBox="0 0 24 24" width="14" height="14">
                    <path d="M7.905 1.09c.216.085.411.225.588.41.295.306.544.744.734 1.263.191.522.315 1.1.362 1.68a5.054 5.054 0 0 1 2.049-.636l.051-.004c.87-.07 1.73.087 2.48.474.101.053.2.11.297.17.05-.569.172-1.134.36-1.644.19-.52.439-.957.733-1.264a1.67 1.67 0 0 1 .589-.41c.257-.1.53-.118.796-.042.401.114.745.368 1.016.737.248.337.434.769.561 1.287.23.934.27 2.163.115 3.645l.053.04.026.019c.757.576 1.284 1.397 1.563 2.35.435 1.487.216 3.155-.534 4.088l-.018.021.002.003c.417.762.67 1.567.724 2.4l.002.03c.064 1.065-.2 2.137-.814 3.19l-.007.01.01.024c.472 1.157.62 2.322.438 3.486l-.006.039a.651.651 0 0 1-.747.536.648.648 0 0 1-.54-.742c.167-1.033.01-2.069-.48-3.123a.643.643 0 0 1 .04-.617l.004-.006c.604-.924.854-1.83.8-2.72-.046-.779-.325-1.544-.8-2.273a.644.644 0 0 1 .18-.886l.009-.006c.243-.159.467-.565.58-1.12a4.229 4.229 0 0 0-.095-1.974c-.205-.7-.58-1.284-1.105-1.683-.595-.454-1.383-.673-2.38-.61a.653.653 0 0 1-.632-.371c-.314-.665-.772-1.141-1.343-1.436a3.288 3.288 0 0 0-1.772-.332c-1.245.099-2.343.801-2.67 1.686a.652.652 0 0 1-.61.425c-1.067.002-1.893.252-2.497.703-.522.39-.878.935-1.066 1.588a4.07 4.07 0 0 0-.068 1.886c.112.558.331 1.02.582 1.269l.008.007c.212.207.257.53.109.785-.36.622-.629 1.549-.673 2.44-.05 1.018.186 1.902.719 2.536l.016.019a.643.643 0 0 1 .095.69c-.576 1.236-.753 2.252-.562 3.052a.652.652 0 0 1-1.269.298c-.243-1.018-.078-2.184.473-3.498l.014-.035-.008-.012a4.339 4.339 0 0 1-.598-1.309l-.005-.019a5.764 5.764 0 0 1-.177-1.785c.044-.91.278-1.842.622-2.59l.012-.026-.002-.002c-.293-.418-.51-.953-.63-1.545l-.005-.024a5.352 5.352 0 0 1 .093-2.49c.262-.915.777-1.701 1.536-2.269.06-.045.123-.09.186-.132-.159-1.493-.119-2.73.112-3.67.127-.518.314-.95.562-1.287.27-.368.614-.622 1.015-.737.266-.076.54-.059.797.042zm4.116 9.09c.936 0 1.8.313 2.446.855.63.527 1.005 1.235 1.005 1.94 0 .888-.406 1.58-1.133 2.022-.62.375-1.451.557-2.403.557-1.009 0-1.871-.259-2.493-.734-.617-.47-.963-1.13-.963-1.845 0-.707.398-1.417 1.056-1.946.668-.537 1.55-.849 2.485-.849zm0 .896a3.07 3.07 0 0 0-1.916.65c-.461.37-.722.835-.722 1.25 0 .428.21.829.61 1.134.455.347 1.124.548 1.943.548.799 0 1.473-.147 1.932-.426.463-.28.7-.686.7-1.257 0-.423-.246-.89-.683-1.256-.484-.405-1.14-.643-1.864-.643zm.662 1.21.004.004c.12.151.095.37-.056.49l-.292.23v.446a.375.375 0 0 1-.376.373.375.375 0 0 1-.376-.373v-.46l-.271-.218a.347.347 0 0 1-.052-.49.353.353 0 0 1 .494-.051l.215.172.22-.174a.353.353 0 0 1 .49.051zm-5.04-1.919c.478 0 .867.39.867.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zm8.706 0c.48 0 .868.39.868.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zM7.44 2.3l-.003.002a.659.659 0 0 0-.285.238l-.005.006c-.138.189-.258.467-.348.832-.17.692-.216 1.631-.124 2.782.43-.128.899-.208 1.404-.237l.01-.001.019-.034c.046-.082.095-.161.148-.239.123-.771.022-1.692-.253-2.444-.134-.364-.297-.65-.453-.813a.628.628 0 0 0-.107-.09L7.44 2.3zm9.174.04-.002.001a.628.628 0 0 0-.107.09c-.156.163-.32.45-.453.814-.29.794-.387 1.776-.23 2.572l.058.097.008.014h.03a5.184 5.184 0 0 1 1.466.212c.086-1.124.038-2.043-.128-2.722-.09-.365-.21-.643-.349-.832l-.004-.006a.659.659 0 0 0-.285-.239h-.004z"/>
                </svg>`;
            default:
                return `<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" fill="currentColor" viewBox="0 0 16 16">
                    <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z"/>
                </svg>`;
        }
    }

    function isProviderConfigured(provider: string): boolean {
        if (!config) return false;

        switch (provider) {
            case "openai":
                return config.openai_api_key.length > 0;
            case "azure_openai":
                return config.azure_api_key.length > 0;
            case "ollama":
                return config.ollama_url?.length > 0;
            default:
                return false;
        }
    }
    async function selectModel(provider: string, modelName: string) {
        try {
            if (config) {
                selectedModel = modelName;

                // Create a new config object to avoid mutating props
                const newConfig = {
                    ...config,
                    api_provider: provider,
                    model: modelName,
                };

                await invoke("save_config", { newConfig });

                if (onModelChange) {
                    onModelChange(modelName, provider);
                }

                isOpen = false;
            }
        } catch (e) {
            console.error("Failed to save model selection:", e);
        }
    }

    function getCurrentProviderDisplayName(): string {
        if (!config) return "Loading...";
        return getProviderDisplayName(config.api_provider);
    }

    function getCurrentModelDisplayName(): string {
        if (!config || !config.available_models) return selectedModel;

        const providerModels =
            config.available_models[config.api_provider] || [];
        const currentModel = providerModels.find(
            (m) => m.name === selectedModel,
        );

        return currentModel?.display_name || selectedModel;
    }

    function getEnabledModelsForProvider(provider: string): ModelConfig[] {
        if (!config?.available_models) return [];
        return (
            config.available_models[provider]?.filter((m) => m.is_enabled) || []
        );
    }

    function hasEnabledModels(provider: string): boolean {
        return getEnabledModelsForProvider(provider).length > 0;
    }

    function hasAnyModels(): boolean {
        if (!config?.available_models) return false;
        return Object.values(config.available_models).some(
            (models) => models.length > 0,
        );
    }
</script>

<div class="model-selector">
    <button
        class="model-selector-button {isOpen ? 'open' : ''}"
        onclick={() => (isOpen = !isOpen)}
        title="Select AI Model"
    >
        <div class="model-info">
            <span class="provider-name">{getCurrentProviderDisplayName()}</span>
            <span class="model-name">{getCurrentModelDisplayName()}</span>
        </div>
        <i class="bi bi-chevron-up {isOpen ? 'rotated' : ''}"></i>
    </button>

    {#if isOpen}
        <div class="model-dropdown">
            <div class="dropdown-content">
                {#if config?.available_models}
                    {#if hasAnyModels()}
                        {#each Object.keys(config.available_models) as provider}
                            {#if hasEnabledModels(provider)}
                                <div class="provider-section">
                                    <div
                                        class="provider-header {isProviderConfigured(
                                            provider,
                                        )
                                            ? 'configured'
                                            : 'not-configured'}"
                                    >
                                        <span class="provider-icon">
                                            {getProviderIcon(provider)}
                                        </span>
                                        <span class="provider-title"
                                            >{getProviderDisplayName(
                                                provider,
                                            )}</span
                                        >
                                        {#if !isProviderConfigured(provider)}
                                            <span class="not-configured-badge"
                                                >Not Configured</span
                                            >
                                        {/if}
                                    </div>

                                    {#if isProviderConfigured(provider)}
                                        <div class="models-list">
                                            {#each getEnabledModelsForProvider(provider) as model}
                                                <button
                                                    class="model-option {config.api_provider ===
                                                        provider &&
                                                    config.model === model.name
                                                        ? 'selected'
                                                        : ''}"
                                                    onclick={() =>
                                                        selectModel(
                                                            provider,
                                                            model.name,
                                                        )}
                                                    disabled={!isProviderConfigured(
                                                        provider,
                                                    )}
                                                >
                                                    <div class="model-details">
                                                        <span
                                                            class="model-display-name"
                                                            >{model.display_name}</span
                                                        >
                                                        {#if model.description}
                                                            <span
                                                                class="model-description"
                                                                >{model.description}</span
                                                            >
                                                        {/if}
                                                    </div>
                                                    {#if config.api_provider === provider && config.model === model.name}
                                                        <i
                                                            class="bi bi-check-circle-fill selected-icon"
                                                        ></i>
                                                    {/if}
                                                </button>
                                            {/each}
                                        </div>
                                    {:else}
                                        <div class="provider-disabled-message">
                                            Configure this provider in settings
                                            to use its models.
                                        </div>
                                    {/if}
                                </div>
                            {/if}
                        {/each}
                    {:else}
                        <div class="no-models-message">
                            <div class="no-models-icon">
                                <i class="bi bi-cpu"></i>
                            </div>
                            <div class="no-models-text">
                                <strong>No models configured</strong>
                                <p>
                                    Please add models in Settings → Model
                                    Management to get started.
                                </p>
                            </div>
                        </div>
                    {/if}
                {:else}
                    <div class="loading-state">Loading models...</div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<!-- Click outside to close -->
{#if isOpen}
    <div
        class="model-selector-overlay"
        onclick={() => (isOpen = false)}
        onkeydown={(e) => {
            if (e.key === "Escape") isOpen = false;
        }}
        role="button"
        tabindex="-1"
        aria-label="Close model selector"
    ></div>
{/if}

<style>
    .model-selector {
        position: relative;
        z-index: 1000;
    }

    .model-selector-button {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 8px 12px;
        background: var(--bg-secondary, #f8f9fa);
        border: 1px solid var(--border-color, #e0e0e0);
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        transition: all 0.2s ease;
        min-width: 200px;
        font-family: inherit;
    }

    .model-selector-button:hover {
        background: var(--bg-hover, #e9ecef);
        border-color: var(--border-hover, #379df1);
    }

    .model-selector-button.open {
        border-color: var(--primary-color, #379df1);
        background: var(--bg-active, #ffffff);
    }

    .model-info {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 2px;
    }

    .provider-name {
        font-size: 12px;
        color: var(--text-muted, #6c757d);
        font-weight: 500;
    }

    .model-name {
        font-size: 14px;
        color: var(--text-primary, #333);
        font-weight: 600;
    }

    .bi-chevron-up {
        transition: transform 0.2s ease;
        font-size: 12px;
        color: var(--text-muted, #6c757d);
    }

    .bi-chevron-up.rotated {
        transform: rotate(180deg);
    }
    .model-dropdown {
        position: absolute;
        bottom: 100%;
        left: 0;
        right: 0;
        margin-bottom: 4px;
        background: var(--bg-primary, #ffffff);
        border: 1px solid var(--border-color, #e0e0e0);
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        max-height: 300px;
        overflow-y: auto;
        z-index: 1001;
    }

    .dropdown-content {
        padding: 8px 0;
    }

    .provider-section {
        margin-bottom: 8px;
    }

    .provider-section:last-child {
        margin-bottom: 0;
    }
    .provider-header {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 6px 12px;
        font-weight: 600;
        font-size: 11px;
        color: var(--text-primary, #333);
        background: var(--bg-muted, #f8f9fa);
        margin: 0 8px;
        border-radius: 4px;
    }

    .provider-header.not-configured {
        color: var(--text-muted, #6c757d);
        background: var(--bg-disabled, #f5f5f5);
    }
    .provider-title {
        flex: 1;
    }
    .provider-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }
    .not-configured-badge {
        font-size: 9px;
        background: var(--warning-color, #ffc107);
        color: var(--warning-text, #856404);
        padding: 2px 6px;
        border-radius: 3px;
        font-weight: 500;
    }

    .models-list {
        padding: 4px 0;
    }
    .model-option {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        padding: 6px 16px;
        border: none;
        background: transparent;
        cursor: pointer;
        transition: background-color 0.2s ease;
        font-family: inherit;
        text-align: left;
    }

    .model-option:hover:not(:disabled) {
        background: var(--bg-hover, #f0f8ff);
    }

    .model-option.selected {
        background: var(--primary-light, #e3f2fd);
        color: var(--primary-color, #379df1);
    }

    .model-option:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .model-details {
        display: flex;
        flex-direction: column;
        gap: 2px;
        flex: 1;
    }
    .model-display-name {
        font-size: 12px;
        font-weight: 500;
        color: var(--text-primary, #333);
    }

    .model-option.selected .model-display-name {
        color: var(--primary-color, #379df1);
    }
    .model-description {
        font-size: 10px;
        color: var(--text-muted, #6c757d);
        line-height: 1.3;
    }

    .selected-icon {
        color: var(--primary-color, #379df1);
        font-size: 16px;
    }

    .provider-disabled-message {
        padding: 8px 16px;
        font-size: 12px;
        color: var(--text-muted, #6c757d);
        font-style: italic;
    }

    .loading-state {
        padding: 16px;
        text-align: center;
        color: #666;
        font-size: 13px;
    }

    .no-models-message {
        padding: 20px;
        text-align: center;
        color: #666;
    }

    .no-models-icon {
        font-size: 24px;
        margin-bottom: 8px;
        color: #999;
    }

    .no-models-text strong {
        display: block;
        margin-bottom: 4px;
        color: #333;
        font-size: 14px;
    }

    .no-models-text p {
        margin: 0;
        font-size: 12px;
        line-height: 1.4;
    }

    .model-selector-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        z-index: 999;
        background: transparent;
    }

    /* Dark theme support */
    @media (prefers-color-scheme: dark) {
        .model-selector-button {
            --bg-secondary: #2d3748;
            --border-color: #4a5568;
            --text-primary: #e2e8f0;
            --text-muted: #a0aec0;
        }

        .model-dropdown {
            --bg-primary: #2d3748;
            --border-color: #4a5568;
        }

        .provider-header {
            --bg-muted: #1a202c;
            --text-primary: #e2e8f0;
        }

        .provider-header.not-configured {
            --text-muted: #718096;
            --bg-disabled: #2d3748;
        }

        .model-option.selected {
            --primary-light: #2b6cb0;
            --primary-color: #63b3ed;
        }

        .model-display-name {
            --text-primary: #e2e8f0;
        }

        .model-description {
            --text-muted: #a0aec0;
        }

        .no-models-text strong {
            color: #f6f6f6;
        }

        .no-models-message {
            color: #aaa;
        }
    }

    /* Manual theme classes */
    :global(.theme-dark) .model-selector-button {
        --bg-secondary: #2d3748;
        --border-color: #4a5568;
        --text-primary: #e2e8f0;
        --text-muted: #a0aec0;
    }

    :global(.theme-dark) .model-dropdown {
        --bg-primary: #2d3748;
        --border-color: #4a5568;
    }

    :global(.theme-dark) .provider-header {
        --bg-muted: #1a202c;
        --text-primary: #e2e8f0;
    }

    :global(.theme-dark) .provider-header.not-configured {
        --text-muted: #718096;
        --bg-disabled: #2d3748;
    }

    :global(.theme-dark) .model-option.selected {
        --primary-light: #2b6cb0;
        --primary-color: #63b3ed;
    }

    :global(.theme-dark) .model-display-name {
        --text-primary: #e2e8f0;
    }

    :global(.theme-dark) .model-description {
        --text-muted: #a0aec0;
    }

    :global(.theme-light) .model-selector-button {
        --bg-secondary: #f8f9fa;
        --border-color: #e0e0e0;
        --text-primary: #333;
        --text-muted: #6c757d;
    }

    :global(.theme-light) .model-dropdown {
        --bg-primary: #ffffff;
        --border-color: #e0e0e0;
    }

    :global(.theme-light) .provider-header {
        --bg-muted: #f8f9fa;
        --text-primary: #333;
    }

    :global(.theme-light) .model-option.selected {
        --primary-light: #e3f2fd;
        --primary-color: #379df1;
    }
</style>
