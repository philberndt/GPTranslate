<script lang="ts">
    // import { invoke } from "@tauri-apps/api/core"; // TODO: Implement API testing

    interface Props {
        config: any;
        isValidatingApiKey: boolean;
        apiKeyValid: boolean | null;
        azureEndpointInfo: any;
        onConfigChange: (updates: any) => void;
        onApiProviderChange: () => void;
        onAzureEndpointChange: () => void;
        validateApiKey: () => void;
    }

    let {
        config,
        isValidatingApiKey,
        apiKeyValid,
        azureEndpointInfo,
        onConfigChange,
        onApiProviderChange,
        onAzureEndpointChange,
        validateApiKey,
    }: Props = $props();
    function updateConfig(field: string, value: any) {
        onConfigChange({ [field]: value });
    } // async function testProvider(provider: ProviderKey) {
    //     isTesting[provider] = true;
    //     testResults[provider] = null;

    //     try {
    //         // For now, just use the existing validateApiKey function
    //         // TODO: Implement proper provider testing    //         await validateApiKey();
    //         testResults[provider] = true;
    //     } catch (e) {
    //         console.error(`Failed to test ${provider}:`, e);
    //         testResults[provider] = false;
    //     } finally {
    //         isTesting[provider] = false;
    //     }
    // }
</script>

<!-- API Configuration Section -->
<section class="settings-section">
    <h3><i class="bi bi-cloud"></i>API Configuration</h3>
    <div class="form-group">
        <label for="api-provider">API Provider</label>
        <div class="provider-grid">
            <button
                type="button"
                class="provider-card {config.api_provider === 'openai'
                    ? 'active'
                    : ''}"
                onclick={() => {
                    updateConfig("api_provider", "openai");
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
                class="provider-card {config.api_provider === 'azure_openai'
                    ? 'active'
                    : ''}"
                onclick={() => {
                    updateConfig("api_provider", "azure_openai");
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
                class="provider-card {config.api_provider === 'ollama'
                    ? 'active'
                    : ''}"
                onclick={() => {
                    updateConfig("api_provider", "ollama");
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
                    value={config.openai_api_key}
                    placeholder="sk-..."
                    onblur={validateApiKey}
                    oninput={(e) =>
                        updateConfig(
                            "openai_api_key",
                            (e.target as HTMLInputElement).value,
                        )}
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
            <label for="azure-endpoint">Azure OpenAI Endpoint</label>
            <input
                id="azure-endpoint"
                type="url"
                value={config.azure_endpoint}
                placeholder="Paste your full Azure OpenAI endpoint URL here..."
                onblur={validateApiKey}
                oninput={(e) => {
                    updateConfig(
                        "azure_endpoint",
                        (e.target as HTMLInputElement).value,
                    );
                    onAzureEndpointChange();
                }}
            />
            <small>
                Paste the complete endpoint URL from Azure portal. Supported
                formats:
                <br />
                •
                <code
                    >https://resource.cognitiveservices.azure.com/openai/...</code
                >
                <br />
                •
                <code>https://resource.services.ai.azure.com/models/...</code>
                <br />
                The app will automatically extract the base URL, API version, and
                deployment name.
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
                    <strong>Invalid endpoint format.</strong> Please use a valid
                    Azure OpenAI endpoint URL.
                </div>
            {/if}
        </div>

        <div class="form-group">
            <label for="azure-key">Azure API Key</label>
            <div class="api-key-group">
                <input
                    id="azure-key"
                    type="password"
                    value={config.azure_api_key}
                    placeholder="Your Azure API key"
                    onblur={validateApiKey}
                    oninput={(e) =>
                        updateConfig(
                            "azure_api_key",
                            (e.target as HTMLInputElement).value,
                        )}
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
            <label for="azure-deployment">Azure Deployment Name</label>
            <input
                id="azure-deployment"
                type="text"
                value={config.azure_deployment_name}
                placeholder="gpt-4"
                oninput={(e) =>
                    updateConfig(
                        "azure_deployment_name",
                        (e.target as HTMLInputElement).value,
                    )}
            />
        </div>

        <div class="form-group">
            <label for="azure-api-version">Azure API Version</label>
            <input
                id="azure-api-version"
                type="text"
                value={config.azure_api_version}
                placeholder="2025-01-01-preview"
                oninput={(e) =>
                    updateConfig(
                        "azure_api_version",
                        (e.target as HTMLInputElement).value,
                    )}
            />
            <small>
                API version for Azure OpenAI requests (e.g., 2025-01-01-preview)
            </small>
        </div>
    {:else if config.api_provider === "ollama"}
        <div class="form-group">
            <label for="ollama-url">Ollama Server URL</label>
            <div class="api-key-group">
                <input
                    id="ollama-url"
                    type="text"
                    value={config.ollama_url || "http://localhost:11434"}
                    placeholder="http://localhost:11434"
                    onblur={validateApiKey}
                    oninput={(e) =>
                        updateConfig(
                            "ollama_url",
                            (e.target as HTMLInputElement).value,
                        )}
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
                URL of your Ollama server. Make sure Ollama is running locally
                or provide the remote server URL.
            </small>
        </div>
    {/if}

    <!-- Model selection is now handled in the Model Management tab -->
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

    .form-group {
        margin-bottom: 16px;
        min-width: 0;
    }

    .form-group label {
        display: block;
        font-weight: 500;
        margin-bottom: 4px;
        color: #333;
    }
    .form-group input {
        width: 100%;
        max-width: 100%;
        padding: 10px 12px;
        border: 1px solid #ddd;
        border-radius: 6px;
        font-family: inherit;
        font-size: 14px;
        transition: border-color 0.2s;
        box-sizing: border-box;
        min-width: 0;
    }

    .form-group input:focus {
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
        min-width: 0;
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

    /* Provider Grid */
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
        flex-shrink: 0;
    }

    .provider-logo svg {
        width: 100%;
        height: 100%;
        transition: all 0.3s ease;
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

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    /* Responsive adjustments */
    @media (max-width: 768px) {
        .provider-grid {
            gap: 0.75rem;
        }

        .provider-card {
            padding: 1rem 0.75rem;
            min-height: 100px;
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

    /* Dark mode */
    @media (prefers-color-scheme: dark) {
        .settings-section h3 {
            color: #f6f6f6;
            border-color: #444;
        }

        .form-group label {
            color: #f6f6f6;
        }
        .form-group input {
            background: #3a3a3a;
            border-color: #555;
            color: #f6f6f6;
        }

        .form-group input:focus {
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
</style>
