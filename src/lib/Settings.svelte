<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import AppIcon from "./AppIcon.svelte";
    import pkg from "../../package.json";
    const version = pkg.version;
    let config = $state({
        api_provider: "openai",
        openai_api_key: "",
        azure_endpoint: "",
        azure_api_key: "",
        azure_api_version: "2025-01-01-preview",
        azure_deployment_name: "gpt-4.1-nano",
        ollama_url: "http://localhost:11434",
        model: "gpt-4.1-nano",
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
        onClose: () => void;
    }
    let { onClose }: Props = $props();
    onMount(async () => {
        console.log("Settings component mounted");
        try {
            config = (await invoke("get_config")) as any;
        } catch (e) {
            console.error("Failed to load config:", e);
        }
    });
    async function onApiProviderChange() {
        // Set default model values based on provider
        if (config.api_provider === "openai" && !config.model) {
            config.model = "gpt-4.1-nano";
        } else if (
            config.api_provider === "azure_openai" &&
            !config.azure_deployment_name
        ) {
            config.azure_deployment_name = "gpt-4.1-nano";
        } else if (config.api_provider === "ollama" && !config.model) {
            config.model = "llama3.2:latest"; // Common Ollama model
        }
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
    function onAzureEndpointChange() {
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
                api_provider: "openai",
                openai_api_key: "",
                azure_endpoint: "",
                azure_api_key: "",
                azure_api_version: "2025-01-01-preview",
                azure_deployment_name: "gpt-4.1-nano",
                ollama_url: "http://localhost:11434",
                model: "gpt-4.1-nano",
                target_language: "English",
                alternative_target_language: "Norwegian",
                auto_start: true,
                hotkey: "CommandOrControl+Alt+C",
                theme: "auto",
                minimize_to_tray: true,
                custom_prompt:
                    "Translate the given text from {detected_language} to {target_language} accurately while preserving the meaning, tone, and nuance of the original content.\n\n# Additional Details\n- Ensure the translation retains the context, cultural meaning, tone, formal/informal style, and any idiomatic expressions.\n- Do **not** alter names, technical terms, or specific formatting unless required for grammatical correctness in the target language.\n- If the detected language is the same as the target language, choose the most appropriate alternative language for translation.\n\n# Output Format\nThe translation output should be provided as valid JSON containing 'detected_language' and 'translated_text' fields.\n\n# Notes\n- Ensure punctuation and capitalization match the norms of the target language.\n- When encountering idiomatic expressions, adapt them to equivalent phrases in the target language rather than direct word-for-word translation.\n- For ambiguous content, aim for the most contextually appropriate meaning.\n- Take into consideration the whole text and what it is about.",
            };
            apiKeyValid = null;
        }
    }
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

        <div class="settings-content">
            <!-- API Configuration -->
            <section class="settings-section">
                <h3><i class="bi bi-cloud"></i>API Configuration</h3>
                <div class="form-group">
                    <label for="api-provider">API Provider</label>
                    <div class="provider-grid">
                        <button
                            type="button"
                            class="provider-card {config.api_provider ===
                            'openai'
                                ? 'active'
                                : ''}"
                            onclick={() => {
                                config.api_provider = "openai";
                                onApiProviderChange();
                            }}
                        >
                            <div class="provider-logo">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 16 16"
                                    fill="currentColor"
                                >
                                    <path
                                        d="M14.949 6.547a3.94 3.94 0 0 0-.348-3.273 4.11 4.11 0 0 0-4.4-1.934A4.1 4.1 0 0 0 8.423.2 4.15 4.15 0 0 0 6.305.086a4.1 4.1 0 0 0-1.891.948 4.04 4.04 0 0 0-1.158 1.753 4.1 4.1 0 0 0-1.563.679A4 4 0 0 0 .554 4.72a3.99 3.99 0 0 0 .502 4.731 3.94 3.94 0 0 0 .346 3.274 4.11 4.11 0 0 0 4.402 1.933c.382.425.852.764 1.377.995.526.231 1.095.35 1.67.346 1.78.002 3.358-1.132 3.901-2.804a4.1 4.1 0 0 0 1.563-.68 4 4 0 0 0 1.14-1.253 3.99 3.99 0 0 0-.506-4.716m-6.097 8.406a3.05 3.05 0 0 1-1.945-.694l.096-.054 3.23-1.838a.53.53 0 0 0 .265-.455v-4.49l1.366.778q.02.011.025.035v3.722c-.003 1.653-1.361 2.992-3.037 2.996m-6.53-2.75a2.95 2.95 0 0 1-.36-2.01l.095.057L5.29 12.09a.53.53 0 0 0 .527 0l3.949-2.246v1.555a.05.05 0 0 1-.022.041L6.473 13.3c-1.454.826-3.311.335-4.15-1.098m-.85-6.94A3.02 3.02 0 0 1 3.07 3.949v3.785a.51.51 0 0 0 .262.451l3.93 2.237-1.366.779a.05.05 0 0 1-.048 0L2.585 9.342a2.98 2.98 0 0 1-1.113-4.094zm11.216 2.571L8.747 5.576l1.362-.776a.05.05 0 0 1 .048 0l3.265 1.86a3 3 0 0 1 1.173 1.207 2.96 2.96 0 0 1-.27 3.2 3.05 3.05 0 0 1-1.36.997V8.279a.52.52 0 0 0-.276-.445m1.36-2.015-.097-.057-3.226-1.855a.53.53 0 0 0-.53 0L6.249 6.153V4.598a.04.04 0 0 1 .019-.04L9.533 2.7a3.07 3.07 0 0 1 3.257.139c.474.325.843.778 1.066 1.303.223.526.289 1.103.191 1.664zM5.503 8.575 4.139 7.8a.05.05 0 0 1-.026-.037V4.049c0-.57.166-1.127.476-1.607s.752-.864 1.275-1.105a3.08 3.08 0 0 1 3.234.41l-.096.054-3.23 1.838a.53.53 0 0 0-.265.455zm.742-1.577 1.758-1 1.762 1v2l-1.755 1-1.762-1z"
                                    />
                                </svg>
                            </div>
                            <div class="provider-name">OpenAI</div>
                        </button>

                        <button
                            type="button"
                            class="provider-card {config.api_provider ===
                            'azure_openai'
                                ? 'active'
                                : ''}"
                            onclick={() => {
                                config.api_provider = "azure_openai";
                                onApiProviderChange();
                            }}
                        >
                            <div class="provider-logo">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 16 16"
                                    fill="currentColor"
                                >
                                    <path
                                        d="M7.462 0H0v7.19h7.462zM16 0H8.538v7.19H16zM7.462 8.211H0V16h7.462zm8.538 0H8.538V16H16z"
                                    />
                                </svg>
                            </div>
                            <div class="provider-name">Azure OpenAI</div>
                        </button>

                        <button
                            type="button"
                            class="provider-card {config.api_provider ===
                            'ollama'
                                ? 'active'
                                : ''}"
                            onclick={() => {
                                config.api_provider = "ollama";
                                onApiProviderChange();
                            }}
                        >
                            <div class="provider-logo">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                    fill-rule="evenodd"
                                >
                                    <path
                                        d="M7.905 1.09c.216.085.411.225.588.41.295.306.544.744.734 1.263.191.522.315 1.1.362 1.68a5.054 5.054 0 0 1 2.049-.636l.051-.004c.87-.07 1.73.087 2.48.474.101.053.2.11.297.17.05-.569.172-1.134.36-1.644.19-.52.439-.957.733-1.264a1.67 1.67 0 0 1 .589-.41c.257-.1.53-.118.796-.042.401.114.745.368 1.016.737.248.337.434.769.561 1.287.23.934.27 2.163.115 3.645l.053.04.026.019c.757.576 1.284 1.397 1.563 2.35.435 1.487.216 3.155-.534 4.088l-.018.021.002.003c.417.762.67 1.567.724 2.4l.002.03c.064 1.065-.2 2.137-.814 3.19l-.007.01.01.024c.472 1.157.62 2.322.438 3.486l-.006.039a.651.651 0 0 1-.747.536.648.648 0 0 1-.54-.742c.167-1.033.01-2.069-.48-3.123a.643.643 0 0 1 .04-.617l.004-.006c.604-.924.854-1.83.8-2.72-.046-.779-.325-1.544-.8-2.273a.644.644 0 0 1 .18-.886l.009-.006c.243-.159.467-.565.58-1.12a4.229 4.229 0 0 0-.095-1.974c-.205-.7-.58-1.284-1.105-1.683-.595-.454-1.383-.673-2.38-.61a.653.653 0 0 1-.632-.371c-.314-.665-.772-1.141-1.343-1.436a3.288 3.288 0 0 0-1.772-.332c-1.245.099-2.343.801-2.67 1.686a.652.652 0 0 1-.61.425c-1.067.002-1.893.252-2.497.703-.522.39-.878.935-1.066 1.588a4.07 4.07 0 0 0-.068 1.886c.112.558.331 1.02.582 1.269l.008.007c.212.207.257.53.109.785-.36.622-.629 1.549-.673 2.44-.05 1.018.186 1.902.719 2.536l.016.019a.643.643 0 0 1 .095.69c-.576 1.236-.753 2.252-.562 3.052a.652.652 0 0 1-1.269.298c-.243-1.018-.078-2.184.473-3.498l.014-.035-.008-.012a4.339 4.339 0 0 1-.598-1.309l-.005-.019a5.764 5.764 0 0 1-.177-1.785c.044-.91.278-1.842.622-2.59l.012-.026-.002-.002c-.293-.418-.51-.953-.63-1.545l-.005-.024a5.352 5.352 0 0 1 .093-2.49c.262-.915.777-1.701 1.536-2.269.06-.045.123-.09.186-.132-.159-1.493-.119-2.73.112-3.67.127-.518.314-.95.562-1.287.27-.368.614-.622 1.015-.737.266-.076.54-.059.797.042zm4.116 9.09c.936 0 1.8.313 2.446.855.63.527 1.005 1.235 1.005 1.94 0 .888-.406 1.58-1.133 2.022-.62.375-1.451.557-2.403.557-1.009 0-1.871-.259-2.493-.734-.617-.47-.963-1.13-.963-1.845 0-.707.398-1.417 1.056-1.946.668-.537 1.55-.849 2.485-.849zm0 .896a3.07 3.07 0 0 0-1.916.65c-.461.37-.722.835-.722 1.25 0 .428.21.829.61 1.134.455.347 1.124.548 1.943.548.799 0 1.473-.147 1.932-.426.463-.28.7-.686.7-1.257 0-.423-.246-.89-.683-1.256-.484-.405-1.14-.643-1.864-.643zm.662 1.21.004.004c.12.151.095.37-.056.49l-.292.23v.446a.375.375 0 0 1-.376.373.375.375 0 0 1-.376-.373v-.46l-.271-.218a.347.347 0 0 1-.052-.49.353.353 0 0 1 .494-.051l.215.172.22-.174a.353.353 0 0 1 .49.051zm-5.04-1.919c.478 0 .867.39.867.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zm8.706 0c.48 0 .868.39.868.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zM7.44 2.3l-.003.002a.659.659 0 0 0-.285.238l-.005.006c-.138.189-.258.467-.348.832-.17.692-.216 1.631-.124 2.782.43-.128.899-.208 1.404-.237l.01-.001.019-.034c.046-.082.095-.161.148-.239.123-.771.022-1.692-.253-2.444-.134-.364-.297-.65-.453-.813a.628.628 0 0 0-.107-.09L7.44 2.3zm9.174.04-.002.001a.628.628 0 0 0-.107.09c-.156.163-.32.45-.453.814-.29.794-.387 1.776-.23 2.572l.058.097.008.014h.03a5.184 5.184 0 0 1 1.466.212c.086-1.124.038-2.043-.128-2.722-.09-.365-.21-.643-.349-.832l-.004-.006a.659.659 0 0 0-.285-.239h-.004z"
                                    />
                                </svg>
                            </div>
                            <div class="provider-name">Ollama</div>
                        </button>
                    </div>
                </div>

                {#if config.api_provider === "openai"}
                    <div class="form-group">
                        <label for="openai-key">OpenAI API Key</label>
                        <div class="api-key-group">
                            <input
                                id="openai-key"
                                type="password"
                                bind:value={config.openai_api_key}
                                placeholder="sk-..."
                                onblur={validateApiKey}
                            />
                            {#if isValidatingApiKey}
                                <span class="validation-icon validating">
                                    <i class="bi bi-arrow-clockwise"></i>
                                </span>
                            {:else if apiKeyValid === true}
                                <span class="validation-icon valid">
                                    <i class="bi bi-check-circle-fill"></i>
                                </span>
                            {:else if apiKeyValid === false}
                                <span class="validation-icon invalid">
                                    <i class="bi bi-x-circle-fill"></i>
                                </span>
                            {/if}
                        </div>
                    </div>
                {:else if config.api_provider === "azure_openai"}
                    <div class="form-group">
                        <label for="azure-endpoint">Azure OpenAI Endpoint</label
                        >
                        <input
                            id="azure-endpoint"
                            type="url"
                            bind:value={config.azure_endpoint}
                            placeholder="Paste your full Azure OpenAI endpoint URL here..."
                            onblur={validateApiKey}
                            oninput={onAzureEndpointChange}
                        />
                        <small>
                            Paste the complete endpoint URL from Azure portal.
                            Supported formats:
                            <br />
                            •
                            <code
                                >https://resource.cognitiveservices.azure.com/openai/...</code
                            >
                            <br />
                            •
                            <code
                                >https://resource.services.ai.azure.com/models/...</code
                            >
                            <br />
                            The app will automatically extract the base URL, API
                            version, and deployment name.
                        </small>

                        {#if azureEndpointInfo?.isValid}
                            <div class="endpoint-info success">
                                <i class="bi bi-check-circle-fill"></i>
                                <strong>Auto-detected:</strong>
                                {azureEndpointInfo.type} endpoint
                                {#if azureEndpointInfo.deploymentDetected}
                                    • Deployment: <code
                                        >{azureEndpointInfo.deploymentDetected}</code
                                    >
                                {/if}
                                {#if azureEndpointInfo.apiVersionDetected}
                                    • API Version: <code
                                        >{azureEndpointInfo.apiVersionDetected}</code
                                    >
                                {/if}
                            </div>
                        {:else if azureEndpointInfo?.isValid === false}
                            <div class="endpoint-info error">
                                <i class="bi bi-exclamation-triangle-fill"></i>
                                <strong>Invalid endpoint format.</strong> Please
                                use a valid Azure OpenAI endpoint URL.
                            </div>
                        {/if}
                    </div>

                    <div class="form-group">
                        <label for="azure-key">Azure API Key</label>
                        <div class="api-key-group">
                            <input
                                id="azure-key"
                                type="password"
                                bind:value={config.azure_api_key}
                                placeholder="Your Azure API key"
                                onblur={validateApiKey}
                            />
                            {#if isValidatingApiKey}
                                <span class="validation-icon validating">
                                    <i class="bi bi-arrow-clockwise"></i>
                                </span>
                            {:else if apiKeyValid === true}
                                <span class="validation-icon valid">
                                    <i class="bi bi-check-circle-fill"></i>
                                </span>
                            {:else if apiKeyValid === false}
                                <span class="validation-icon invalid">
                                    <i class="bi bi-x-circle-fill"></i>
                                </span>
                            {/if}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="azure-deployment"
                            >Azure Deployment Name</label
                        >
                        <input
                            id="azure-deployment"
                            type="text"
                            bind:value={config.azure_deployment_name}
                            placeholder="gpt-4"
                        />
                    </div>

                    <div class="form-group">
                        <label for="azure-api-version">Azure API Version</label>
                        <input
                            id="azure-api-version"
                            type="text"
                            bind:value={config.azure_api_version}
                            placeholder="2025-01-01-preview"
                        />
                        <small>
                            API version for Azure OpenAI requests (e.g.,
                            2025-01-01-preview)
                        </small>
                    </div>
                {:else if config.api_provider === "ollama"}
                    <div class="form-group">
                        <label for="ollama-url">Ollama Server URL</label>
                        <div class="api-key-group">
                            <input
                                id="ollama-url"
                                type="text"
                                bind:value={config.ollama_url}
                                placeholder="http://localhost:11434"
                                onblur={validateApiKey}
                            />
                            {#if isValidatingApiKey}
                                <span class="validation-icon validating">
                                    <i class="bi bi-arrow-clockwise"></i>
                                </span>
                            {:else if apiKeyValid === true}
                                <span class="validation-icon valid">
                                    <i class="bi bi-check-circle-fill"></i>
                                </span>
                            {:else if apiKeyValid === false}
                                <span class="validation-icon invalid">
                                    <i class="bi bi-x-circle-fill"></i>
                                </span>
                            {/if}
                        </div>
                        <small>
                            URL of your Ollama server. Make sure Ollama is
                            running locally or provide the remote server URL.
                        </small>
                    </div>
                {/if}
                {#if config.api_provider === "openai"}
                    <div class="form-group">
                        <label for="model">OpenAI Model</label>
                        <input
                            id="model"
                            type="text"
                            bind:value={config.model}
                            placeholder="gpt-4.1-nano"
                        />
                        <small>
                            Specify the OpenAI model to use (e.g., gpt-4.1-nano,
                            gpt-4o-mini, gpt-4, etc.)
                        </small>
                    </div>
                {:else if config.api_provider === "ollama"}
                    <div class="form-group">
                        <label for="ollama-model">Ollama Model</label>
                        <input
                            id="ollama-model"
                            type="text"
                            bind:value={config.model}
                            placeholder="llama3.2:latest"
                        />
                        <small>
                            Specify the Ollama model to use (e.g.,
                            llama3.2:latest, mistral:latest, codellama:latest).
                            Make sure the model is downloaded with 'ollama pull
                            model_name'
                        </small>
                    </div>
                {/if}
            </section>
            <!-- App Behavior -->
            <section class="settings-section">
                <h3><i class="bi bi-sliders"></i>App Behavior</h3>

                <div class="form-group">
                    <label for="target-language">Target Language</label>
                    <input
                        id="target-language"
                        type="text"
                        bind:value={config.target_language}
                        placeholder="English, Spanish, French, etc."
                    />
                    <small
                        >Specify the default language to translate to. This
                        language will be used in the custom prompt as
                        &#123;target_language&#125;.</small
                    >
                </div>

                <div class="form-group">
                    <label for="alternative-target-language"
                        >Alternative Target Language</label
                    >
                    <input
                        id="alternative-target-language"
                        type="text"
                        bind:value={config.alternative_target_language}
                        placeholder="Norwegian, Spanish, German, etc."
                    />
                    <small
                        >Language to use when the detected language is the same
                        as the target language. For example, if you normally
                        translate to English, but the input is already English,
                        it will translate to this alternative language instead.</small
                    >
                </div>

                <div class="form-group">
                    <label for="theme">Theme</label>
                    <select id="theme" bind:value={config.theme}>
                        <option value="auto">Auto (System)</option>
                        <option value="light">Light</option>
                        <option value="dark">Dark</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="hotkey">Global Hotkey</label>
                    <input
                        id="hotkey"
                        type="text"
                        bind:value={config.hotkey}
                        placeholder="CommandOrControl+Alt+C"
                    />
                    <small
                        >Example: CommandOrControl+Alt+C, Alt+Shift+T, etc.</small
                    >
                </div>

                <div class="checkbox-group">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            bind:checked={config.auto_start}
                        />
                        <span class="checkmark"></span>
                        Start with Windows
                    </label>
                </div>
                <div class="checkbox-group">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            bind:checked={config.minimize_to_tray}
                        />
                        <span class="checkmark"></span>
                        Minimize to system tray
                    </label>
                </div>
            </section>
            <!-- Custom Prompt -->
            <section class="settings-section">
                <h3>
                    <i class="bi bi-chat-text"></i>Custom Translation Prompt
                </h3>

                <div class="form-group">
                    <label for="custom-prompt">Translation Instructions</label>
                    <textarea
                        id="custom-prompt"
                        bind:value={config.custom_prompt}
                        placeholder="Enter custom instructions for the AI translator..."
                        class="prompt-textarea"
                        rows="8"
                    ></textarea>
                    <small>
                        Customize how the AI translates text. You can use these
                        variables in your prompt:
                        <br />
                        <code>&#123;detected_language&#125;</code> - The
                        automatically detected source language
                        <br />
                        <code>&#123;target_language&#125;</code> - Your configured
                        target language
                    </small>
                </div>
            </section>

            <!-- About -->
            <section class="settings-section">
                <h3><i class="bi bi-info-circle"></i>About</h3>

                <div class="about-content">
                    <div class="about-item">
                        <strong>Developer:</strong> Phil Berndt
                    </div>
                    <div class="about-item">
                        <strong>Email:</strong> phil@berndt.no
                    </div>
                    <div class="about-item">
                        <strong>Website:</strong>
                        <a
                            href="https://gptranslate.berndt.no"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            https://gptranslate.berndt.no
                        </a>
                    </div>
                    <div class="about-item">
                        <strong>Version:</strong>
                        {version}
                    </div>
                </div>
            </section>
        </div>

        <div class="settings-footer">
            {#if saveMessage}
                <div
                    class="save-message"
                    class:success={saveMessage.includes("successfully")}
                    class:error={!saveMessage.includes("successfully")}
                >
                    {saveMessage}
                </div>
            {/if}
            <div class="settings-actions">
                <button class="reset-btn" onclick={resetToDefaults}>
                    <i class="bi bi-arrow-counterclockwise"></i>Reset to
                    Defaults
                </button>
                <button
                    class="save-btn"
                    onclick={saveSettings}
                    disabled={isSaving}
                >
                    {#if isSaving}
                        <i class="bi bi-arrow-clockwise spinning"></i>Saving...
                    {:else}
                        <i class="bi bi-check-lg"></i>Save Settings
                    {/if}
                </button>
            </div>
        </div>
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
    .settings-section {
        margin: 24px 0;
        min-width: 0; /* Prevent flex item from overflowing */
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
    .form-group {
        margin-bottom: 16px;
        min-width: 0; /* Prevent flex item from overflowing */
    }
    .form-group label {
        display: block;
        font-weight: 500;
        margin-bottom: 4px;
        color: #333;
    }
    .form-group input,
    .form-group select {
        width: 100%;
        max-width: 100%;
        padding: 10px 12px;
        border: 1px solid #ddd;
        border-radius: 6px;
        font-family: inherit;
        font-size: 14px;
        transition: border-color 0.2s;
        box-sizing: border-box;
        min-width: 0; /* Prevent input from overflowing */
    }
    .form-group input:focus,
    .form-group select:focus {
        outline: none;
        border-color: #379df1;
        box-shadow: 0 0 0 2px rgba(55, 157, 241, 0.1);
    }
    .form-group small {
        color: #666;
        font-size: 12px;
        margin-top: 4px;
        display: block;
    }

    .form-group small code {
        background: #f1f3f4;
        color: #d73a49;
        padding: 2px 4px;
        border-radius: 3px;
        font-family: "Consolas", "Monaco", "Courier New", monospace;
        font-size: 11px;
    }
    .api-key-group {
        position: relative;
        display: flex;
        align-items: center;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
    }
    .api-key-group input {
        padding-right: 40px;
        flex: 1;
        min-width: 0; /* Prevent input from overflowing */
    }

    .validation-icon {
        position: absolute;
        right: 12px;
        font-size: 16px;
    }
    .validation-icon.validating {
        color: #379df1;
        animation: spin 1s linear infinite;
    }

    .validation-icon.valid {
        color: #28a745;
    }
    .validation-icon.invalid {
        color: #dc3545;
    }

    .endpoint-info {
        margin-top: 8px;
        padding: 8px 12px;
        border-radius: 6px;
        font-size: 13px;
        display: flex;
        align-items: flex-start;
        gap: 8px;
        border: 1px solid;
    }

    .endpoint-info.success {
        background: #d4edda;
        color: #155724;
        border-color: #c3e6cb;
    }

    .endpoint-info.error {
        background: #f8d7da;
        color: #721c24;
        border-color: #f5c6cb;
    }

    .endpoint-info i {
        font-size: 14px;
        margin-top: 1px;
        flex-shrink: 0;
    }

    .endpoint-info code {
        background: rgba(0, 0, 0, 0.1);
        padding: 2px 4px;
        border-radius: 3px;
        font-family: "Consolas", "Monaco", "Courier New", monospace;
        font-size: 11px;
    }

    .checkbox-group {
        margin-bottom: 12px;
    }

    .checkbox-label {
        display: flex;
        align-items: center;
        cursor: pointer;
        font-weight: normal;
        user-select: none;
    }

    .checkbox-label input[type="checkbox"] {
        width: auto;
        margin-right: 8px;
    }

    .settings-footer {
        padding: 20px 24px;
        border-top: 1px solid #e0e0e0;
        flex-shrink: 0;
    }

    .save-message {
        padding: 8px 12px;
        border-radius: 6px;
        margin-bottom: 12px;
        font-size: 14px;
    }

    .save-message.success {
        background: #d4edda;
        color: #155724;
        border: 1px solid #c3e6cb;
    }

    .save-message.error {
        background: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }

    .settings-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
    }

    .reset-btn,
    .save-btn {
        padding: 10px 20px;
        border: none;
        border-radius: 6px;
        font-family: inherit;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .reset-btn {
        background: #6c757d;
        color: white;
    }

    .reset-btn:hover {
        background: #5a6268;
    }
    .save-btn {
        background: #379df1;
        color: white;
    }

    .save-btn:hover:not(:disabled) {
        background: #2980e6;
    }

    .save-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .spinning {
        animation: spin 1s linear infinite;
    }
    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    .prompt-textarea {
        width: 100%;
        max-width: 100%;
        padding: 12px;
        border: 1px solid #ddd;
        border-radius: 6px;
        font-family: "Consolas", "Monaco", "Courier New", monospace;
        font-size: 13px;
        line-height: 1.4;
        resize: vertical;
        min-height: 120px;
        transition: border-color 0.2s;
        box-sizing: border-box;
    }

    .prompt-textarea:focus {
        outline: none;
        border-color: #379df1;
        box-shadow: 0 0 0 2px rgba(55, 157, 241, 0.1);
    }

    .about-content {
        background: #f8f9fa;
        border: 1px solid #e0e0e0;
        border-radius: 8px;
        padding: 16px;
    }

    .about-item {
        margin-bottom: 8px;
        font-size: 14px;
        line-height: 1.5;
    }

    .about-item:last-child {
        margin-bottom: 0;
    }

    .about-item strong {
        color: #333;
        min-width: 80px;
        display: inline-block;
    }
    .about-item a {
        color: #379df1;
        text-decoration: none;
        transition: color 0.2s;
    }
    .about-item a:hover {
        color: #2980e6;
        text-decoration: underline;
    }

    /* Provider Grid - Using CSS Grid for layout, Flexbox for card content */
    .provider-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        margin-top: 0.75rem;
        width: 100%;
    }

    .provider-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 1.5rem 1rem;
        border: 2px solid #e5e7eb;
        border-radius: 16px;
        background: #ffffff;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        position: relative;
        min-height: 120px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        /* Ensure equal height and width */
        aspect-ratio: 1;
    }

    .provider-card:hover {
        border-color: #3b82f6;
        background: #f8faff;
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(59, 130, 246, 0.15);
    }

    .provider-card.active {
        border-color: #3b82f6;
        background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
        color: white;
        box-shadow: 0 8px 25px rgba(59, 130, 246, 0.35);
        transform: translateY(-1px);
    }

    .provider-card.active:hover {
        background: linear-gradient(135deg, #1d4ed8 0%, #1e40af 100%);
        transform: translateY(-3px);
        box-shadow: 0 12px 35px rgba(59, 130, 246, 0.4);
    }

    .provider-logo {
        width: 3rem;
        height: 3rem;
        margin-bottom: 0.75rem;
        display: flex;
        align-items: center;
        justify-content: center;
        /* Ensure consistent sizing */
        flex-shrink: 0;
    }

    .provider-logo svg {
        width: 100%;
        height: 100%;
        transition: all 0.3s ease;
        /* Ensure SVG fills the container properly */
        object-fit: contain;
    }

    .provider-card:not(.active) .provider-logo svg {
        color: #6b7280;
    }

    .provider-card:hover:not(.active) .provider-logo svg {
        color: #3b82f6;
        transform: scale(1.05);
    }

    .provider-card.active .provider-logo svg {
        color: white;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }

    .provider-name {
        font-size: 0.95rem;
        font-weight: 600;
        text-align: center;
        line-height: 1.2;
        margin: 0;
        /* Ensure text doesn't grow the card */
        flex-shrink: 0;
    }

    .provider-card:not(.active) .provider-name {
        color: #374151;
    }

    .provider-card:hover:not(.active) .provider-name {
        color: #3b82f6;
    }

    .provider-card.active .provider-name {
        color: white;
    }

    /* Responsive adjustments for provider grid */
    @media (max-width: 768px) {
        .provider-grid {
            gap: 0.75rem;
        }

        .provider-card {
            padding: 1rem 0.75rem;
            min-height: 100px;
            /* Maintain aspect ratio on mobile */
            aspect-ratio: 1;
        }

        .provider-logo {
            width: 2.5rem;
            height: 2.5rem;
            margin-bottom: 0.5rem;
        }

        .provider-name {
            font-size: 0.875rem;
        }
    }

    @media (max-width: 480px) {
        .provider-grid {
            grid-template-columns: 1fr;
            max-width: 280px;
            margin: 0.75rem auto 0;
        }

        .provider-card {
            aspect-ratio: 3/1;
            flex-direction: row;
            justify-content: flex-start;
            padding: 1rem 1.5rem;
            min-height: auto;
        }

        .provider-logo {
            margin-right: 1rem;
            margin-bottom: 0;
            width: 2rem;
            height: 2rem;
        }

        .provider-name {
            font-size: 1rem;
        }
    }

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

        .settings-section h3 {
            color: #f6f6f6;
            border-color: #444;
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

        .form-group small {
            color: #ccc;
        }

        .form-group small code {
            background: #444;
            color: #7dd3fc;
        }

        .settings-footer {
            border-color: #444;
        }

        .prompt-textarea {
            background: #404040;
            border-color: #444;
            color: #f6f6f6;
        }
        .prompt-textarea:focus {
            border-color: #379df1;
            background: #4a4a4a;
        }

        .about-content {
            background: #383838;
            border-color: #444;
        }

        .about-item strong {
            color: #f6f6f6;
        }
        .about-item a {
            color: #379df1;
        }

        .about-item a:hover {
            color: #2980e6;
        }

        .save-message.success {
            background: #1e4620;
            color: #a3cfac;
            border-color: #2d5930;
        }
        .save-message.error {
            background: #4a1e23;
            color: #f5c6cb;
            border-color: #5c2329;
        }

        .endpoint-info.success {
            background: #1e4620;
            color: #a3cfac;
            border-color: #2d5930;
        }

        .endpoint-info.error {
            background: #4a1e23;
            color: #f5c6cb;
            border-color: #5c2329;
        }
        .endpoint-info code {
            background: rgba(255, 255, 255, 0.1);
            color: #7dd3fc;
        }

        /* Dark mode provider grid styles */
        .provider-card {
            border-color: #444;
            background: #3a3a3a;
        }

        .provider-card:hover {
            border-color: #3b82f6;
            background: #404040;
        }

        .provider-card.active {
            border-color: #3b82f6;
            background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
        }

        .provider-card:not(.active) .provider-name {
            color: #f6f6f6;
        }

        .provider-card:not(.active) .provider-logo svg {
            color: #ccc;
        }
    }

    @media (max-width: 768px) {
        .settings-overlay {
            padding: 10px;
        }

        .settings-container {
            max-height: 95vh;
        }

        .settings-actions {
            flex-direction: column;
        }

        .reset-btn,
        .save-btn {
            width: 100%;
            justify-content: center;
        }
    }
</style>
