<script lang="ts">
  import { invoke } from "./tauri"

  // Import Heroicons
  import {
    ChevronDownIcon,
    CheckIcon,
    InformationCircleIcon,
  } from "heroicons-svelte/24/outline"

  interface ModelConfig {
    name: string
    display_name: string
    provider: string
    is_enabled: boolean
    description?: string
  }

  interface Config {
    provider: string
    model: string
    available_models: Record<string, ModelConfig[]>
    openai_api_key: string
    azure_api_key: string
    azure_translator_api_key: string
    azure_translator_endpoint: string
    azure_translator_region: string
    ollama_url: string
    azure_deployment_name?: string
  }
  interface Props {
    config: Config | null
    onModelChange?: (model: string, provider: string) => void
  }

  let { config, onModelChange }: Props = $props()

  let isOpen = $state(false)
  let selectedModel = $state("")

  // Update selectedModel when config changes
  $effect(() => {
    console.log(
      "ModelSelector $effect triggered with config:",
      $state.snapshot(config)
    )
    if (config) {
      let newSelectedModel = ""
      // Special case for Azure AI Translator - no model needed
      if (config.provider === "azure_translator") {
        newSelectedModel = ""
      } else if (config.model && config.model.trim()) {
        newSelectedModel = config.model
      } else if (
        config.provider === "azure_openai" &&
        config.azure_deployment_name
      ) {
        newSelectedModel = config.azure_deployment_name
      } else {
        const firstEnabled = (
          config.available_models?.[config.provider] || []
        ).find((m) => m.is_enabled)
        if (firstEnabled) newSelectedModel = firstEnabled.name
      }
      console.log(
        "ModelSelector updating selectedModel from",
        selectedModel,
        "to",
        newSelectedModel
      )
      selectedModel = newSelectedModel
    }
  })

  function getProviderIcon(provider: string) {
    // Updated to match the actual SVG icons from files
    switch (provider) {
      case "openai":
        return {
          viewBox: "0 0 16 16",
          path: "M14.949 6.547a3.94 3.94 0 0 0-.348-3.273 4.11 4.11 0 0 0-4.4-1.934A4.1 4.1 0 0 0 8.423.2 4.15 4.15 0 0 0 6.305.086a4.1 4.1 0 0 0-1.891.948 4.04 4.04 0 0 0-1.158 1.753 4.1 4.1 0 0 0-1.563.679A4 4 0 0 0 .554 4.72a3.99 3.99 0 0 0 .502 4.731 3.94 3.94 0 0 0 .346 3.274 4.11 4.11 0 0 0 4.402 1.933c.382.425.852.764 1.377.995.526.231 1.095.35 1.67.346 1.78.002 3.358-1.132 3.901-2.804a4.1 4.1 0 0 0 1.563-.68 4 4 0 0 0 1.14-1.253 3.99 3.99 0 0 0-.506-4.716m-6.097 8.406a3.05 3.05 0 0 1-1.945-.694l.096-.054 3.23-1.838a.53.53 0 0 0 .265-.455v-4.49l1.366.778q.02.011.025.035v3.722c-.003 1.653-1.361 2.992-3.037 2.996m-6.53-2.75a2.95 2.95 0 0 1-.36-2.01l.095.057L5.29 12.09a.53.53 0 0 0 .527 0l3.949-2.246v1.555a.05.05 0 0 1-.022.041L6.473 13.3c-1.454.826-3.311.335-4.15-1.098m-.85-6.94A3.02 3.02 0 0 1 3.07 3.949v3.785a.51.51 0 0 0 .262.451l3.93 2.237-1.366.779a.05.05 0 0 1-.048 0L2.585 9.342a2.98 2.98 0 0 1-1.113-4.094zm11.216 2.571L8.747 5.576l1.362-.776a.05.05 0 0 1 .048 0l3.265 1.86a3 3 0 0 1 1.173 1.207 2.96 2.96 0 0 1-.27 3.2 3.05 3.05 0 0 1-1.36.997V8.279a.52.52 0 0 0-.276-.445m1.36-2.015-.097-.057-3.226-1.855a.53.53 0 0 0-.53 0L6.249 6.153V4.598a.04.04 0 0 1 .019-.04L9.533 2.7a3.07 3.07 0 0 1 3.257.139c.474.325.843.778 1.066 1.303.223.526.289 1.103.191 1.664zM5.503 8.575 4.139 7.8a.05.05 0 0 1-.026-.037V4.049c0-.57.166-1.127.476-1.607s.752-.864 1.275-1.105a3.08 3.08 0 0 1 3.234.41l-.096.054-3.23 1.838a.53.53 0 0 0-.265.455zm.742-1.577 1.758-1 1.762 1v2l-1.755 1-1.762-1z",
        }
      case "azure_openai":
        return {
          viewBox: "0 0 16 16",
          path: "M7.462 0H0v7.19h7.462zM16 0H8.538v7.19H16zM7.462 8.211H0V16h7.462zm8.538 0H8.538V16H16z",
        }
      case "azure_translator":
        return {
          viewBox: "0 0 64 64",
          path: "M50.916 36.445c-2.917-.056-5.787.736-8.263 2.28v3.991a11.9737 11.9737 0 0 1 7.932-2.972c3.343 0 5.013 2.065 5.01 6.195l-7.29 1.023c-5.35.742-8.025 3.385-8.025 7.931-.04.947.123 1.891.477 2.77.355.878.893 1.671 1.578 2.325a7.988 7.988 0 0 0 5.685 1.924 8.1532 8.1532 0 0 0 4.369-1.109 8.1558 8.1558 0 0 0 3.112-3.26h.094v3.8h3.893V45.514c0-6.046-2.857-9.069-8.572-9.069Zm4.67 15.005a7.241 7.241 0 0 1-1.889 5.142 6.2497 6.2497 0 0 1-2.169 1.535c-.825.351-1.716.519-2.612.494a4.9035 4.9035 0 0 1-3.397-1.125 3.705 3.705 0 0 1-.963-1.311 3.7022 3.7022 0 0 1-.31-1.597 3.8755 3.8755 0 0 1 1.375-3.397 9.5452 9.5452 0 0 1 4.102-1.355l5.863-.806v2.42Zm-2.543-18.957 6.526-6.708c.709-.705.45-1.321-.679-1.321h-2.9a.849.849 0 0 1-.601-.248.8525.8525 0 0 1-.248-.601c.093-4.326-1.121-16.338-17.532-16.746a.85.85 0 0 0-.849.85v4.246c0 .117.024.233.071.341.046.108.115.204.201.284.086.08.188.141.299.18.111.038.229.053.346.044 1.549-.335 3.154-.299 4.686.104 1.533.402 2.948 1.16 4.132 2.213a10.0675 10.0675 0 0 1 2.684 3.843c.58 1.474.804 3.064.654 4.641a.8484.8484 0 0 1-.849.849h-2.658c-1.367 0-1.596.425-.684 1.321l6.051 6.708c.079.104.181.188.298.246.117.058.246.088.377.088s.26-.03.377-.088a.8454.8454 0 0 0 .298-.246Zm-39.325 4.42-6.53 6.708c-.709.709-.45 1.321.684 1.321h2.895c.226 0 .442.089.601.248.159.16.249.376.249.601-.107 4.327 1.103 16.338 17.514 16.746.225 0 .441-.09.6-.249a.8494.8494 0 0 0 .249-.6v-4.246c0-.117-.024-.233-.07-.341a.8456.8456 0 0 0-.201-.283.8552.8552 0 0 0-.297-.18.8432.8432 0 0 0-.345-.045c-1.551.335-3.159.299-4.694-.105a10.07 10.07 0 0 1-4.137-2.221 10.0826 10.0826 0 0 1-2.682-3.855 10.0813 10.0813 0 0 1-.643-4.651c0-.111.022-.222.065-.325a.8358.8358 0 0 1 .184-.275.8358.8358 0 0 1 .275-.184.8455.8455 0 0 1 .325-.065h2.684c1.363 0 1.592-.425.679-1.32l-6.042-6.679a.839.839 0 0 0-.3-.252.8458.8458 0 0 0-.763 0 .839.839 0 0 0-.3.252Zm8.959-23.489c2.285.403 4.388 1.51 6.012 3.167a8.4896 8.4896 0 0 1 1.941 2.816c.449 1.061.678 2.202.673 3.354a8.5075 8.5075 0 0 1-.705 3.346 8.5006 8.5006 0 0 1-1.968 2.798 12.8967 12.8967 0 0 1-8.127 3.15l-1.21-2.73c2.414.069 4.788-.617 6.794-1.962a5.5147 5.5147 0 0 0 2.398-4.573A6.0235 6.0235 0 0 0 28 20.429a6.019 6.019 0 0 0-1.374-1.982 8.6929 8.6929 0 0 0-4.641-2.433 27.5372 27.5372 0 0 1-4.832 9.765 9.37 9.37 0 0 0 1.113 1.941l-2.276 1.579c-.35-.442-.66-.915-.926-1.414a9.818 9.818 0 0 1-6.131 2.697c-.641.018-1.28-.092-1.878-.323a4.8383 4.8383 0 0 1-1.608-1.023 5.3072 5.3072 0 0 1-1.112-1.844 5.3048 5.3048 0 0 1-.285-2.135c.045-2.257.843-4.435 2.268-6.186a14.0818 14.0818 0 0 1 5.8-4.505c.068-2.264.141-3.848.22-4.751-2.36.091-4.224.136-5.592.136l-.454-2.628c1.469.053 3.551.031 6.246-.068.187-1.946.485-3.88.892-5.792l2.628.387-.726 5.205c3.686-.24 7.357-.679 10.997-1.316l.556 2.547c-3.904.659-7.84 1.116-11.791 1.368a29.993 29.993 0 0 0-.251 3.94c1.3-.328 2.634-.504 3.975-.522.348 0 .687 0 1.027.033.242-.704.406-1.433.488-2.173l2.692.386c-.069.709-.185 1.412-.348 2.106Zm-8.823 11.77a29.5187 29.5187 0 0 1-1.617-7.732 10.4795 10.4795 0 0 0-3.877 3.396 7.479 7.479 0 0 0-1.431 4.246c-.046.382-.016.769.089 1.139.104.37.28.716.518 1.018.33.337.777.535 1.249.552a6.788 6.788 0 0 0 2.22-.586 8.2776 8.2776 0 0 0 2.849-2.033Zm5.269-9.384a15.72 15.72 0 0 0-4.144.624c.055 2.106.384 4.195.977 6.216a25.442 25.442 0 0 0 3.167-6.84Z",
        }
      case "ollama":
        return {
          viewBox: "0 0 24 24",
          path: "M7.905 1.09c.216.085.411.225.588.41.295.306.544.744.734 1.263.191.522.315 1.1.362 1.68a5.054 5.054 0 0 1 2.049-.636l.051-.004c.87-.07 1.73.087 2.48.474.101.053.2.11.297.17.05-.569.172-1.134.36-1.644.19-.52.439-.957.733-1.264a1.67 1.67 0 0 1 .589-.41c.257-.1.53-.118.796-.042.401.114.745.368 1.016.737.248.337.434.769.561 1.287.23.934.27 2.163.115 3.645l.053.04.026.019c.757.576 1.284 1.397 1.563 2.35.435 1.487.216 3.155-.534 4.088l-.018.021.002.003c.417.762.67 1.567.724 2.4l.002.03c.064 1.065-.2 2.137-.814 3.19l-.007.01.01.024c.472 1.157.62 2.322.438 3.486l-.006.039a.651.651 0 0 1-.747.536.648.648 0 0 1-.54-.742c.167-1.033.01-2.069-.48-3.123a.643.643 0 0 1 .04-.617l.004-.006c.604-.924.854-1.83.8-2.72-.046-.779-.325-1.544-.8-2.273a.644.644 0 0 1 .18-.886l.009-.006c.243-.159.467-.565.58-1.12a4.229 4.229 0 0 0-.095-1.974c-.205-.7-.58-1.284-1.105-1.683-.595-.454-1.383-.673-2.38-.61a.653.653 0 0 1-.632-.371c-.314-.665-.772-1.141-1.343-1.436a3.288 3.288 0 0 0-1.772-.332c-1.245.099-2.343.801-2.67 1.686a.652.652 0 0 1-.61.425c-1.067.002-1.893.252-2.497.703-.522.39-.878.935-1.066 1.588a4.07 4.07 0 0 0-.068 1.886c.112.558.331 1.02.582 1.269l.008.007c.212.207.257.53.109.785-.36.622-.629 1.549-.673 2.44-.05 1.018.186 1.902.719 2.536l.016.019a.643.643 0 0 1 .095.69c-.576 1.236-.753 2.252-.562 3.052a.652.652 0 0 1-1.269.298c-.243-1.018-.078-2.184.473-3.498l.014-.035-.008-.012a4.339 4.339 0 0 1-.598-1.309l-.005-.019a5.764 5.764 0 0 1-.177-1.785c.044-.91.278-1.842.622-2.59l.012-.026-.002-.002c-.293-.418-.51-.953-.63-1.545l-.005-.024a5.352 5.352 0 0 1 .093-2.49c.262-.915.777-1.701 1.536-2.269.06-.045.123-.09.186-.132-.159-1.493-.119-2.73.112-3.67.127-.518.314-.95.562-1.287.27-.368.614-.622 1.015-.737.266-.076.54-.059.797.042zm4.116 9.09c.936 0 1.8.313 2.446.855.63.527 1.005 1.235 1.005 1.94 0 .888-.406 1.58-1.133 2.022-.62.375-1.451.557-2.403.557-1.009 0-1.871-.259-2.493-.734-.617-.47-.963-1.13-.963-1.845 0-.707.398-1.417 1.056-1.946.668-.537 1.55-.849 2.485-.849zm0 .896a3.07 3.07 0 0 0-1.916.65c-.461.37-.722.835-.722 1.25 0 .428.21.829.61 1.134.455.347 1.124.548 1.943.548.799 0 1.473-.147 1.932-.426.463-.28.7-.686.7-1.257 0-.423-.246-.89-.683-1.256-.484-.405-1.14-.643-1.864-.643zm.662 1.21.004.004c.12.151.095.37-.056.49l-.292.23v.446a.375.375 0 0 1-.376.373.375.375 0 0 1-.376-.373v-.46l-.271-.218a.347.347 0 0 1-.052-.49.353.353 0 0 1 .494-.051l.215.172.22-.174a.353.353 0 0 1 .49.051zm-5.04-1.919c.478 0 .867.39.867.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zm8.706 0c.48 0 .868.39.868.871a.87.87 0 0 1-.868.871.87.87 0 0 1-.867-.87.87.87 0 0 1 .867-.872zM7.44 2.3l-.003.002a.659.659 0 0 0-.285.238l-.005.006c-.138.189-.258.467-.348.832-.17.692-.216 1.631-.124 2.782.43-.128.899-.208 1.404-.237l.01-.001.019-.034c.046-.082.095-.161.148-.239.123-.771.022-1.692-.253-2.444-.134-.364-.297-.65-.453-.813a.628.628 0 0 0-.107-.09L7.44 2.3zm9.174.04-.002.001a.628.628 0 0 0-.107.09c-.156.163-.32.45-.453.814-.29.794-.387 1.776-.23 2.572l.058.097.008.014h.03a5.184 5.184 0 0 1 1.466.212c.086-1.124.038-2.043-.128-2.722-.09-.365-.21-.643-.349-.832l-.004-.006a.659.659 0 0 0-.285-.239h-.004z",
        }
      default:
        return {
          viewBox: "0 0 16 16",
          path: "M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z",
        }
    }
  }

  function isProviderConfigured(provider: string): boolean {
    if (!config) return false

    switch (provider) {
      case "openai":
        return config.openai_api_key.length > 0
      case "azure_openai":
        return config.azure_api_key.length > 0
      case "azure_translator":
        return !!(
          config.azure_translator_api_key &&
          config.azure_translator_api_key.length > 0 &&
          config.azure_translator_endpoint &&
          config.azure_translator_endpoint.length > 0
        )
      case "ollama":
        return config.ollama_url?.length > 0
      default:
        return false
    }
  }
  async function selectModel(provider: string, modelName: string) {
    console.log(
      `🔄 selectModel called with provider: ${provider}, modelName: ${modelName}`
    )
    console.log("📋 Current config:", $state.snapshot(config))
    console.log("🎯 Current selectedModel:", selectedModel)

    try {
      if (config) {
        // Update selectedModel immediately for visual feedback
        console.log(
          `👁️ Updating selectedModel from ${selectedModel} to ${modelName}`
        )
        selectedModel = modelName

        // Create a new config object to avoid mutating props
        const newConfig = {
          ...config,
          provider: provider,
          model: modelName,
          // Keep azure_deployment_name in sync with model for Azure
          azure_deployment_name:
            provider === "azure_openai" ? modelName : (
              config.azure_deployment_name || ""
            ),
        }

        console.log("💾 Saving new config:", newConfig)
        await invoke("save_config", { newConfig: newConfig })
        console.log("✅ Config saved successfully")

        // Call onModelChange to let parent handle config updates
        if (onModelChange) {
          console.log("📢 Calling onModelChange callback")
          onModelChange(modelName, provider)
        } else {
          console.log("⚠️ No onModelChange callback provided")
        }

        isOpen = false
      }
    } catch (e) {
      console.error("Failed to save model selection:", e)
    }
  }

  function getCurrentModelDisplayName(): string {
    if (!config || !config.available_models) {
      return selectedModel && selectedModel.trim().length > 0 ?
          selectedModel
        : "Select Model"
    }

    // Special case for Azure AI Translator - check this FIRST
    if (config.provider === "azure_translator") {
      return "Azure AI Translator"
    }

    const key =
      selectedModel ||
      config.model ||
      (config.provider === "azure_openai" ?
        config.azure_deployment_name || ""
      : "")

    // For Azure AI Translator, don't use the key logic
    if (key === "azure_translator") {
      return "Azure AI Translator"
    }

    // Try current provider first
    const providerModels = config.available_models[config.provider] || []
    let currentModel = providerModels.find((m) => m.name === key)

    // If not found, search across all providers by name
    if (!currentModel && key) {
      for (const models of Object.values(config.available_models)) {
        const found = models.find((m) => m.name === key)
        if (found) {
          currentModel = found
          break
        }
      }
    }

    const name = currentModel?.display_name || key
    return name && name.trim().length > 0 ? name : "Select Model"
  }

  function getEnabledModelsForProvider(provider: string): ModelConfig[] {
    if (!config?.available_models) return []
    return config.available_models[provider]?.filter((m) => m.is_enabled) || []
  }

  function hasEnabledModels(provider: string): boolean {
    // Azure AI Translator doesn't use traditional models but should still be shown
    if (provider === "azure_translator") {
      return isProviderConfigured(provider)
    }
    return getEnabledModelsForProvider(provider).length > 0
  }

  function hasAnyModels(): boolean {
    if (!config?.available_models) return false
    // Check if Azure AI Translator is configured
    if (isProviderConfigured("azure_translator")) return true
    return Object.values(config.available_models).some(
      (models) => models.length > 0
    )
  }
</script>

<div class="dropdown relative">
  <button
    class="btn btn-soft btn-sm gap-2"
    type="button"
    onclick={() => (isOpen = !isOpen)}
    aria-expanded={isOpen}
    title="Select AI Model"
  >
    {#if config}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="12"
        height="12"
        fill="currentColor"
        viewBox={getProviderIcon(config.provider).viewBox}
      >
        <path d={getProviderIcon(config.provider).path}></path>
      </svg>
    {/if}
    <span class="text-sm truncate max-w-32">{getCurrentModelDisplayName()}</span
    >
    <ChevronDownIcon class="w-4 h-4" />
  </button>

  {#if isOpen}
    <div
      class="dropdown-content menu bg-base-100 rounded-box w-80 max-h-96 overflow-y-auto shadow-lg border border-base-300/50 z-50 absolute top-full left-0 mt-1"
    >
      {#if config?.available_models}
        {#if hasAnyModels()}
          <!-- Include all providers that have models OR are special cases like Azure AI Translator -->
          {#each [...Object.keys(config.available_models), "azure_translator"].filter((provider, index, arr) => arr.indexOf(provider) === index) as provider (provider)}
            {#if hasEnabledModels(provider)}
              <div class="menu-title">
                <div class="flex items-center gap-2 px-3 py-2">
                  <span class="w-4 h-4">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="14"
                      height="14"
                      fill="currentColor"
                      viewBox={getProviderIcon(provider).viewBox}
                    >
                      <path d={getProviderIcon(provider).path}></path>
                    </svg>
                  </span>
                  <span class="font-semibold text-sm"
                    >{provider === "azure_openai" ? "Azure OpenAI"
                    : provider === "azure_translator" ? "Azure AI Translator"
                    : provider
                        .replace("_", " ")
                        .replace(/\b\w/g, (l) => l.toUpperCase())}</span
                  >

                  {#if !isProviderConfigured(provider)}
                    <span class="badge badge-warning badge-xs"
                      >Not Configured</span
                    >
                  {/if}
                </div>
              </div>

              {#if isProviderConfigured(provider)}
                {#if provider === "azure_translator"}
                  <!-- Azure AI Translator doesn't use models -->
                  <li>
                    <button
                      class="flex items-center justify-between p-3 text-left hover:bg-base-200 transition-colors duration-200"
                      type="button"
                      onclick={(e) => {
                        e.stopPropagation()
                        console.log(`🖱️ Azure AI Translator selected`)
                        selectModel(provider, "")
                      }}
                    >
                      <div class="flex-1">
                        <div class="font-medium text-sm">
                          Azure AI Translator
                        </div>
                        <div class="text-xs text-base-content/60">
                          Neural machine translation service
                        </div>
                      </div>
                      {#if config.provider === provider}
                        <CheckIcon class="w-4 h-4 text-success" />
                      {/if}
                    </button>
                  </li>
                {:else}
                  {#each getEnabledModelsForProvider(provider) as model (model.name)}
                    <li>
                      <button
                        class="flex items-center justify-between p-3 text-left hover:bg-base-200 transition-colors duration-200"
                        type="button"
                        onclick={(e) => {
                          e.stopPropagation()
                          console.log(
                            `🖱️ Model button clicked: ${model.name} from provider ${provider}`
                          )
                          selectModel(provider, model.name)
                        }}
                      >
                        <div class="flex-1">
                          <div class="font-medium text-sm">
                            {model.display_name}
                          </div>
                          {#if model.description}
                            <div class="text-xs text-base-content/60">
                              {model.description}
                            </div>
                          {/if}
                        </div>
                        {#if config.provider === provider && config.model === model.name}
                          <CheckIcon class="w-4 h-4 text-success" />
                        {/if}
                      </button>
                    </li>
                  {/each}
                {/if}
              {:else}
                <li>
                  <div class="p-3 text-xs text-base-content/60">
                    Configure this provider in settings to use its models.
                  </div>
                </li>
              {/if}
            {/if}
          {/each}
        {:else}
          <li>
            <div class="p-4 text-center">
              <InformationCircleIcon class="w-8 h-8 mx-auto mb-2 text-info" />
              <div class="font-medium text-sm">No models configured</div>
              <div class="text-xs text-base-content/60 mt-1">
                Please add models in Settings → Model Management to get started.
              </div>
            </div>
          </li>
        {/if}
      {:else}
        <li>
          <div class="p-4 text-center">
            <div
              class="loading loading-spinner loading-md"
              role="status"
              aria-hidden="true"
            ></div>
            <div class="text-sm mt-2">Loading models...</div>
          </div>
        </li>
      {/if}
    </div>
  {/if}
</div>

<!-- Click outside to close -->
{#if isOpen}
  <div
    class="fixed inset-0 z-40"
    onclick={() => (isOpen = false)}
    onkeydown={(e) => {
      if (e.key === "Escape") isOpen = false
    }}
    role="button"
    tabindex="-1"
    aria-label="Close model selector"
  ></div>
{/if}

<!-- Custom CSS goes in /src/styles.css */ -->
