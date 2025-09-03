<script lang="ts">
  import { invoke } from "./tauri"
  import { onMount } from "svelte"
  import AppIcon from "./AppIcon.svelte"
  import ApiConfiguration from "./ApiConfiguration.svelte"
  import ModelManagement from "./ModelManagement.svelte"
  import AppBehavior from "./AppBehavior.svelte"
  import LanguagesTab from "./LanguagesTab.svelte"
  import AboutTab from "./AboutTab.svelte"
  import SettingsFooter from "./SettingsFooter.svelte"
  import pkg from "../../package.json"
  import { XMarkIcon, WrenchScrewdriverIcon, CpuChipIcon, GlobeAltIcon, CogIcon, InformationCircleIcon } from "heroicons-svelte/24/outline"

  // Props
  let { config, onClose, theme, onThemeChange } = $props<{
    config: any
    onClose: () => void
    theme: string
    onThemeChange: (theme: string) => void
  }>()

  const version = pkg.version

  // Tab management
  let activeTab = $state("api")

  function setActiveTab(tab: string) {
    activeTab = tab
  }

  // Model Config interface
  interface ModelConfig {
    name: string
    display_name: string
    provider: string
    is_enabled: boolean
    description?: string
  }

  let isValidatingApiKey = $state(false)
  let apiKeyValid = $state<boolean | null>(null)
  let isSaving = $state(false)
  let saveMessage = $state("")
  let azureEndpointInfo = $state<{
    isValid: boolean
    type?: string
    deploymentDetected?: string
    apiVersionDetected?: string
  } | null>(null)

  onMount(async () => {
    console.log("Settings component mounted")
    // Ensure available_models exists and has default structure
    if (!config.available_models) {
      config.available_models = {}
    }
  })

  // Configuration change handler
  function handleConfigChange(updates: any) {
    // Update the parent config through callback or event
    Object.assign(config, updates)
  } // Model management functions
  async function handleModelAdd(provider: string, model: ModelConfig) {
    if (!config.available_models[provider]) {
      config.available_models[provider] = []
    }
    config.available_models[provider].push(model)
    // Auto-save and notify parent about config changes
    await saveModelChanges()
  }

  async function handleModelRemove(provider: string, modelIndex: number) {
    if (config.available_models[provider]) {
      config.available_models[provider].splice(modelIndex, 1)
    }
    // Auto-save and notify parent about config changes
    await saveModelChanges()
  }

  async function handleModelToggle(provider: string, modelIndex: number) {
    if (
      config.available_models[provider] &&
      config.available_models[provider][modelIndex]
    ) {
      config.available_models[provider][modelIndex].is_enabled =
        !config.available_models[provider][modelIndex].is_enabled
    }
    // Auto-save and notify parent about config changes
    await saveModelChanges()
  }

  // Helper function to save model changes without showing UI feedback
  async function saveModelChanges() {
    try {
      await invoke("save_config", { newConfig: config })
      // Notify parent component about config changes
    } catch (e) {
      console.error("Failed to save model changes:", e)
    }
  }
  async function onApiProviderChange() {
    // Clear validation when changing providers
    apiKeyValid = null
  }

  function parseAzureEndpoint(url: string): {
    baseUrl: string
    apiVersion: string
    deploymentName: string | null
    isModelsEndpoint: boolean
    isValid: boolean
    errorMessage?: string
  } {
    try {
      const parsedUrl = new URL(url)

      // Extract API version from query parameters
      const apiVersion = parsedUrl.searchParams.get("api-version")

      // Determine endpoint type based on hostname
      const isModelsEndpoint = parsedUrl.hostname.includes(
        "services.ai.azure.com"
      )
      const isCognitiveServicesEndpoint = parsedUrl.hostname.includes(
        "cognitiveservices.azure.com"
      )

      if (!isModelsEndpoint && !isCognitiveServicesEndpoint) {
        return {
          baseUrl: url,
          apiVersion: config.azure_api_version,
          deploymentName: null,
          isModelsEndpoint: false,
          isValid: false,
          errorMessage:
            "URL must be either a cognitiveservices.azure.com or services.ai.azure.com endpoint",
        }
      }

      // Extract deployment name for cognitive services endpoints
      let deploymentName: string | null = null
      if (isCognitiveServicesEndpoint) {
        const pathParts = parsedUrl.pathname
          .split("/")
          .filter((part) => part.length > 0)
        const deploymentIndex = pathParts.findIndex(
          (part) => part === "deployments"
        )

        if (deploymentIndex !== -1 && deploymentIndex + 1 < pathParts.length) {
          deploymentName = pathParts[deploymentIndex + 1]
        } else {
          // If no deployment in path, we can still extract from a full endpoint URL
          console.log(
            "No deployment found in path, cognitive services endpoint may need deployment name"
          )
        }
      }

      // Create base URL (without path and query parameters)
      const baseUrl = `${parsedUrl.protocol}//${parsedUrl.host}`

      return {
        baseUrl,
        apiVersion: apiVersion || config.azure_api_version,
        deploymentName,
        isModelsEndpoint,
        isValid: true,
      }
    } catch (e) {
      console.error("Error parsing Azure endpoint URL:", e)
      return {
        baseUrl: url,
        apiVersion: config.azure_api_version,
        deploymentName: null,
        isModelsEndpoint: false,
        isValid: false,
        errorMessage: `Invalid URL format: ${e instanceof Error ? e.message : "Unknown error"}`,
      }
    }
  }

  async function onAzureEndpointChange() {
    azureEndpointInfo = null // Reset info

    if (config.azure_endpoint) {
      try {
        const {
          baseUrl,
          apiVersion,
          deploymentName,
          isModelsEndpoint,
          isValid,
          errorMessage,
        } = parseAzureEndpoint(config.azure_endpoint)

        if (!isValid) {
          console.warn(`Invalid Azure endpoint: ${errorMessage}`)
          azureEndpointInfo = { isValid: false }
          return
        }

        // Update the config with parsed values
        config.azure_endpoint = baseUrl

        if (apiVersion) {
          config.azure_api_version = apiVersion
          console.log(`✓ Extracted API version: ${apiVersion}`)
        }

        if (isModelsEndpoint) {
          // For models endpoints, clear deployment name as it's not needed
          config.azure_deployment_name = ""
          console.log(
            "✓ Models API endpoint detected - deployment name cleared"
          )
        } else if (deploymentName) {
          // For cognitive services endpoints, use extracted deployment name
          config.azure_deployment_name = deploymentName
          console.log(`✓ Extracted deployment name: ${deploymentName}`)

          // Automatically add the model if it doesn't exist
          await autoAddAzureModel(deploymentName)
        } else {
          // Cognitive services endpoint but no deployment name in URL
          console.log(
            "⚠ Cognitive Services endpoint detected but no deployment name found in URL. You may need to specify it manually."
          )
        }

        // Set endpoint info for UI feedback
        azureEndpointInfo = {
          isValid: true,
          type: isModelsEndpoint ? "Models API" : "Cognitive Services",
          deploymentDetected: deploymentName || undefined,
          apiVersionDetected: apiVersion || undefined,
        }

        console.log(
          `✓ Endpoint type: ${isModelsEndpoint ? "Models API" : "Cognitive Services"}`
        )
        console.log(`✓ Base URL set to: ${baseUrl}`)
      } catch (e) {
        console.error("Failed to parse Azure endpoint:", e)
        azureEndpointInfo = { isValid: false }
      }
    }
    // Reset API key validation when endpoint changes
    apiKeyValid = null
  }

  async function validateApiKey() {
    if (
      (config.api_provider === "openai" && !config.openai_api_key) ||
      (config.api_provider === "azure_openai" &&
        (!config.azure_api_key || !config.azure_endpoint)) ||
      (config.api_provider === "azure_translator" &&
        (!config.azure_translator_api_key ||
          !config.azure_translator_endpoint)) ||
      (config.api_provider === "ollama" && !config.ollama_url)
    ) {
      return
    }

    isValidatingApiKey = true
    try {
      const isValid = (await invoke("validate_api_key", {
        apiProvider: config.api_provider,
        apiKey:
          config.api_provider === "openai" ? config.openai_api_key
          : config.api_provider === "azure_openai" ? config.azure_api_key
          : config.api_provider === "azure_translator" ?
            config.azure_translator_api_key
          : "", // Ollama doesn't need an API key
        endpoint:
          config.api_provider === "azure_openai" ? config.azure_endpoint
          : config.api_provider === "azure_translator" ?
            config.azure_translator_endpoint
          : config.api_provider === "ollama" ? config.ollama_url
          : null,
        apiVersion:
          config.api_provider === "azure_openai" ?
            config.azure_api_version
          : null,
        region:
          config.api_provider === "azure_translator" ?
            config.azure_translator_region || null
          : null,
      })) as boolean
      apiKeyValid = isValid
    } catch (e) {
      console.error("API key validation failed:", e)
      apiKeyValid = false
    } finally {
      isValidatingApiKey = false
    }
  }
  async function saveSettings() {
    isSaving = true
    saveMessage = ""
    try {
      await invoke("save_config", { newConfig: config })
      saveMessage = "Settings saved"
      // Notify parent component about config changes

      setTimeout(() => {
        saveMessage = ""
      }, 5000)
    } catch (e) {
      console.error("Failed to save settings:", e)
      saveMessage = "Failed to save settings: " + e
    } finally {
      isSaving = false
    }
  }

  function resetToDefaults() {
    // Temporarily simplified
    console.log("Reset to defaults clicked")
  }

  function autoAddAzureModel(_deploymentName: string) {
    throw new Error("Function not implemented.")
  }
</script>

<!-- Settings View -->
<div class="h-full bg-base-100 p-6 overflow-hidden">
  <div class="max-w-6xl mx-auto h-full flex flex-col overflow-hidden space-y-4">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <AppIcon size={32} className="" />
        <h1 class="text-2xl font-bold text-base-content">Settings</h1>
      </div>
      <button
        type="button"
        class="btn btn-soft btn-circle"
        onclick={onClose}
        aria-label="Close settings"
      >
        <XMarkIcon class="w-6 h-6" />
      </button>
    </div>

    <!-- Tab Navigation with Styled Tabs -->
    <div class="tabs tabs-lift mb-2 shrink-0" role="tablist">
      <button 
        role="tab" 
        class="tab {activeTab === 'api' ? 'tab-active' : ''}"
        onclick={() => setActiveTab('api')}
      >
        <WrenchScrewdriverIcon class="w-4 h-4 mr-2" />
        API Configuration
      </button>
      <button 
        role="tab" 
        class="tab {activeTab === 'models' ? 'tab-active' : ''}"
        onclick={() => setActiveTab('models')}
      >
        <CpuChipIcon class="w-4 h-4 mr-2" />
        Model Management
      </button>
      <button 
        role="tab" 
        class="tab {activeTab === 'languages' ? 'tab-active' : ''}"
        onclick={() => setActiveTab('languages')}
      >
        <GlobeAltIcon class="w-4 h-4 mr-2" />
        Languages
      </button>
      <button 
        role="tab" 
        class="tab {activeTab === 'behavior' ? 'tab-active' : ''}"
        onclick={() => setActiveTab('behavior')}
      >
        <CogIcon class="w-4 h-4 mr-2" />
        App Behavior
      </button>
      <button 
        role="tab" 
        class="tab {activeTab === 'about' ? 'tab-active' : ''}"
        onclick={() => setActiveTab('about')}
      >
        <InformationCircleIcon class="w-4 h-4 mr-2" />
        About
      </button>
    </div>

    <!-- Scrollable content area -->
    <div class="card bg-base-100 shadow-md border border-base-300/50 flex-1 min-h-0 overflow-auto">
      <!-- inner scroll area stays within card-body -->
      <div class="card-body">
        {#if activeTab === 'api'}
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
        {:else if activeTab === 'models'}
          <ModelManagement
            {config}
            availableModels={config.available_models}
            onConfigChange={handleConfigChange}
            onModelAdd={handleModelAdd}
            onModelRemove={handleModelRemove}
            onModelToggle={handleModelToggle}
          />
        {:else if activeTab === 'languages'}
          <LanguagesTab
            {config}
            onConfigUpdate={async (newConfig) => {
              config = { ...config, ...newConfig }
              await saveSettings()
            }}
          />
        {:else if activeTab === 'behavior'}
          <AppBehavior
            {config}
            onConfigChange={handleConfigChange}
            {theme}
            {onThemeChange}
          />
        {:else if activeTab === 'about'}
          <AboutTab {version} />
        {/if}
      </div>
    </div>


    <!-- Settings Footer (sticky) -->
    <div class="mt-4 sticky bottom-0 bg-base-100/90 backdrop-blur border-t border-base-300 z-10">
      <div class="py-3">
        <SettingsFooter
          {isSaving}
          {saveMessage}
          onSave={saveSettings}
          onReset={resetToDefaults}
        />
      </div>
    </div>
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
