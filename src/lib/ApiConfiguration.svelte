<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core"; // TODO: Implement API testing

  interface Props {
    config: any
    isValidatingApiKey: boolean
    apiKeyValid: boolean | null
    azureEndpointInfo: any
    onConfigChange: (updates: any) => void
    onApiProviderChange: () => void
    onAzureEndpointChange: () => void
    validateApiKey: () => void
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
  }: Props = $props()
  function updateConfig(field: string, value: any) {
    onConfigChange({ [field]: value })
  }

  // Get available configured models for fallback provider dropdown
  function getAvailableFallbackModels() {
    if (!config?.available_models) return []

    const availableModels: Array<{ value: string; label: string }> = []

    // Iterate through all providers and their models
    Object.entries(config.available_models).forEach(([provider, models]) => {
      if (models && Array.isArray(models)) {
        models.forEach((model) => {
          if (model.is_enabled) {
            const providerName =
              provider === "azure_openai" ? "Azure OpenAI"
              : provider === "openai" ? "OpenAI"
              : provider === "ollama" ? "Ollama"
              : provider.charAt(0).toUpperCase() + provider.slice(1)

            availableModels.push({
              value: `${provider}:${model.name}`,
              label: `${providerName}: ${model.display_name || model.name}`,
            })
          }
        })
      }
    })

    return availableModels
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
<div class="">
  <h4 class=""><i class=""></i>API Configuration</h4>
  <div class="">
    <fieldset>
      <legend class="">API Provider</legend>
      <div class="">
        <div class="">
          <button
            type="button"
            class=""
            onclick={() => {
              updateConfig("api_provider", "openai")
              onApiProviderChange()
            }}
          >
            <div class="">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="48"
                height="48"
                viewBox="0 0 16 16"
                fill="currentColor"
              >
                <path
                  d="M14.949 6.547a3.94 3.94 0 0 0-.348-3.273 4.11 4.11 0 0 0-4.4-1.934A4.1 4.1 0 0 0 8.423.2 4.15 4.15 0 0 0 6.305.086a4.1 4.1 0 0 0-1.891.948 4.04 4.04 0 0 0-1.158 1.753 4.1 4.1 0 0 0-1.563.679A4 4 0 0 0 .554 4.72a3.99 3.99 0 0 0 .502 4.731 3.94 3.94 0 0 0 .346 3.274 4.11 4.11 0 0 0 4.402 1.933c.382.425.852.764 1.377.995.526.231 1.095.35 1.67.346 1.78.002 3.358-1.132 3.901-2.804a4.1 4.1 0 0 0 1.563-.68 4 4 0 0 0 1.14-1.253 3.99 3.99 0 0 0-.506-4.716m-6.097 8.406a3.05 3.05 0 0 1-1.945-.694l.096-.054 3.23-1.838a.53.53 0 0 0 .265-.455v-4.49l1.366.778q.02.011.025.035v3.722c-.003 1.653-1.361 2.992-3.037 2.996m-6.53-2.75a2.95 2.95 0 0 1-.36-2.01l.095.057L5.29 12.09a.53.53 0 0 0 .527 0l3.949-2.246v1.555a.05.05 0 0 1-.022.041L6.473 13.3c-1.454.826-3.311.335-4.15-1.098m-.85-6.94A3.02 3.02 0 0 1 3.07 3.949v3.785a.51.51 0 0 0 .262.451l3.93 2.237-1.366.779a.05.05 0 0 1-.048 0L2.585 9.342a2.98 2.98 0 0 1-1.113-4.094zm11.216 2.571L8.747 5.576l1.362-.776a.05.05 0 0 1 .048 0l3.265 1.86a3 3 0 0 1 1.173 1.207 2.96 2.96 0 0 1-.27 3.2 3.05 3.05 0 0 1-1.36.997V8.279a.52.52 0 0 0-.276-.445m1.36-2.015-.097-.057-3.226-1.855a.53.53 0 0 0-.53 0L6.249 6.153V4.598a.04.04 0 0 1 .019-.04L9.533 2.7a3.07 3.07 0 0 1 3.257.139c.474.325.843.778 1.066 1.303.223.526.289 1.103.191 1.664zM5.503 8.575 4.139 7.8a.05.05 0 0 1-.026-.037V4.049c0-.57.166-1.127.476-1.607s.752-.864 1.275-1.105a3.08 3.08 0 0 1 3.234.41l-.096.054-3.23 1.838a.53.53 0 0 0-.265.455zm.742-1.577 1.758-1 1.762 1v2l-1.755 1-1.762-1z"
                ></path>
              </svg>
            </div>
            <div>OpenAI</div>
          </button>
        </div>

        <div class="">
          <button
            type="button"
            class=""
            onclick={() => {
              updateConfig("api_provider", "azure_openai")
              onApiProviderChange()
            }}
          >
            <div class="">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="48"
                height="48"
                viewBox="0 0 16 16"
                fill="currentColor"
              >
                <path
                  d="M7.462 0H0v7.19h7.462zM16 0H8.538v7.19H16zM7.462 8.211H0V16h7.462zm8.538 0H8.538V16H16z"
                ></path>
              </svg>
            </div>
            <div>Azure OpenAI</div>
          </button>
        </div>

        <div class="">
          <button
            type="button"
            class=""
            onclick={() => {
              updateConfig("api_provider", "azure_translator")
              onApiProviderChange()
            }}
          >
            <div class="">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="48"
                height="48"
                fill="currentColor"
                viewBox="0 0 64 64"
              >
                <path
                  fill-rule="nonzero"
                  d="M50.916 36.445c-2.917-.056-5.787.736-8.263 2.28v3.991a11.9737 11.9737 0 0 1 7.932-2.972c3.343 0 5.013 2.065 5.01 6.195l-7.29 1.023c-5.35.742-8.025 3.385-8.025 7.931-.04.947.123 1.891.477 2.77.355.878.893 1.671 1.578 2.325a7.988 7.988 0 0 0 5.685 1.924 8.1532 8.1532 0 0 0 4.369-1.109 8.1558 8.1558 0 0 0 3.112-3.26h.094v3.8h3.893V45.514c0-6.046-2.857-9.069-8.572-9.069Zm4.67 15.005a7.241 7.241 0 0 1-1.889 5.142 6.2497 6.2497 0 0 1-2.169 1.535c-.825.351-1.716.519-2.612.494a4.9035 4.9035 0 0 1-3.397-1.125 3.705 3.705 0 0 1-.963-1.311 3.7022 3.7022 0 0 1-.31-1.597 3.8755 3.8755 0 0 1 1.375-3.397 9.5452 9.5452 0 0 1 4.102-1.355l5.863-.806v2.42Zm-2.543-18.957 6.526-6.708c.709-.705.45-1.321-.679-1.321h-2.9a.849.849 0 0 1-.601-.248.8525.8525 0 0 1-.248-.601c.093-4.326-1.121-16.338-17.532-16.746a.85.85 0 0 0-.849.85v4.246c0 .117.024.233.071.341.046.108.115.204.201.284.086.08.188.141.299.18.111.038.229.053.346.044 1.549-.335 3.154-.299 4.686.104 1.533.402 2.948 1.16 4.132 2.213a10.0675 10.0675 0 0 1 2.684 3.843c.58 1.474.804 3.064.654 4.641a.8484.8484 0 0 1-.849.849h-2.658c-1.367 0-1.596.425-.684 1.321l6.051 6.708c.079.104.181.188.298.246.117.058.246.088.377.088s.26-.03.377-.088a.8454.8454 0 0 0 .298-.246Zm-39.325 4.42-6.53 6.708c-.709.709-.45 1.321.684 1.321h2.895c.226 0 .442.089.601.248.159.16.249.376.249.601-.107 4.327 1.103 16.338 17.514 16.746.225 0 .441-.09.6-.249a.8494.8494 0 0 0 .249-.6v-4.246c0-.117-.024-.233-.07-.341a.8456.8456 0 0 0-.201-.283.8552.8552 0 0 0-.297-.18.8432.8432 0 0 0-.345-.045c-1.551.335-3.159.299-4.694-.105a10.07 10.07 0 0 1-4.137-2.221 10.0826 10.0826 0 0 1-2.682-3.855 10.0813 10.0813 0 0 1-.643-4.651c0-.111.022-.222.065-.325a.8358.8358 0 0 1 .184-.275.8358.8358 0 0 1 .275-.184.8455.8455 0 0 1 .325-.065h2.684c1.363 0 1.592-.425.679-1.32l-6.042-6.679a.839.839 0 0 0-.3-.252.8458.8458 0 0 0-.763 0 .839.839 0 0 0-.3.252Zm8.959-23.489c2.285.403 4.388 1.51 6.012 3.167a8.4896 8.4896 0 0 1 1.941 2.816c.449 1.061.678 2.202.673 3.354a8.5075 8.5075 0 0 1-.705 3.346 8.5006 8.5006 0 0 1-1.968 2.798 12.8967 12.8967 0 0 1-8.127 3.15l-1.21-2.73c2.414.069 4.788-.617 6.794-1.962a5.5147 5.5147 0 0 0 2.398-4.573A6.0235 6.0235 0 0 0 28 20.429a6.019 6.019 0 0 0-1.374-1.982 8.6929 8.6929 0 0 0-4.641-2.433 27.5372 27.5372 0 0 1-4.832 9.765 9.37 9.37 0 0 0 1.113 1.941l-2.276 1.579c-.35-.442-.66-.915-.926-1.414a9.818 9.818 0 0 1-6.131 2.697c-.641.018-1.28-.092-1.878-.323a4.8383 4.8383 0 0 1-1.608-1.023 5.3072 5.3072 0 0 1-1.112-1.844 5.3048 5.3048 0 0 1-.285-2.135c.045-2.257.843-4.435 2.268-6.186a14.0818 14.0818 0 0 1 5.8-4.505c.068-2.264.141-3.848.22-4.751-2.36.091-4.224.136-5.592.136l-.454-2.628c1.469.053 3.551.031 6.246-.068.187-1.946.485-3.88.892-5.792l2.628.387-.726 5.205c3.686-.24 7.357-.679 10.997-1.316l.556 2.547c-3.904.659-7.84 1.116-11.791 1.368a29.993 29.993 0 0 0-.251 3.94c1.3-.328 2.634-.504 3.975-.522.348 0 .687 0 1.027.033.242-.704.406-1.433.488-2.173l2.692.386c-.069.709-.185 1.412-.348 2.106Zm-8.823 11.77a29.5187 29.5187 0 0 1-1.617-7.732 10.4795 10.4795 0 0 0-3.877 3.396 7.479 7.479 0 0 0-1.431 4.246c-.046.382-.016.769.089 1.139.104.37.28.716.518 1.018.33.337.777.535 1.249.552a6.788 6.788 0 0 0 2.22-.586 8.2776 8.2776 0 0 0 2.849-2.033Zm5.269-9.384a15.72 15.72 0 0 0-4.144.624c.055 2.106.384 4.195.977 6.216a25.442 25.442 0 0 0 3.167-6.84Z"
                ></path>
              </svg>
            </div>
            <div>Azure AI Translator</div>
          </button>
        </div>

        <div class="">
          <button
            type="button"
            class=""
            onclick={() => {
              updateConfig("api_provider", "ollama")
              onApiProviderChange()
            }}
          >
            <div class="">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="48"
                height="48"
                viewBox="0 0 24 24"
                fill="currentColor"
                fill-rule="evenodd"
              >
                <path
                  d="M7.905 1.09c.216.085.411.225.588.41.295.306.544.744.734 1.263.191.522.315 1.1.362 1.68a5.054 5.054 0 0 1 2.049-.636l.051-.004c.87-.07 1.73.087 2.48.474.101.053.2.11.297.17.05-.569.172-1.134.36-1.644.19-.52.439-.957.733-1.264a1.67 1.67 0 0 1 .589-.41c.257-.1.53-.118.796-.042.401.114.745.368 1.016.737.248.337.434.769.561 1.287.23.934.27 2.163.115 3.645l.053.04.026.019c.757.576 1.284 1.397 1.563 2.35.435 1.487.216 3.155-.534 4.088l-.018.021.002.003c.417.762.67 1.567.724 2.4l.002.03c.064 1.065-.2 2.137-.814 3.19l-.007.01.01.024c.472 1.157.62 2.322.438 3.486l-.006.039a.651.651 0 0 1-.747.536.648.648 0 0 1-.54-.742c.167-1.033.01-2.069-.48-3.123a.643.643 0 0 1 .04-.617l.004-.006c.604-.924.854-1.83.8-2.72-.046-.779-.325-1.544-.8-2.273a.644.644 0 0 1 .18-.886l.009-.006c.243-.159.467-.565.58-1.12a4.229 4.229 0 0 0-.095-1.974c-.205-.7-.58-1.284-1.105-1.683-.595-.454-1.383-.673-2.38-.61a.653.653 0 0 1-.632-.371c-.314-.665-.772-1.141-1.343-1.436a3.288 3.288 0 0 0-1.772-.332c-1.245.099-2.343.801-2.67 1.686a.652.652 0 0 1-.61.425c-1.067.002-1.893.252-2.497.703-.522.39-.878.935-1.066 1.588a4.07 4.07 0 0 0-.068 1.886c.112.558.331 1.02.582 1.269l.008.007c.212.207.257.53.109.785-.36.622-.629 1.549-.673 2.44-.05 1.018.186 1.902.719 2.536l.016.019a.643.643 0 0 1 .095.69c-.576 1.236-.753 2.252-.562 3.052a.652.652 0 0 1-1.269.298c-.243-1.018-.078-2.184.473-3.498l.014-.035-.008-.012a4.339 4.339 0 0 1-.598-1.309l-.005-.019a5.764 5.764 0 0 1-.177-1.785c.044-.91.278-1.842.622-2.59l.012-.026-.002-.002c-.293-.418-.51-.953-.63-1.545l-.005-.024a5.352 5.352 0 0 1 .093-2.49c.262-.915.777-1.701 1.536-2.269.06-.045.123-.09.186-.132-.159-1.493-.119-2.73.112-3.67.127-.518.314-.95.562-1.287.27-.368.614-.622 1.015-.737.266-.076.54-.059.797.042zm4.116 9.09c.936 0 1.8.313 2.446.855.63.527 1.005 1.235 1.005 1.94 0 .888-.406 1.58-1.133 2.022-.62.375-1.451.557-2.403.557-1.009 0-1.871-.259-2.493-.734-.617-.47-.963-1.13-.963-1.845 0-.707.398-1.417 1.056-1.946.668-.537 1.55-.849 2.485-.849zm0 .896a3.07 3.07 0 0 0-1.916.65c-.461.37-.722.835-.722 1.25 0 .428.21.829.61 1.134.455.347 1.124.548 1.943.548.799 0 1.473-.147 1.932-.426.463-.28.7-.686.7-1.257 0-.423-.246-.89-.683-1.256-.484-.405-1.14-.643-1.864-.643zm.662 1.21.004.004c.12.151.095.37-.056.49l-.292.23v.446a.375.375 0 0 1-.376.373.375.375 0 0 1-.376-.373v-.46l-.271-.218a.347.347 0 0 1-.052-.49.353.353 0 0 1 .494-.051l.215.172.22-.174a.353.353 0 0 1 .49.051zm-5.04-1.919c.478 0 .867.39.867.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zm8.706 0c.48 0 .868.39.868.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zM7.44 2.3l-.003.002a.659.659 0 0 0-.285.238l-.005.006c-.138.189-.258.467-.348.832-.17.692-.216 1.631-.124 2.782.43-.128.899-.208 1.404-.237l.01-.001.019-.034c.046-.082.095-.161.148-.239.123-.771.022-1.692-.253-2.444-.134-.364-.297-.65-.453-.813a.628.628 0 0 0-.107-.09L7.44 2.3zm9.174.04-.002.001a.628.628 0 0 0-.107.09c-.156.163-.32.45-.453.814-.29.794-.387 1.776-.23 2.572l.058.097.008.014h.03a5.184 5.184 0 0 1 1.466.212c.086-1.124.038-2.043-.128-2.722-.09-.365-.21-.643-.349-.832l-.004-.006a.659.659 0 0 0-.285-.239h-.004z"
                ></path>
              </svg>
            </div>
            <div>Ollama</div>
          </button>
        </div>
      </div>
    </fieldset>
  </div>

  {#if config.api_provider === "openai"}
    <div class="">
      <label for="openai-key" class="">OpenAI API Key</label>
      <div class="">
        <input
          id="openai-key"
          type="password"
          class=""
          value={config.openai_api_key}
          placeholder="sk-..."
          onblur={validateApiKey}
          oninput={(e) =>
            updateConfig(
              "openai_api_key",
              (e.target as HTMLInputElement).value
            )}
        />
        <span class="">
          {#if isValidatingApiKey}
            <i class=""></i>
          {:else if apiKeyValid === true}
            <i class=""></i>
          {:else if apiKeyValid === false}
            <i class=""></i>
          {:else}
            <i class=""></i>
          {/if}
        </span>
      </div>
    </div>
  {:else if config.api_provider === "azure_openai"}
    <div class="">
      <label for="azure-endpoint" class=""
        >Azure OpenAI Endpoint</label
      >
      <input
        id="azure-endpoint"
        type="url"
        class=""
        value={config.azure_endpoint}
        placeholder="Paste your full Azure OpenAI endpoint URL here..."
        onblur={validateApiKey}
        oninput={(e) => {
          updateConfig("azure_endpoint", (e.target as HTMLInputElement).value)
          onAzureEndpointChange()
        }}
      />
      <div class="">
        Paste the complete endpoint URL from Azure portal. Supported formats:
        <br />
        •
        <code>https://resource.cognitiveservices.azure.com/openai/...</code>
        <br />
        •
        <code>https://resource.services.ai.azure.com/models/...</code>
        <br />
        The app will automatically extract the base URL, API version, and deployment
        name.
      </div>

      {#if azureEndpointInfo?.isValid}
        <div class="">
          <i class=""></i>
          <div>
            <strong>Auto-detected:</strong>
            {azureEndpointInfo.type} endpoint
            {#if azureEndpointInfo.deploymentDetected}
              • Deployment: <code>{azureEndpointInfo.deploymentDetected}</code>
            {/if}
            {#if azureEndpointInfo.apiVersionDetected}
              • API Version: <code>{azureEndpointInfo.apiVersionDetected}</code>
            {/if}
          </div>
        </div>
      {:else if azureEndpointInfo?.isValid === false}
        <div class="">
          <i class=""></i>
          <div>
            <strong>Invalid endpoint format.</strong> Please use a valid Azure OpenAI
            endpoint URL.
          </div>
        </div>
      {/if}
    </div>

    <div class="">
      <label for="azure-key" class="">Azure API Key</label>
      <div class="">
        <input
          id="azure-key"
          type="password"
          class=""
          value={config.azure_api_key}
          placeholder="Your Azure API key"
          onblur={validateApiKey}
          oninput={(e) =>
            updateConfig("azure_api_key", (e.target as HTMLInputElement).value)}
        />
        <span class="">
          {#if isValidatingApiKey}
            <i class=""></i>
          {:else if apiKeyValid === true}
            <i class=""></i>
          {:else if apiKeyValid === false}
            <i class=""></i>
          {:else}
            <i class=""></i>
          {/if}
        </span>
      </div>
    </div>
    <div class="">
      <div class="">
        <label for="azure-deployment" class=""
          >Azure Deployment Name</label
        >
        <input
          id="azure-deployment"
          type="text"
          class=""
          value={config.azure_deployment_name}
          placeholder="gpt-4"
          oninput={(e) =>
            updateConfig(
              "azure_deployment_name",
              (e.target as HTMLInputElement).value
            )}
        />
      </div>
      <div class="">
        <label for="azure-api-version" class=""
          >Azure API Version</label
        >
        <input
          id="azure-api-version"
          type="text"
          class=""
          value={config.azure_api_version}
          placeholder="2025-01-01-preview"
          oninput={(e) =>
            updateConfig(
              "azure_api_version",
              (e.target as HTMLInputElement).value
            )}
        />
        <div class="">
          API version for Azure OpenAI requests (e.g., 2025-01-01-preview)
        </div>
      </div>
    </div>
  {:else if config.api_provider === "azure_translator"}
    <div class="">
      <label for="azure-translator-endpoint" class=""
        >Azure Translator Endpoint</label
      >
      <input
        id="azure-translator-endpoint"
        type="url"
        class=""
        value={config.azure_translator_endpoint}
        placeholder="https://api.cognitive.microsofttranslator.com"
        onblur={validateApiKey}
        oninput={(e) =>
          updateConfig(
            "azure_translator_endpoint",
            (e.target as HTMLInputElement).value
          )}
      />
      <div class="">
        Azure Translator Service endpoint URL. Default is the global endpoint.
      </div>
    </div>

    <div class="">
      <label for="azure-translator-key" class=""
        >Azure Translator API Key</label
      >
      <div class="">
        <input
          id="azure-translator-key"
          type="password"
          class=""
          value={config.azure_translator_api_key}
          placeholder="Your Azure Translator API key"
          onblur={validateApiKey}
          oninput={(e) =>
            updateConfig(
              "azure_translator_api_key",
              (e.target as HTMLInputElement).value
            )}
        />
        <span class="">
          {#if isValidatingApiKey}
            <i class=""></i>
          {:else if apiKeyValid === true}
            <i class=""></i>
          {:else if apiKeyValid === false}
            <i class=""></i>
          {:else}
            <i class=""></i>
          {/if}
        </span>
      </div>
      <div class="">
        Get your API key from the Azure portal under your Translator resource's
        "Keys and Endpoint" section.
      </div>
    </div>

    <div class="">
      <label for="azure-translator-region" class=""
        >Azure Translator Region</label
      >
      <input
        id="azure-translator-region"
        type="text"
        class=""
        value={config.azure_translator_region}
        placeholder="e.g., eastus, westus2, uksouth"
        onblur={validateApiKey}
        oninput={(e) =>
          updateConfig(
            "azure_translator_region",
            (e.target as HTMLInputElement).value
          )}
      />
      <div class="">
        Azure region where your Translator resource is located. Required for
        multi-service or regional resources. Optional for global single-service
        resources. Find this in the Azure portal under "Keys and Endpoint".
      </div>
    </div>

    <!-- Fallback Provider for Alternative Translations -->
    <div class="">
      <label for="alternatives-fallback" class="">
        <i class=""></i>
        Alternative Translations Fallback Provider
      </label>
      <select
        id="alternatives-fallback"
        class=""
        value={config.alternatives_fallback_provider || ""}
        onchange={(e) =>
          updateConfig(
            "alternatives_fallback_provider",
            (e.target as HTMLSelectElement).value || null
          )}
      >
        <option value="">Not configured</option>
        {#if getAvailableFallbackModels().length === 0}
          <option value="" disabled>No configured models available</option>
        {:else}
          {#each getAvailableFallbackModels() as model}
            <option value={model.value}>{model.label}</option>
          {/each}
        {/if}
      </select>
      <div class="">
        Azure Translator cannot generate alternative translations by itself.
        Configure a fallback AI model to enable alternative translations when
        using Azure Translator. Only models that you have configured and enabled
        in the Model Management section will appear in this dropdown.
        {#if getAvailableFallbackModels().length === 0}
          <br /><strong>No models available:</strong> Please configure and enable
          at least one AI model in the Model Management section first.
        {/if}
      </div>
    </div>
  {:else if config.api_provider === "ollama"}
    <div class="">
      <label for="ollama-url" class="">Ollama Server URL</label>
      <div class="">
        <input
          id="ollama-url"
          type="text"
          class=""
          value={config.ollama_url || "http://localhost:11434"}
          placeholder="http://localhost:11434"
          onblur={validateApiKey}
          oninput={(e) =>
            updateConfig("ollama_url", (e.target as HTMLInputElement).value)}
        />
        <span class="">
          {#if isValidatingApiKey}
            <i class=""></i>
          {:else if apiKeyValid === true}
            <i class=""></i>
          {:else if apiKeyValid === false}
            <i class=""></i>
          {:else}
            <i class=""></i>
          {/if}
        </span>
      </div>
      <div class="">
        URL of your Ollama server. Make sure Ollama is running locally or
        provide the remote server URL.
      </div>
    </div>
  {/if}

  <!-- Model selection is now handled in the Model Management tab -->
</div>

<style>
  /* CSS moved to /src/app.css */
</style>
