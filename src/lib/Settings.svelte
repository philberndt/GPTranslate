<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import AppIcon from "./AppIcon.svelte";
    import TabNavigation from "./TabNavigation.svelte";
    import ApiConfiguration from "./ApiConfiguration.svelte";
    import ModelManagement from "./ModelManagement.svelte";
    import AppBehavior from "./AppBehavior.svelte";
    import AboutTab from "./AboutTab.svelte";
    import SettingsFooter from "./SettingsFooter.svelte";
    import pkg from "../../package.json";
    const version = pkg.version; // Tab management
    let activeTab = $state("api");

    // Model Config interface
    interface ModelConfig {
        name: string;
        display_name: string;
        provider: string;
        is_enabled: boolean;
        description?: string;
    }
    let config = $state({
        api_provider: "",
        openai_api_key: "",
        azure_endpoint: "",
        azure_api_key: "",
        azure_api_version: "",
        azure_deployment_name: "",
        ollama_url: "http://localhost:11434",
        model: "",
        available_models: {} as Record<string, ModelConfig[]>,
        target_language: "English",
        alternative_target_language: "Norwegian",
        auto_start: true,
        hotkey: "Ctrl+Alt+C",
        theme: "auto",
        minimize_to_tray: true,
        custom_prompt: "",
    });

    let isValidatingApiKey = $state(false);
    let apiKeyValid = $state<boolean | null>(null);
    let isSaving = $state(false);
    let saveMessage = $state("");
    let azureEndpointInfo = $state<{
        isValid: boolean;
        type?: string;
        deploymentDetected?: string;
        apiVersionDetected?: string;
    } | null>(null);
    interface Props {
        config: any;
        onClose: () => void;
        onConfigChange: () => void;
    }
    let { config: configProp, onClose, onConfigChange }: Props = $props();
    onMount(async () => {
        console.log("Settings component mounted");
        // Use the config prop if available, otherwise load from backend
        if (configProp) {
            config = { ...configProp };
            // Ensure available_models exists and has default structure
            if (!config.available_models) {
                config.available_models = {};
            }
        } else {
            try {
                config = (await invoke("get_config")) as any;
                // Ensure available_models exists and has default structure
                if (!config.available_models) {
                    config.available_models = {};
                }
            } catch (e) {
                console.error("Failed to load config:", e);
            }
        }
    });

    // Configuration change handler
    function handleConfigChange(updates: any) {
        config = { ...config, ...updates };
    } // Model management functions
    async function handleModelAdd(provider: string, model: ModelConfig) {
        if (!config.available_models[provider]) {
            config.available_models[provider] = [];
        }
        config.available_models[provider].push(model);
        // Auto-save and notify parent about config changes
        await saveModelChanges();
    }

    async function handleModelRemove(provider: string, modelIndex: number) {
        if (config.available_models[provider]) {
            config.available_models[provider].splice(modelIndex, 1);
        }
        // Auto-save and notify parent about config changes
        await saveModelChanges();
    }

    async function handleModelToggle(provider: string, modelIndex: number) {
        if (
            config.available_models[provider] &&
            config.available_models[provider][modelIndex]
        ) {
            config.available_models[provider][modelIndex].is_enabled =
                !config.available_models[provider][modelIndex].is_enabled;
        }
        // Auto-save and notify parent about config changes
        await saveModelChanges();
    }

    // Helper function to save model changes without showing UI feedback
    async function saveModelChanges() {
        try {
            await invoke("save_config", { newConfig: config });
            // Notify parent component about config changes
            onConfigChange();
        } catch (e) {
            console.error("Failed to save model changes:", e);
        }
    }
    async function onApiProviderChange() {
        // Clear validation when changing providers
        apiKeyValid = null;
    }

    function parseAzureEndpoint(url: string): {
        baseUrl: string;
        apiVersion: string;
        deploymentName: string | null;
        isModelsEndpoint: boolean;
        isValid: boolean;
        errorMessage?: string;
    } {
        try {
            const parsedUrl = new URL(url);

            // Extract API version from query parameters
            const apiVersion = parsedUrl.searchParams.get("api-version");

            // Determine endpoint type based on hostname
            const isModelsEndpoint = parsedUrl.hostname.includes(
                "services.ai.azure.com",
            );
            const isCognitiveServicesEndpoint = parsedUrl.hostname.includes(
                "cognitiveservices.azure.com",
            );

            if (!isModelsEndpoint && !isCognitiveServicesEndpoint) {
                return {
                    baseUrl: url,
                    apiVersion: config.azure_api_version,
                    deploymentName: null,
                    isModelsEndpoint: false,
                    isValid: false,
                    errorMessage:
                        "URL must be either a cognitiveservices.azure.com or services.ai.azure.com endpoint",
                };
            }

            // Extract deployment name for cognitive services endpoints
            let deploymentName: string | null = null;
            if (isCognitiveServicesEndpoint) {
                const pathParts = parsedUrl.pathname
                    .split("/")
                    .filter((part) => part.length > 0);
                const deploymentIndex = pathParts.findIndex(
                    (part) => part === "deployments",
                );

                if (
                    deploymentIndex !== -1 &&
                    deploymentIndex + 1 < pathParts.length
                ) {
                    deploymentName = pathParts[deploymentIndex + 1];
                } else {
                    // If no deployment in path, we can still extract from a full endpoint URL
                    console.log(
                        "No deployment found in path, cognitive services endpoint may need deployment name",
                    );
                }
            }

            // Create base URL (without path and query parameters)
            const baseUrl = `${parsedUrl.protocol}//${parsedUrl.host}`;

            return {
                baseUrl,
                apiVersion: apiVersion || config.azure_api_version,
                deploymentName,
                isModelsEndpoint,
                isValid: true,
            };
        } catch (e) {
            console.error("Error parsing Azure endpoint URL:", e);
            return {
                baseUrl: url,
                apiVersion: config.azure_api_version,
                deploymentName: null,
                isModelsEndpoint: false,
                isValid: false,
                errorMessage: `Invalid URL format: ${e instanceof Error ? e.message : "Unknown error"}`,
            };
        }
    }

    async function onAzureEndpointChange() {
        azureEndpointInfo = null; // Reset info

        if (config.azure_endpoint) {
            try {
                const {
                    baseUrl,
                    apiVersion,
                    deploymentName,
                    isModelsEndpoint,
                    isValid,
                    errorMessage,
                } = parseAzureEndpoint(config.azure_endpoint);

                if (!isValid) {
                    console.warn(`Invalid Azure endpoint: ${errorMessage}`);
                    azureEndpointInfo = { isValid: false };
                    return;
                }

                // Update the config with parsed values
                config.azure_endpoint = baseUrl;

                if (apiVersion) {
                    config.azure_api_version = apiVersion;
                    console.log(`✓ Extracted API version: ${apiVersion}`);
                }

                if (isModelsEndpoint) {
                    // For models endpoints, clear deployment name as it's not needed
                    config.azure_deployment_name = "";
                    console.log(
                        "✓ Models API endpoint detected - deployment name cleared",
                    );
                } else if (deploymentName) {
                    // For cognitive services endpoints, use extracted deployment name
                    config.azure_deployment_name = deploymentName;
                    console.log(
                        `✓ Extracted deployment name: ${deploymentName}`,
                    );

                    // Automatically add the model if it doesn't exist
                    await autoAddAzureModel(deploymentName);
                } else {
                    // Cognitive services endpoint but no deployment name in URL
                    console.log(
                        "⚠ Cognitive Services endpoint detected but no deployment name found in URL. You may need to specify it manually.",
                    );
                }

                // Set endpoint info for UI feedback
                azureEndpointInfo = {
                    isValid: true,
                    type: isModelsEndpoint
                        ? "Models API"
                        : "Cognitive Services",
                    deploymentDetected: deploymentName || undefined,
                    apiVersionDetected: apiVersion || undefined,
                };

                console.log(
                    `✓ Endpoint type: ${isModelsEndpoint ? "Models API" : "Cognitive Services"}`,
                );
                console.log(`✓ Base URL set to: ${baseUrl}`);
            } catch (e) {
                console.error("Failed to parse Azure endpoint:", e);
                azureEndpointInfo = { isValid: false };
            }
        }
        // Reset API key validation when endpoint changes
        apiKeyValid = null;
    }

    async function validateApiKey() {
        if (
            (config.api_provider === "openai" && !config.openai_api_key) ||
            (config.api_provider === "azure_openai" &&
                (!config.azure_api_key || !config.azure_endpoint)) ||
            (config.api_provider === "ollama" && !config.ollama_url)
        ) {
            return;
        }

        isValidatingApiKey = true;
        try {
            const isValid = (await invoke("validate_api_key", {
                apiProvider: config.api_provider,
                apiKey:
                    config.api_provider === "openai"
                        ? config.openai_api_key
                        : config.api_provider === "azure_openai"
                          ? config.azure_api_key
                          : "", // Ollama doesn't need an API key
                endpoint:
                    config.api_provider === "azure_openai"
                        ? config.azure_endpoint
                        : config.api_provider === "ollama"
                          ? config.ollama_url
                          : null,
                apiVersion:
                    config.api_provider === "azure_openai"
                        ? config.azure_api_version
                        : null,
            })) as boolean;
            apiKeyValid = isValid;
        } catch (e) {
            console.error("API key validation failed:", e);
            apiKeyValid = false;
        } finally {
            isValidatingApiKey = false;
        }
    }
    async function saveSettings() {
        isSaving = true;
        saveMessage = "";
        try {
            await invoke("save_config", { newConfig: config });
            saveMessage =
                "Settings saved successfully! Hotkey changes take effect immediately.";
            // Notify parent component about config changes
            onConfigChange();
            setTimeout(() => {
                saveMessage = "";
            }, 5000);
        } catch (e) {
            console.error("Failed to save settings:", e);
            saveMessage = "Failed to save settings: " + e;
        } finally {
            isSaving = false;
        }
    }

    function resetToDefaults() {
        if (
            confirm("Are you sure you want to reset all settings to defaults?")
        ) {
            config = {
                api_provider: "",
                openai_api_key: "",
                azure_endpoint: "",
                azure_api_key: "",
                azure_api_version: "",
                azure_deployment_name: "",
                ollama_url: "http://localhost:11434",
                model: "",
                available_models: {} as Record<string, ModelConfig[]>,
                target_language: "English",
                alternative_target_language: "Norwegian",
                auto_start: true,
                hotkey: "Ctrl+Alt+C",
                theme: "auto",
                minimize_to_tray: true,
                custom_prompt:
                    "Translate the given text from {detected_language} to {target_language} accurately while preserving the meaning, tone, and nuance of the original content.\n\n# Additional Details\n- Ensure the translation retains the context, cultural meaning, tone, formal/informal style, and any idiomatic expressions.\n- Do **not** alter names, technical terms, or specific formatting unless required for grammatical correctness in the target language.\n- If the detected language is the same as the target language, choose the most appropriate alternative language for translation.\n\n# Output Format\nThe translation output should be provided as valid JSON containing 'detected_language' and 'translated_text' fields.\n\n# Notes\n- Ensure punctuation and capitalization match the norms of the target language.\n- When encountering idiomatic expressions, adapt them to equivalent phrases in the target language rather than direct word-for-word translation.\n- For ambiguous content, aim for the most contextually appropriate meaning.\n- Take into consideration the whole text and what it is about.",
            };
            apiKeyValid = null;
        }
    }

    // Helper function to automatically add Azure model when extracted from endpoint
    async function autoAddAzureModel(deploymentName: string) {
        try {
            // Check if the model already exists
            if (!config.available_models.azure_openai) {
                config.available_models.azure_openai = [];
            }

            const existingModel = config.available_models.azure_openai.find(
                (model) => model.name === deploymentName,
            );

            if (!existingModel) {
                const newModel: ModelConfig = {
                    name: deploymentName,
                    display_name: deploymentName,
                    provider: "azure_openai",
                    is_enabled: true,
                    description: "Auto-added from endpoint URL",
                };

                config.available_models.azure_openai.push(newModel);
                console.log(`✓ Auto-added Azure model: ${deploymentName}`);

                // Auto-save the model addition
                await saveModelChanges();
            }
        } catch (e) {
            console.error("Failed to auto-add Azure model:", e);
        }
    }

    // ...existing code...
</script>

<div class="settings-overlay">
    <div class="settings-container">
        <div class="settings-header">
            <div class="settings-header-content">
                <AppIcon size={48} className="settings-logo" />
                <h1 class="settings-title">GPTranslate</h1>
            </div>
            <button
                class="close-btn"
                onclick={onClose}
                title="Close settings"
                aria-label="Close settings"
            >
                <i class="bi bi-x-lg"></i>
            </button>
        </div>

        <!-- Tab Navigation -->
        <TabNavigation {activeTab} onTabChange={(tab) => (activeTab = tab)} />
        <div class="settings-content">
            {#if activeTab === "api"}
                <!-- API Configuration Tab -->
                <ApiConfiguration
                    {config}
                    {isValidatingApiKey}
                    {apiKeyValid}
                    {azureEndpointInfo}
                    onConfigChange={handleConfigChange}
                    {onApiProviderChange}
                    {onAzureEndpointChange}
                    {validateApiKey}
                />
            {:else if activeTab === "models"}
                <!-- Model Management Tab -->
                <ModelManagement
                    {config}
                    availableModels={config.available_models}
                    onConfigChange={handleConfigChange}
                    onModelAdd={handleModelAdd}
                    onModelRemove={handleModelRemove}
                    onModelToggle={handleModelToggle}
                />
            {:else if activeTab === "behavior"}
                <!-- App Behavior -->
                <AppBehavior {config} onConfigChange={handleConfigChange} />
            {:else if activeTab === "about"}
                <!-- About -->
                <AboutTab {version} />
            {/if}
        </div>

        <SettingsFooter
            {isSaving}
            {saveMessage}
            onSave={saveSettings}
            onReset={resetToDefaults}
        />
    </div>
</div>

<style>
    * {
        box-sizing: border-box;
    }

    .settings-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        padding: 20px;
    }

    .settings-container {
        background: white;
        border-radius: 12px;
        width: 100%;
        max-width: 600px;
        max-height: 90vh;
        display: flex;
        flex-direction: column;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
        overflow: hidden;
    }

    .settings-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 24px 24px 20px 24px;
        border-bottom: 1px solid #e0e0e0;
        flex-shrink: 0;
    }

    .settings-header-content {
        display: flex;
        align-items: center;
        justify-content: center;
        flex: 1;
        gap: 12px;
    }

    .settings-title {
        margin: 0;
        font-size: 1.8rem;
        font-weight: 600;
        color: #333;
        letter-spacing: -0.02em;
    }

    :global(.settings-logo) {
        margin-right: 0 !important;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }

    .close-btn {
        background: none;
        border: none;
        font-size: 1.2rem;
        color: #666;
        cursor: pointer;
        padding: 8px;
        border-radius: 6px;
        transition: all 0.2s;
    }

    .close-btn:hover {
        background: #f0f0f0;
        color: #333;
    }

    .settings-content {
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
        padding: 0 24px;
        box-sizing: border-box;
    }

    /* Dark mode */
    @media (prefers-color-scheme: dark) {
        .settings-container {
            background: #2d2d2d;
            color: #f6f6f6;
        }

        .settings-header {
            border-color: #444;
        }

        .settings-title {
            color: #f6f6f6;
        }

        .close-btn {
            color: #ccc;
        }

        .close-btn:hover {
            background: #444;
            color: #f6f6f6;
        }
    }

    @media (max-width: 768px) {
        .settings-overlay {
            padding: 10px;
        }

        .settings-container {
            max-height: 95vh;
        }
    }
</style>
