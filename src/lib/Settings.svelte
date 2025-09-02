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
  const version = pkg.version

  // Tab management - keeping a state for initial setup

  // Model Config interface
  interface ModelConfig {
    name: string
    display_name: string
    provider: string
    is_enabled: boolean
    description?: string
  }
  let config = $state({
    api_provider: "",
    openai_api_key: "",
    azure_endpoint: "",
    azure_api_key: "",
    azure_api_version: "",
    azure_deployment_name: "",
    azure_translator_endpoint: "",
    azure_translator_api_key: "",
    azure_translator_region: "",
    ollama_url: "http://localhost:11434",
    model: "",
    available_models: {} as Record<string, ModelConfig[]>,
    target_language: "English",
    alternative_target_language: "Norwegian",
    favorite_languages: [] as string[],
    user_source_language: null as string | null,
    auto_start: true,
    hotkey: "Ctrl+Alt+C",
    theme: "auto",
    minimize_to_tray: true,
    custom_prompt: "",
    reasoning_effort: "medium" as string | null,
    alternatives_fallback_provider: null as string | null,
    auto_translate_enabled: true,
    auto_translate_debounce_ms: 500,
    auto_translate_on_paste: true,
    auto_translate_while_typing: true,
  })

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
  let { config: configProp }: { config: any } = $props()
  onMount(async () => {
    console.log("Settings component mounted")
    // Use the config prop if available, otherwise load from backend
    if (configProp) {
      config = { ...configProp }
      // Ensure available_models exists and has default structure
      if (!config.available_models) {
        config.available_models = {}
      }
    } else {
      try {
        config = (await invoke("get_config")) as any
        // Ensure available_models exists and has default structure
        if (!config.available_models) {
          config.available_models = {}
        }
      } catch (e) {
        console.error("Failed to load config:", e)
      }
    }

    // Initialize Bootstrap tabs
    const triggerTabList = document.querySelectorAll(
      '#settingsTab button[data-bs-toggle="tab"]'
    )
    if (triggerTabList.length > 0) {
      triggerTabList.forEach((triggerEl) => {
        new Tab(triggerEl)
      })
    }
  })

  // Configuration change handler
  function handleConfigChange(updates: any) {
    config = { ...config, ...updates }
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
    if (confirm("Are you sure you want to reset all settings to defaults?")) {
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
        favorite_languages: [],
        user_source_language: null,
        auto_start: true,
        hotkey: "Ctrl+Alt+C",
        theme: "auto",
        minimize_to_tray: true,
        custom_prompt:
          "Translate the given text from {detected_language} to {target_language} accurately while preserving the meaning, tone, and nuance of the original content.\n\n# Additional Details\n- Ensure the translation retains the context, cultural meaning, tone, formal/informal style, and any idiomatic expressions.\n- Do **not** alter names, technical terms, or specific formatting unless required for grammatical correctness in the target language.\n- If the detected language is the same as the target language, choose the most appropriate alternative language for translation.\n\n# Output Format\nThe translation output should be provided as valid JSON containing 'detected_language' and 'translated_text' fields.\n\n# Notes\n- Ensure punctuation and capitalization match the norms of the target language.\n- When encountering idiomatic expressions, adapt them to equivalent phrases in the target language rather than direct word-for-word translation.\n- For ambiguous content, aim for the most contextually appropriate meaning.\n- Take into consideration the whole text and what it is about.",
        reasoning_effort: "medium",
        azure_translator_endpoint: "",
        azure_translator_api_key: "",
        azure_translator_region: "",
        alternatives_fallback_provider: null,
        auto_translate_enabled: true,
        auto_translate_debounce_ms: 500,
        auto_translate_on_paste: true,
        auto_translate_while_typing: true,
      }
      apiKeyValid = null
    }
  }

  // Helper function to automatically add Azure model when extracted from endpoint
  async function autoAddAzureModel(deploymentName: string) {
    try {
      // Check if the model already exists
      if (!config.available_models.azure_openai) {
        config.available_models.azure_openai = []
      }

      const existingModel = config.available_models.azure_openai.find(
        (model) => model.name === deploymentName
      )

      if (!existingModel) {
        const newModel: ModelConfig = {
          name: deploymentName,
          display_name: deploymentName,
          provider: "azure_openai",
          is_enabled: true,
          description: "Auto-added from endpoint URL",
        }

        config.available_models.azure_openai.push(newModel)
        console.log(`✓ Auto-added Azure model: ${deploymentName}`)

        // Auto-save the model addition
        await saveModelChanges()
      }
    } catch (e) {
      console.error("Failed to auto-add Azure model:", e)
    }
  }

  // ...existing code...
</script>

<!-- Settings Modal -->
<div
  class=""
  id="settingsModal"
  tabindex="-1"
  aria-labelledby="settingsModalLabel"
  aria-hidden="true"
>
  <div class="">
    <div class="">
      <div class="">
        <div class="">
          <AppIcon size="{32}" className="me-3" />
          <h1 class="" id="settingsModalLabel">
            Settings
          </h1>
        </div>
        <button
          type="button"
          class=""
          data-bs-dismiss="modal"
          aria-label="Close settings"
        ></button>
      </div>

      <div class="">
        <!-- Tab Navigation with darker background -->
        <div class="">
          <ul class="" id="settingsTab" role="tablist">
            <li class="" role="presentation">
              <button
                class=""
                id="api-tab"
                data-bs-toggle="tab"
                data-bs-target="#api-tab-pane"
                type="button"
                role="tab"
                aria-controls="api-tab-pane"
                aria-selected="true"
              >
                <i class=""></i>
                API Configuration
              </button>
            </li>
            <li class="" role="presentation">
              <button
                class=""
                id="models-tab"
                data-bs-toggle="tab"
                data-bs-target="#models-tab-pane"
                type="button"
                role="tab"
                aria-controls="models-tab-pane"
                aria-selected="false"
              >
                <i class=""></i>
                Model Management
              </button>
            </li>
            <li class="" role="presentation">
              <button
                class=""
                id="languages-tab"
                data-bs-toggle="tab"
                data-bs-target="#languages-tab-pane"
                type="button"
                role="tab"
                aria-controls="languages-tab-pane"
                aria-selected="false"
              >
                <i class=""></i>
                Languages
              </button>
            </li>
            <li class="" role="presentation">
              <button
                class=""
                id="behavior-tab"
                data-bs-toggle="tab"
                data-bs-target="#behavior-tab-pane"
                type="button"
                role="tab"
                aria-controls="behavior-tab-pane"
                aria-selected="false"
              >
                <i class=""></i>
                App Behavior
              </button>
            </li>
            <li class="" role="presentation">
              <button
                class=""
                id="about-tab"
                data-bs-toggle="tab"
                data-bs-target="#about-tab-pane"
                type="button"
                role="tab"
                aria-controls="about-tab-pane"
                aria-selected="false"
              >
                <i class=""></i>
                About
              </button>
            </li>
          </ul>
        </div>

        <!-- Content area in a card -->
        <div class="">
          <div class="">
            <div class="">
              <div class="" id="settingsTabContent">
                <div
                  class=""
                  id="api-tab-pane"
                  role="tabpanel"
                  aria-labelledby="api-tab"
                  tabindex="0"
                >
                  <!-- API Configuration Tab -->
                  <ApiConfiguration
                    {config}
                    {isValidatingApiKey}
                    {apiKeyValid}
                    {azureEndpointInfo}
                    onConfigChange="{handleConfigChange}"
                    {onApiProviderChange}
                    {onAzureEndpointChange}
                    {validateApiKey}
                  />
                </div>
                <div
                  class=""
                  id="models-tab-pane"
                  role="tabpanel"
                  aria-labelledby="models-tab"
                  tabindex="0"
                >
                  <!-- Model Management Tab -->
                  <ModelManagement
                    {config}
                    availableModels="{config.available_models}"
                    onConfigChange="{handleConfigChange}"
                    onModelAdd="{handleModelAdd}"
                    onModelRemove="{handleModelRemove}"
                    onModelToggle="{handleModelToggle}"
                  />
                </div>
                <div
                  class=""
                  id="languages-tab-pane"
                  role="tabpanel"
                  aria-labelledby="languages-tab"
                  tabindex="0"
                >
                  <!-- Languages -->
                  <LanguagesTab
                    {config}
                    onConfigUpdate="{async (newConfig) => {
                      config = { ...config, ...newConfig }
                      await saveSettings()
                    }}"
                  />
                </div>
                <div
                  class=""
                  id="behavior-tab-pane"
                  role="tabpanel"
                  aria-labelledby="behavior-tab"
                  tabindex="0"
                >
                  <!-- App Behavior -->
                  <AppBehavior {config} onConfigChange="{handleConfigChange}" />
                </div>
                <div
                  class=""
                  id="about-tab-pane"
                  role="tabpanel"
                  aria-labelledby="about-tab"
                  tabindex="0"
                >
                  <!-- About -->
                  <AboutTab {version} />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="">
        <SettingsFooter
          {isSaving}
          {saveMessage}
          onSave="{saveSettings}"
          onReset="{resetToDefaults}"
        />
      </div>
    </div>
  </div>
</div>

<style>
  /* CSS moved to /src/app.css */
</style>
