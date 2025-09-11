<script lang="ts">
  import { Cog6ToothIcon, BoltIcon } from "heroicons-svelte/24/outline"
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
        const providerName =
          provider.charAt(0).toUpperCase() + provider.slice(1).replace("_", " ")
        models.forEach((model: any) => {
          if (model.is_enabled) {
            availableModels.push({
              value: `${provider}:${model.name}`,
              label: `${providerName}: ${model.display_name || model.name}`,
            })
          }
        })
      }
    })

    return availableModels
  }
</script>

<div class="space-y-6 overflow-hidden">
  <div class="card bg-base-100 border border-base-300/50 form-control">
    <div class="card-body">
      <h5 class="card-title flex items-center gap-2">API Provider</h5>
      <div class="grid grid-cols-4 gap-2">
        <div class="form-control">
          <button
            type="button"
            class={`btn btn-soft w-full justify-center flex-col gap-2 text-center text-xs aspect-square h-24 rounded-xl ${config.api_provider === "openai" ? "btn-active" : ""}`}
            onclick={() => {
              updateConfig("api_provider", "openai")
              onApiProviderChange()
            }}
          >
            <div class="w-10 h-10 flex-shrink-0">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="40"
                height="40"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M22.2819 9.8211a5.9847 5.9847 0 0 0-.5157-4.9108 6.0462 6.0462 0 0 0-6.5098-2.9A6.0651 6.0651 0 0 0 4.9807 4.1818a5.9847 5.9847 0 0 0-3.9977 2.9 6.0462 6.0462 0 0 0 .7427 7.0966 5.98 5.98 0 0 0 .511 4.9107 6.051 6.051 0 0 0 6.5146 2.9001A5.9847 5.9847 0 0 0 13.2599 24a6.0557 6.0557 0 0 0 5.7718-4.2058 5.9894 5.9894 0 0 0 3.9977-2.9001 6.0557 6.0557 0 0 0-.7475-7.0729zm-9.022 12.6081a4.4755 4.4755 0 0 1-2.8764-1.0408l.1419-.0804 4.7783-2.7582a.7948.7948 0 0 0 .3927-.6813v-6.7369l2.02 1.1686a.071.071 0 0 1 .038.052v5.5826a4.504 4.504 0 0 1-4.4945 4.4944zm-9.6607-4.1254a4.4708 4.4708 0 0 1-.5346-3.0137l.142.0852 4.783 2.7582a.7712.7712 0 0 0 .7806 0l5.8428-3.3685v2.3324a.0804.0804 0 0 1-.0332.0615L9.74 19.9502a4.4992 4.4992 0 0 1-6.1408-1.6464zM2.3408 7.8956a4.485 4.485 0 0 1 2.3655-1.9728V11.6a.7664.7664 0 0 0 .3879.6765l5.8144 3.3543-2.0201 1.1685a.0757.0757 0 0 1-.071 0l-4.8303-2.7865A4.504 4.504 0 0 1 2.3408 7.872zm16.5963 3.8558L13.1038 8.364 15.1192 7.2a.0757.0757 0 0 1 .071 0l4.8303 2.7913a4.4944 4.4944 0 0 1-.6765 8.1042v-5.6772a.79.79 0 0 0-.407-.667zm2.0107-3.0231l-.142-.0852-4.7735-2.7818a.7759.7759 0 0 0-.7854 0L9.409 9.2297V6.8974a.0662.0662 0 0 1 .0284-.0615l4.8303-2.7866a4.4992 4.4992 0 0 1 6.6802 4.66zM8.3065 12.863l-2.02-1.1638a.0804.0804 0 0 1-.038-.0567V6.0742a4.4992 4.4992 0 0 1 7.3757-3.4537l-.142.0805L8.704 5.459a.7948.7948 0 0 0-.3927.6813zm1.0976-2.3654l2.602-1.4998 2.6069 1.4998v2.9994l-2.5974 1.4997-2.6067-1.4997Z"
                ></path>
              </svg>
            </div>
            <span class="truncate">OpenAI</span>
          </button>
        </div>
        <div class="form-control">
          <button
            type="button"
            class={`btn btn-soft w-full justify-center flex-col gap-2 text-center text-xs aspect-square h-24 rounded-xl ${config.api_provider === "azure_openai" ? "btn-active" : ""}`}
            onclick={() => {
              updateConfig("api_provider", "azure_openai")
              onApiProviderChange()
            }}
          >
            <div class="w-10 h-10 flex-shrink-0">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="40"
                height="40"
                viewBox="0 0 16 16"
                fill="currentColor"
              >
                <path
                  d="M7.462 0H0v7.19h7.462zM16 0H8.538v7.19H16zM7.462 8.211H0V16h7.462zm8.538 0H8.538V16H16z"
                ></path>
              </svg>
            </div>
            <span class="truncate">Azure OpenAI</span>
          </button>
        </div>

        <div class="form-control">
          <button
            type="button"
            class={`btn btn-soft w-full justify-center flex-col gap-2 text-center text-xs aspect-square h-24 rounded-xl ${config.api_provider === "azure_translator" ? "btn-active" : ""}`}
            onclick={() => {
              updateConfig("api_provider", "azure_translator")
              onApiProviderChange()
            }}
          >
            <div class="w-10 h-10 flex-shrink-0">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="40"
                height="40"
                fill="currentColor"
                viewBox="0 0 64 64"
              >
                <path
                  fill-rule="nonzero"
                  d="M50.916 36.445c-2.917-.056-5.787.736-8.263 2.28v3.991a11.9737 11.9737 0 0 1 7.932-2.972c3.343 0 5.013 2.065 5.01 6.195l-7.29 1.023c-5.35.742-8.025 3.385-8.025 7.931-.04.947.123 1.891.477 2.77.355.878.893 1.671 1.578 2.325a7.988 7.988 0 0 0 5.685 1.924 8.1532 8.1532 0 0 0 4.369-1.109 8.1558 8.1558 0 0 0 3.112-3.26h.094v3.8h3.893V45.514c0-6.046-2.857-9.069-8.572-9.069Zm4.67 15.005a7.241 7.241 0 0 1-1.889 5.142 6.2497 6.2497 0 0 1-2.169 1.535c-.825.351-1.716.519-2.612.494a4.9035 4.9035 0 0 1-3.397-1.125 3.705 3.705 0 0 1-.963-1.311 3.7022 3.7022 0 0 1-.31-1.597 3.8755 3.8755 0 0 1 1.375-3.397 9.5452 9.5452 0 0 1 4.102-1.355l5.863-.806v2.42Zm-2.543-18.957 6.526-6.708c.709-.705.45-1.321-.679-1.321h-2.9a.849.849 0 0 1-.601-.248.8525.8525 0 0 1-.248-.601c.093-4.326-1.121-16.338-17.532-16.746a.85.85 0 0 0-.849.85v4.246c0 .117.024.233.071.341.046.108.115.204.201.284.086.08.188.141.299.18.111.038.229.053.346.044 1.549-.335 3.154-.299 4.686.104 1.533.402 2.948 1.16 4.132 2.213a10.0675 10.0675 0 0 1 2.684 3.843c.58 1.474.804 3.064.654 4.641a.8484.8484 0 0 1-.849.849h-2.658c-1.367 0-1.596.425-.684 1.321l6.051 6.708c.079.104.181.188.298.246.117.058.246.088.377.088s.26-.03.377-.088a.8454.8454 0 0 0 .298-.246Zm-39.325 4.42-6.53 6.708c-.709.709-.45 1.321.684 1.321h2.895c.226 0 .442.089.601.248.159.16.249.376.249.601-.107 4.327 1.103 16.338 17.514 16.746.225 0 .441-.09.6-.249a.8494.8494 0 0 0 .249-.6v-4.246c0-.117-.024-.233-.07-.341a.8456.8456 0 0 0-.201-.283.8552.8552 0 0 0-.297-.18.8432.8432 0 0 0-.345-.045c-1.551.335-3.159.299-4.694-.105a10.07 10.07 0 0 1-4.137-2.221 10.0826 10.0826 0 0 1-2.682-3.855 10.0813 10.0813 0 0 1-.643-4.651c0-.111.022-.222.065-.325a.8358.8358 0 0 1 .184-.275.8358.8358 0 0 1 .275-.184.8455.8455 0 0 1 .325-.065h2.684c1.363 0 1.592-.425.679-1.32l-6.042-6.679a.839.839 0 0 0-.3-.252.8458.8458 0 0 0-.763 0 .839.839 0 0 0-.3.252Zm8.959-23.489c2.285.403 4.388 1.51 6.012 3.167a8.4896 8.4896 0 0 1 1.941 2.816c.449 1.061.678 2.202.673 3.354a8.5075 8.5075 0 0 1-.705 3.346 8.5006 8.5006 0 0 1-1.968 2.798 12.8967 12.8967 0 0 1-8.127 3.15l-1.21-2.73c2.414.069 4.788-.617 6.794-1.962a5.5147 5.5147 0 0 0 2.398-4.573A6.0235 6.0235 0 0 0 28 20.429a6.019 6.019 0 0 0-1.374-1.982 8.6929 8.6929 0 0 0-4.641-2.433 27.5372 27.5372 0 0 1-4.832 9.765 9.37 9.37 0 0 0 1.113 1.941l-2.276 1.579c-.35-.442-.66-.915-.926-1.414a9.818 9.818 0 0 1-6.131 2.697c-.641.018-1.28-.092-1.878-.323a4.8383 4.8383 0 0 1-1.608-1.023 5.3072 5.3072 0 0 1-1.112-1.844 5.3048 5.3048 0 0 1-.285-2.135c.045-2.257.843-4.435 2.268-6.186a14.0818 14.0818 0 0 1 5.8-4.505c.068-2.264.141-3.848.22-4.751-2.36.091-4.224.136-5.592.136l-.454-2.628c1.469.053 3.551.031 6.246-.068.187-1.946.485-3.88.892-5.792l2.628.387-.726 5.205c3.686-.24 7.357-.679 10.997-1.316l.556 2.547c-3.904.659-7.84 1.116-11.791 1.368a29.993 29.993 0 0 0-.251 3.94c1.3-.328 2.634-.504 3.975-.522.348 0 .687 0 1.027.033.242-.704.406-1.433.488-2.173l2.692.386c-.069.709-.185 1.412-.348 2.106Zm-8.823 11.77a29.5187 29.5187 0 0 1-1.617-7.732 10.4795 10.4795 0 0 0-3.877 3.396 7.479 7.479 0 0 0-1.431 4.246c-.046.382-.016.769.089 1.139.104.37.28.716.518 1.018.33.337.777.535 1.249.552a6.788 6.788 0 0 0 2.22-.586 8.2776 8.2776 0 0 0 2.849-2.033Zm5.269-9.384a15.72 15.72 0 0 0-4.144.624c.055 2.106.384 4.195.977 6.216a25.442 25.442 0 0 0 3.167-6.84Z"
                ></path>
              </svg>
            </div>
            <span class="truncate">Azure AI Translator</span>
          </button>
        </div>

        <div class="form-control">
          <button
            type="button"
            class={`btn btn-soft w-full justify-center flex-col gap-2 text-center text-xs aspect-square h-24 rounded-xl ${config.api_provider === "ollama" ? "btn-active" : ""}`}
            onclick={() => {
              updateConfig("api_provider", "ollama")
              onApiProviderChange()
            }}
          >
            <div class="w-10 h-10 flex-shrink-0">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="40"
                height="40"
                viewBox="0 0 24 24"
                fill="currentColor"
                fill-rule="evenodd"
              >
                <path
                  d="M7.905 1.09c.216.085.411.225.588.41.295.306.544.744.734 1.263.191.522.315 1.1.362 1.68a5.054 5.054 0 0 1 2.049-.636l.051-.004c.87-.07 1.73.087 2.48.474.101.053.2.11.297.17.05-.569.172-1.134.36-1.644.19-.52.439-.957.733-1.264a1.67 1.67 0 0 1 .589-.41c.257-.1.53-.118.796-.042.401.114.745.368 1.016.737.248.337.434.769.561 1.287.23.934.27 2.163.115 3.645l.053.04.026.019c.757.576 1.284 1.397 1.563 2.35.435 1.487.216 3.155-.534 4.088l-.018.021.002.003c.417.762.67 1.567.724 2.4l.002.03c.064 1.065-.2 2.137-.814 3.19l-.007.01.01.024c.472 1.157.62 2.322.438 3.486l-.006.039a.651.651 0 0 1-.747.536.648.648 0 0 1-.54-.742c.167-1.033.01-2.069-.48-3.123a.643.643 0 0 1 .04-.617l.004-.006c.604-.924.854-1.83.8-2.72-.046-.779-.325-1.544-.8-2.273a.644.644 0 0 1 .18-.886l.009-.006c.243-.159.467-.565.58-1.12a4.229 4.229 0 0 0-.095-1.974c-.205-.7-.58-1.284-1.105-1.683-.595-.454-1.383-.673-2.38-.61a.653.653 0 0 1-.632-.371c-.314-.665-.772-1.141-1.343-1.436a3.288 3.288 0 0 0-1.772-.332c-1.245.099-2.343.801-2.67 1.686a.652.652 0 0 1-.61.425c-1.067.002-1.893.252-2.497.703-.522.39-.878.935-1.066 1.588a4.07 4.07 0 0 0-.068 1.886c.112.558.331 1.02.582 1.269l.008.007c.212.207.257.53.109.785-.36.622-.629 1.549-.673 2.44-.05 1.018.186 1.902.719 2.536l.016.019a.643.643 0 0 1 .095.69c-.576 1.236-.753 2.252-.562 3.052a.652.652 0 0 1-1.269.298c-.243-1.018-.078-2.184.473-3.498l.014-.035-.008-.012a4.339 4.339 0 0 1-.598-1.309l-.005-.019a5.764 5.764 0 0 1-.177-1.785c.044-.91.278-1.842.622-2.59l.012-.026-.002-.002c-.293-.418-.51-.953-.63-1.545l-.005-.024a5.352 5.352 0 0 1 .093-2.49c.262-.915.777-1.701 1.536-2.269.06-.045.123-.09.186-.132-.159-1.493-.119-2.73.112-3.67.127-.518.314-.95.562-1.287.27-.368.614-.622 1.015-.737.266-.076.54-.059.797.042zm4.116 9.09c.936 0 1.8.313 2.446.855.63.527 1.005 1.235 1.005 1.94 0 .888-.406 1.58-1.133 2.022-.62.375-1.451.557-2.403.557-1.009 0-1.871-.259-2.493-.734-.617-.47-.963-1.13-.963-1.845 0-.707.398-1.417 1.056-1.946.668-.537 1.55-.849 2.485-.849zm0 .896a3.07 3.07 0 0 0-1.916.65c-.461.37-.722.835-.722 1.25 0 .428.21.829.61 1.134.455.347 1.124.548 1.943.548.799 0 1.473-.147 1.932-.426.463-.28.7-.686.7-1.257 0-.423-.246-.89-.683-1.256-.484-.405-1.14-.643-1.864-.643zm.662 1.21.004.004c.12.151.095.37-.056.49l-.292.23v.446a.375.375 0 0 1-.376.373.375.375 0 0 1-.376-.373v-.46l-.271-.218a.347.347 0 0 1-.052-.49.353.353 0 0 1 .494-.051l.215.172.22-.174a.353.353 0 0 1 .49.051zm-5.04-1.919c.478 0 .867.39.867.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zm8.706 0c.48 0 .868.39.868.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zM7.44 2.3l-.003.002a.659.659 0 0 0-.285.238l-.005.006c-.138.189-.258.467-.348.832-.17.692-.216 1.631-.124 2.782.43-.128.899-.208 1.404-.237l.01-.001.019-.034c.046-.082.095-.161.148-.239.123-.771.022-1.692-.253-2.444-.134-.364-.297-.65-.453-.813a.628.628 0 0 0-.107-.09L7.44 2.3zm9.174.04-.002.001a.628.628 0 0 0-.107.09c-.156.163-.32.45-.453.814-.29.794-.387 1.776-.23 2.572l.058.097.008.014h.03a5.184 5.184 0 0 1 1.466.212c.086-1.124.038-2.043-.128-2.722-.09-.365-.21-.643-.349-.832l-.004-.006a.659.659 0 0 0-.285-.239h-.004z"
                ></path>
              </svg>
            </div>
            <span class="truncate">Ollama</span>
          </button>
        </div>
      </div>
    </div>

    <div class="max-w-2xl mx-auto">
      {#if config.api_provider === "openai"}
        <div class="form-control w-full">
          <label class="label" for="openai-key">
            <span class="label-text font-medium">OpenAI API Key</span>
          </label>
          <div class="flex items-center gap-2">
            <input
              id="openai-key"
              type="password"
              class="input input-bordered bg-base-200 flex-1"
              value={config.openai_api_key}
              placeholder="sk-..."
              onblur={validateApiKey}
              oninput={(e) =>
                updateConfig(
                  "openai_api_key",
                  (e.target as HTMLInputElement).value
                )}
            />
            <div class="w-8 h-8 flex items-center justify-center">
              {#if isValidatingApiKey}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if apiKeyValid === true}
                <span class="text-success text-lg">✓</span>
              {:else if apiKeyValid === false}
                <span class="text-error text-lg">✕</span>
              {:else}
                <span class="text-base-content/50 text-lg">?</span>
              {/if}
            </div>
          </div>
        </div>
      {:else if config.api_provider === "azure_openai"}
        <div class="form-control w-full">
          <label class="label" for="azure-endpoint">
            <span class="label-text font-medium">Azure OpenAI Endpoint</span>
          </label>
          <input
            id="azure-endpoint"
            type="url"
            class="input input-bordered bg-base-200"
            value={config.azure_endpoint}
            placeholder="Paste your full Azure OpenAI endpoint URL here..."
            onblur={validateApiKey}
            oninput={(e) => {
              updateConfig(
                "azure_endpoint",
                (e.target as HTMLInputElement).value
              )
              onAzureEndpointChange()
            }}
          />
          <div class="label">
            <span class="label-text-alt text-base-content/70">
              Paste the complete endpoint URL from Azure portal. Supported
              formats:
              <br />
              •
              <code class="text-xs bg-base-300 px-1 py-0.5 rounded"
                >https://resource.cognitiveservices.azure.com/openai/...</code
              >
              <br />
              •
              <code class="text-xs bg-base-300 px-1 py-0.5 rounded"
                >https://resource.services.ai.azure.com/models/...</code
              >
              <br />
              The app will automatically extract the base URL, API version, and deployment
              name.
            </span>
          </div>
        </div>

        {#if azureEndpointInfo?.isValid}
          <div class="alert alert-success">
            <span>
              <strong>Auto-detected:</strong>
              {azureEndpointInfo.type} endpoint
              {#if azureEndpointInfo.deploymentDetected}
                • Deployment: <code
                  class="bg-success/20 px-1 py-0.5 rounded text-xs"
                  >{azureEndpointInfo.deploymentDetected}</code
                >
              {/if}
              {#if azureEndpointInfo.apiVersionDetected}
                • API Version: <code
                  class="bg-success/20 px-1 py-0.5 rounded text-xs"
                  >{azureEndpointInfo.apiVersionDetected}</code
                >
              {/if}
            </span>
          </div>
        {:else if azureEndpointInfo?.isValid === false}
          <div class="alert alert-error">
            <span>
              <strong>Invalid endpoint format.</strong> Please use a valid Azure
              OpenAI endpoint URL.
            </span>
          </div>
        {/if}

        <div class="form-control w-full">
          <label class="label" for="azure-key">
            <span class="label-text font-medium">Azure API Key</span>
          </label>
          <div class="flex items-center gap-2">
            <input
              id="azure-key"
              type="password"
              class="input input-bordered bg-base-200 flex-1"
              value={config.azure_api_key}
              placeholder="Your Azure API key"
              onblur={validateApiKey}
              oninput={(e) =>
                updateConfig(
                  "azure_api_key",
                  (e.target as HTMLInputElement).value
                )}
            />
            <div class="w-8 h-8 flex items-center justify-center">
              {#if isValidatingApiKey}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if apiKeyValid === true}
                <span class="text-success text-lg">✓</span>
              {:else if apiKeyValid === false}
                <span class="text-error text-lg">✕</span>
              {:else}
                <span class="text-base-content/50 text-lg">?</span>
              {/if}
            </div>
          </div>
        </div>

        <div class="grid md:grid-cols-2 gap-4">
          <div class="form-control w-full">
            <label class="label" for="azure-deployment">
              <span class="label-text font-medium">Azure Deployment Name</span>
            </label>
            <input
              id="azure-deployment"
              type="text"
              class="input input-bordered bg-base-200"
              value={config.azure_deployment_name}
              placeholder="my-gpt-4o-deployment"
              oninput={(e) =>
                updateConfig(
                  "azure_deployment_name",
                  (e.target as HTMLInputElement).value
                )}
            />
            <div class="label">
              <span class="label-text-alt text-base-content/70"
                >Custom name for your model deployment (used in API calls, not
                the model name)</span
              >
            </div>
          </div>
          <div class="form-control w-full">
            <label class="label" for="azure-api-version">
              <span class="label-text font-medium">Azure API Version</span>
            </label>
            <input
              id="azure-api-version"
              type="text"
              class="input input-bordered bg-base-200"
              value={config.azure_api_version}
              placeholder="2025-01-01-preview"
              oninput={(e) =>
                updateConfig(
                  "azure_api_version",
                  (e.target as HTMLInputElement).value
                )}
            />
            <div class="label">
              <span class="label-text-alt text-base-content/70"
                >API version for Azure OpenAI requests (e.g.,
                2025-01-01-preview)</span
              >
            </div>
          </div>
        </div>
      {:else if config.api_provider === "azure_translator"}
        <div class="form-control w-full">
          <label class="label" for="azure-translator-endpoint">
            <span class="label-text font-medium">Azure Translator Endpoint</span
            >
          </label>
          <input
            id="azure-translator-endpoint"
            type="url"
            class="input input-bordered bg-base-200 w-full min-w-0"
            value={config.azure_translator_endpoint}
            placeholder="https://api.cognitive.microsofttranslator.com"
            onblur={validateApiKey}
            oninput={(e) =>
              updateConfig(
                "azure_translator_endpoint",
                (e.target as HTMLInputElement).value
              )}
          />
          <div class="label">
            <span class="label-text-alt text-base-content/70 break-words">
              Azure Translator Service endpoint URL. Default is the global
              endpoint.
            </span>
          </div>
        </div>

        <div class="form-control w-full">
          <label class="label" for="azure-translator-key">
            <span class="label-text font-medium">Azure Translator API Key</span>
          </label>
          <div class="flex items-center gap-2">
            <input
              id="azure-translator-key"
              type="password"
              class="input input-bordered bg-base-200 flex-1 min-w-0"
              value={config.azure_translator_api_key}
              placeholder="Your Azure Translator API key"
              onblur={validateApiKey}
              oninput={(e) =>
                updateConfig(
                  "azure_translator_api_key",
                  (e.target as HTMLInputElement).value
                )}
            />
            <span class="w-8 h-8 flex items-center justify-center">
              {#if isValidatingApiKey}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if apiKeyValid === true}
                <span class="text-success text-lg">✓</span>
              {:else if apiKeyValid === false}
                <span class="text-error text-lg">✕</span>
              {:else}
                <span class="text-base-content/50 text-lg">?</span>
              {/if}
            </span>
          </div>
          <div class="label">
            <span class="label-text-alt text-base-content/70 break-words">
              Get your API key from the Azure portal under your Translator
              resource's "Keys and Endpoint" section.
            </span>
          </div>
        </div>

        <div class="form-control w-full">
          <label class="label" for="azure-translator-region">
            <span class="label-text font-medium">Azure Translator Region</span>
          </label>
          <input
            id="azure-translator-region"
            type="text"
            class="input input-bordered bg-base-200 w-full min-w-0"
            value={config.azure_translator_region}
            placeholder="e.g., eastus, westus2, uksouth"
            onblur={validateApiKey}
            oninput={(e) =>
              updateConfig(
                "azure_translator_region",
                (e.target as HTMLInputElement).value
              )}
          />
          <div class="label">
            <span class="label-text-alt text-base-content/70 break-words">
              Azure region where your Translator resource is located. Required
              for multi-service or regional resources. Optional for global
              single-service resources. Find this in the Azure portal under
              "Keys and Endpoint".
            </span>
          </div>
        </div>

        <!-- Fallback Provider for Alternative Translations -->
        <div class="form-control w-full">
          <label class="label" for="alternatives-fallback">
            <span class="label-text font-medium"
              >Alternative Translations Fallback Provider</span
            >
          </label>
          <select
            id="alternatives-fallback"
            class="select select-bordered bg-base-200 w-full min-w-0"
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
              {#each getAvailableFallbackModels() as model (model.value)}
                <option value={model.value}>{model.label}</option>
              {/each}
            {/if}
          </select>
          <div class="label">
            <span class="label-text-alt text-base-content/70 break-words">
              Azure Translator cannot generate alternative translations by
              itself. Configure a fallback AI model to enable alternative
              translations when using Azure Translator. Only models that you
              have configured and enabled in the Model Management section will
              appear in this dropdown.
              {#if getAvailableFallbackModels().length === 0}
                <br /><strong>No models available:</strong> Please configure and
                enable at least one AI model in the Model Management section first.
              {/if}
            </span>
          </div>
        </div>
      {:else if config.api_provider === "ollama"}
        <div class="form-control w-full">
          <label class="label" for="ollama-url">
            <span class="label-text font-medium">Ollama Server URL</span>
          </label>
          <div class="flex items-center gap-2">
            <input
              id="ollama-url"
              type="text"
              class="input input-bordered bg-base-200 flex-1"
              value={config.ollama_url || "http://localhost:11434"}
              placeholder="http://localhost:11434"
              onblur={validateApiKey}
              oninput={(e) =>
                updateConfig(
                  "ollama_url",
                  (e.target as HTMLInputElement).value
                )}
            />
            <span class="w-8 h-8 flex items-center justify-center">
              {#if isValidatingApiKey}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if apiKeyValid === true}
                <span class="text-success text-lg">✓</span>
              {:else if apiKeyValid === false}
                <span class="text-error text-lg">✕</span>
              {:else}
                <span class="text-base-content/50 text-lg">?</span>
              {/if}
            </span>
          </div>
          <div class="label">
            <span class="label-text-alt text-base-content/70">
              URL of your Ollama server. Make sure Ollama is running locally or
              provide the remote server URL.
            </span>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css -->
