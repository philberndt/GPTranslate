<script lang="ts">
  import { invoke, listen } from "../lib/tauri"
  import { onMount } from "svelte"
  import History from "../lib/History.svelte"
  import Settings from "../lib/Settings.svelte"
  import ModelSelector from "../lib/ModelSelector.svelte"
  import AlternativeTranslations from "../lib/AlternativeTranslations.svelte"
  import NoConfigScreen from "../lib/NoConfigScreen.svelte"

  import CompactLanguageDropdown from "../lib/CompactLanguageDropdown.svelte"
  import { LanguageManager, type Language } from "../lib/languages"
  import { isAnyProviderConfigured } from "../lib/utils/configUtils"

  // Import Heroicons
  import {
    ClockIcon,
    CogIcon,
    ArrowsRightLeftIcon,
    ClipboardDocumentIcon,
    TrashIcon,
    CheckCircleIcon,
  } from "heroicons-svelte/24/outline"
  let originalText = $state("")
  let translatedText = $state("")
  let detectedLanguage = $state("")
  let targetLanguage = $state("") // Track the target language used for translation
  let isTranslating = $state(false)
  let config = $state<any>(null)

  // Language management state
  let sourceLanguage = $state<Language>(LanguageManager.getAutoDetect())
  let primaryTargetLanguage = $state<Language>(
    LanguageManager.findByCode("en") ||
      LanguageManager.createCustomLanguage("English")
  )

  // Active view state: 'translate' | 'settings' | 'history'
  let activeView = $state<"translate" | "settings" | "history">("translate")
  let currentTheme = $state("auto")

  // Check if any API provider is configured
  let hasConfiguredProvider = $derived(isAnyProviderConfigured(config))
  // TEMP: override to disable welcome screen during testing. Set to true to force normal UI.
  const DISABLE_NO_CONFIG_SCREEN = true
  if (DISABLE_NO_CONFIG_SCREEN) {
    hasConfiguredProvider = true
  }

  // Notification system
  let showCopyNotification = $state(false)
  let notificationTimer: ReturnType<typeof setTimeout> | null = null // Debouncing variables
  let debounceTimer: ReturnType<typeof setTimeout> | null = null
  // Dynamic debounce – falls back to 500ms if config not yet loaded or invalid
  function getDebounceDelay() {
    const v = config?.auto_translate_debounce_ms
    if (typeof v === "number" && v >= 100 && v <= 2000) return v
    return 500
  }

  // Reset protection variables
  let resetDebounceTimer: ReturnType<typeof setTimeout> | null = null
  let lastTranslationTime = 0
  const RESET_PROTECTION_DELAY = 1000

  // Function to apply theme based on configuration
  function applyTheme(theme: string) {
    if (theme === "auto") {
      updateAutoTheme()
    } else {
      // Apply the specified theme
      document.documentElement.setAttribute("data-theme", theme)
    }
  }

  function updateAutoTheme() {
    const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches
    document.documentElement.setAttribute(
      "data-theme",
      isDark ? "dark" : "light"
    )
  }

  // Function to change theme and save it to config
  async function changeTheme(newTheme: string) {
    currentTheme = newTheme
    applyTheme(newTheme)

    // Save to configuration
    if (config) {
      const newConfig = { ...config, theme: newTheme }
      try {
        await invoke("save_config", { newConfig })
        config = newConfig
      } catch (error) {
        console.error("Failed to save theme:", error)
      }
    }
  }
  // Function to load/refresh config
  async function loadConfig() {
    try {
      const newConfig = await invoke("get_config")
      console.log("📥 Loaded new config:", newConfig)
      console.log("🔍 Current config reference:", $state.snapshot(config))
      config = newConfig
      console.log("✅ Config after assignment:", $state.snapshot(config))
      // Apply theme from config
      if (config && config.theme) {
        currentTheme = config.theme
        applyTheme(config.theme)
      } else {
        // Default to auto if no theme is set
        currentTheme = "auto"
        applyTheme("auto")
      }

      // Sync language state with config
      syncLanguageState()
    } catch (e) {
      console.error("Failed to load config:", e)
      // Fallback config for browser mode
      config = {
        api_provider: "openai",
        theme: "auto",
        openai_api_key: "",
        azure_api_key: "",
        azure_endpoint: "",
        azure_api_version: "2024-02-01",
        user_source_language: "English",
        user_target_language: "Spanish",
        target_language: "English",
        alternative_target_language: "Spanish",
        favorite_languages: ["en", "es", "fr", "de", "it"],
        auto_copy_translation: false,
        auto_copy_original: false,
        keep_app_open: true,
        startup_behavior: "tray",
        hotkey_enabled: true,
        hotkey_modifiers: ["ctrl", "shift"],
        hotkey_key: "t",
        available_models: {
          openai: [],
        },
      }
      currentTheme = "auto"
      applyTheme("auto")
    }
  }

  // Sync language state with config
  function syncLanguageState() {
    if (!config) return

    // Update source language
    if (config.user_source_language) {
      const lang = LanguageManager.search(
        config.user_source_language,
        false
      ).find(
        (l) =>
          l.english_name.toLowerCase() ===
          config.user_source_language.toLowerCase()
      )
      sourceLanguage =
        lang ||
        LanguageManager.createCustomLanguage(config.user_source_language)
    } else {
      sourceLanguage = LanguageManager.getAutoDetect()
    }

    // Update primary target language
    const primaryLang = LanguageManager.search(
      config.target_language,
      false
    ).find(
      (l) =>
        l.english_name.toLowerCase() === config.target_language.toLowerCase()
    )
    primaryTargetLanguage =
      primaryLang ||
      LanguageManager.createCustomLanguage(config.target_language)
  }

  // Handle source language change
  async function handleSourceLanguageChange(language: Language) {
    sourceLanguage = language

    // Update config
    const newConfig = {
      ...config,
      user_source_language:
        language.code === "auto" ? null : language.english_name,
    }

    try {
      await invoke("save_config", { newConfig: newConfig })
      config = newConfig
    } catch (e) {
      console.error("Failed to save source language:", e)
    }
  }

  // Handle primary target language change
  async function handlePrimaryTargetLanguageChange(language: Language) {
    primaryTargetLanguage = language

    // Update config
    const newConfig = {
      ...config,
      target_language: language.english_name,
    }

    try {
      await invoke("save_config", { newConfig: newConfig })
      config = newConfig
    } catch (e) {
      console.error("Failed to save primary target language:", e)
    }
  }

  // Handle model selection change
  async function handleModelChange(modelName: string, provider: string) {
    console.log(`🔄 Model changed to: ${modelName} (${provider})`)
    console.log("📋 Current config before reload:", $state.snapshot(config))
    // Reload config to ensure all components get the updated values
    await loadConfig()
    console.log("📋 Config after reload:", $state.snapshot(config))
  }
  onMount(() => {
    // Load config
    const initializeApp = async () => {
      await loadConfig() // Make sure config is loaded first

      // Listen for clipboard text from global shortcut
      await listen("clipboard-text", (event) => {
        originalText = event.payload as string
        console.log("Received clipboard text")
        if (config?.auto_translate_enabled && config?.auto_translate_on_paste) {
          console.log(
            "Auto translate (on paste) is enabled – triggering debounced translation"
          )
          debouncedTranslateText()
        } else {
          console.log(
            "Auto translate (on paste) disabled – not translating automatically"
          )
        }
      }) // Listen for reset detected language from global shortcut
      await listen("reset-detected-language", () => {
        // Debounce reset events and protect recent translations
        if (resetDebounceTimer !== null) {
          clearTimeout(resetDebounceTimer)
        }

        resetDebounceTimer = setTimeout(() => {
          const timeSinceLastTranslation = Date.now() - lastTranslationTime

          if (timeSinceLastTranslation < RESET_PROTECTION_DELAY) {
            console.log(
              `Ignoring reset request - too soon after translation (${timeSinceLastTranslation}ms)`
            )
            return
          }

          detectedLanguage = ""
          console.log("Detected language reset via global shortcut")
          resetDebounceTimer = null
        }, 200) // Small debounce to prevent rapid resets
      })
    }

    initializeApp()

    // Set up system theme change listener
    const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
    const handleThemeChange = () => {
      if (currentTheme === "auto") updateAutoTheme()
    }
    darkModeMediaQuery.addEventListener("change", handleThemeChange)

    // Set up global keyboard event listener
    document.addEventListener("keydown", handleKeydown)

    return () => {
      // Clean up the listeners when the component is destroyed
      darkModeMediaQuery.removeEventListener("change", handleThemeChange)
      document.removeEventListener("keydown", handleKeydown) // Clear any pending timers
      if (debounceTimer !== null) {
        clearTimeout(debounceTimer)
      }
      if (notificationTimer !== null) {
        clearTimeout(notificationTimer)
      }
      if (resetDebounceTimer !== null) {
        clearTimeout(resetDebounceTimer)
      }
    }
  })
  async function translateText() {
    if (!originalText.trim()) {
      // Clear translation and detected language when text is empty
      translatedText = ""
      detectedLanguage = ""
      targetLanguage = ""
      return
    }

    isTranslating = true
    try {
      const result = (await invoke("translate", {
        text: originalText,
      })) as any

      console.log("Translation result:", result)
      try {
        console.log("Translation result keys:", Object.keys(result || {}))
      } catch {
        // Ignore errors when logging result keys
      }

      // Support multiple possible backend response shapes
      const translated =
        (result &&
          (result.translated_text ??
            result.translatedText ??
            result.translation ??
            result.output ??
            result.text)) ??
        ""

      translatedText =
        typeof translated === "string" ? translated : JSON.stringify(translated)

      // Set target language from the response
      if (
        result?.target_language &&
        String(result.target_language).trim() !== ""
      ) {
        targetLanguage = result.target_language
        console.log("Set target language to:", result.target_language)
      } else {
        targetLanguage = ""
      }

      // Only set detectedLanguage if it's a valid, non-empty string and not "unknown" variants
      if (
        result?.detected_language &&
        String(result.detected_language).trim() !== "" &&
        String(result.detected_language).toLowerCase() !== "unknown" &&
        String(result.detected_language).toLowerCase() !== "unknowm"
      ) {
        detectedLanguage = result.detected_language
        console.log("Set detected language to:", result.detected_language)
      } else {
        detectedLanguage = ""
        console.log(
          "Cleared detected language, received:",
          result?.detected_language
        )
      }

      // Update last translation time to protect against immediate resets
      lastTranslationTime = Date.now()
    } catch (e) {
      console.error("Translation failed:", e)
      translatedText = "Translation failed: " + e
      detectedLanguage = ""
      targetLanguage = ""
    } finally {
      isTranslating = false
    }
  }

  // Debounced version of translateText
  function debouncedTranslateText() {
    if (!config?.auto_translate_enabled) return
    // Clear existing timer
    if (debounceTimer !== null) {
      clearTimeout(debounceTimer)
    }
    const delay = getDebounceDelay()
    debounceTimer = setTimeout(() => {
      translateText()
    }, delay)
  }

  // Handle keyboard events
  function handleKeydown(event: KeyboardEvent) {
    // Check for Ctrl+C to copy translated text
    if (event.ctrlKey && event.key === "c") {
      const target = event.target as HTMLElement
      const isInputElement =
        target.tagName === "INPUT" || target.tagName === "TEXTAREA"

      // Only intercept Ctrl+C if:
      // 1. We have translated text
      // 2. User is not in an input/textarea (to allow normal copy operation)
      // 3. Or user is in the readonly translated text area
      const isReadonlyTextarea =
        isInputElement && target.hasAttribute("readonly")

      if (translatedText.trim() && (!isInputElement || isReadonlyTextarea)) {
        event.preventDefault()
        copyToClipboard()
        showCopyNotificationMessage()
        console.log("Copied translated text to clipboard via Ctrl+C")
      }
    }
  }
  async function copyToClipboard() {
    try {
      await invoke("copy_to_clipboard", { text: translatedText })
    } catch (e) {
      console.error("Failed to copy to clipboard:", e)
    }
  }

  function showCopyNotificationMessage() {
    // Clear any existing timer
    if (notificationTimer !== null) {
      clearTimeout(notificationTimer)
    }

    showCopyNotification = true
    notificationTimer = setTimeout(() => {
      showCopyNotification = false
      notificationTimer = null
    }, 2000) // Show for 2 seconds
  }
  function clearText() {
    originalText = ""
    translatedText = ""
    detectedLanguage = ""
    targetLanguage = ""
  }
  function openSettings() {
    activeView = "settings"
  }

  function openHistory() {
    activeView = "history"
  }
  function closeHistory() {
    activeView = "translate"
  }

  function closeSettings() {
    activeView = "translate"
    // Reload config when settings are closed to pick up any changes
    loadConfig()
  }

  function handleTranslatedTextUpdate(newText: string) {
    translatedText = newText
    console.log("Translated text updated:", newText)
  }
</script>

<main class="mx-auto p-3 md:p-4 h-screen overflow-hidden bg-base-100">
  {#if !hasConfiguredProvider}
    <!-- Show fullscreen no-config message -->
    <NoConfigScreen onOpenSettings={openSettings} />
  {:else if activeView === "settings"}
    <div data-view="settings" class="flex flex-col h-full overflow-hidden">
      <Settings
        config={config || {
          api_provider: "openai",
          theme: "auto",
          openai_api_key: "",
          azure_api_key: "",
          azure_endpoint: "",
          azure_api_version: "2024-02-01",
          user_source_language: "English",
          user_target_language: "Spanish",
          target_language: "English",
          alternative_target_language: "Spanish",
          favorite_languages: ["en", "es", "fr", "de"],
          auto_copy_translation: false,
          auto_copy_original: false,
          keep_app_open: true,
          startup_behavior: "tray",
          hotkey_enabled: true,
          hotkey_modifiers: ["ctrl", "shift"],
          hotkey_key: "t",
          available_models: { openai: [] },
        }}
        onClose={closeSettings}
        theme={currentTheme}
        onThemeChange={changeTheme}
      />
    </div>
  {:else if activeView === "translate"}
    <div
      data-view="translate"
      class="flex flex-col h-full overflow-hidden space-y-1"
    >
      <!-- Translation Cards Grid -->
      <div
        class="grid grid-cols-2 items-stretch gap-1 md:gap-2 flex-1 overflow-hidden"
      >
        <!-- Source Text Card -->
        <div class="card bg-base-100 border border-base-300/50 h-full">
          <div class="card-body flex flex-col h-full p-2">
            <div class="card-title justify-between items-center mb-1">
              <div class="flex items-center gap-2">
                {#if detectedLanguage && sourceLanguage.code === "auto"}
                  <div class="badge badge-soft badge-info">
                    <span>{detectedLanguage}</span>
                  </div>
                {/if}
              </div>
              <div class="flex items-center gap-2">
                {#if config}
                  <CompactLanguageDropdown
                    selectedLanguage={sourceLanguage}
                    favoriteLanguages={config.favorite_languages || []}
                    includeAutoDetect={true}
                    onLanguageSelect={handleSourceLanguageChange}
                    label=""
                  />
                {/if}
              </div>
            </div>
            <textarea
              class="textarea textarea-bordered w-full flex-1 min-h-0 resize-none overflow-auto bg-base-200 border-base-300 focus:border-primary/30 focus:bg-base-200 p-3"
              bind:value={originalText}
              placeholder={`Enter text to translate or use ${config?.hotkey || "Ctrl+Alt+C"} to capture from clipboard...`}
              oninput={() => {
                if (
                  config?.auto_translate_enabled &&
                  config?.auto_translate_while_typing
                ) {
                  debouncedTranslateText()
                }
              }}
            ></textarea>
          </div>
        </div>

        <!-- Translation Result Card -->
        <div class="card bg-base-100 border border-base-300/50 h-full">
          <div class="card-body flex flex-col h-full p-2">
            <div class="card-title justify-between items-center mb-1">
              <div class="flex items-center gap-2">
                {#if config}
                  <CompactLanguageDropdown
                    selectedLanguage={primaryTargetLanguage}
                    favoriteLanguages={config.favorite_languages || []}
                    includeAutoDetect={false}
                    onLanguageSelect={handlePrimaryTargetLanguageChange}
                    label=""
                  />
                {/if}
              </div>
              <div class="flex items-center gap-2">
                {#if targetLanguage}
                  <span class="badge badge-soft badge-success"
                    >{targetLanguage}</span
                  >
                {/if}
              </div>
            </div>
            <div class="flex-1 min-h-0 overflow-hidden">
              <AlternativeTranslations
                {translatedText}
                targetLanguage={targetLanguage ||
                  primaryTargetLanguage.english_name}
                onTextUpdate={handleTranslatedTextUpdate}
                {isTranslating}
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Control Panel -->
      <div class="card bg-base-100 border border-base-300/50 shrink-0">
        <div class="card-body py-1">
          <div class="flex flex-wrap justify-between items-center gap-1">
            <!-- Navigation icons on the left -->
            <div class="flex gap-2">
              <button
                type="button"
                class="btn btn-circle btn-sm"
                onclick={openHistory}
                title="Translation History"
                aria-label="Open translation history"
              >
                <ClockIcon class="w-5 h-5" />
              </button>
              <button
                type="button"
                class="btn btn-circle btn-sm"
                onclick={openSettings}
                title="Settings"
                aria-label="Open settings"
              >
                <CogIcon class="w-5 h-5" />
              </button>
            </div>

            <!-- Model selector Dropup -->
            <div class="flex-shrink-0 max-w-[200px]">
              <ModelSelector {config} onModelChange={handleModelChange} />
            </div>

            <!-- Action buttons on the right -->
            <div
              class="btn-group"
              role="group"
              aria-label="Translation actions"
            >
              <button
                type="button"
                class="btn btn-soft btn-primary btn-sm"
                onclick={translateText}
                disabled={!originalText.trim() || isTranslating}
                title="Translate text"
              >
                <ArrowsRightLeftIcon class="w-5 h-5" />
                Translate
              </button>
              <button
                type="button"
                class="btn btn-soft btn-accent btn-sm"
                onclick={() => {
                  copyToClipboard()
                  showCopyNotificationMessage()
                }}
                disabled={!translatedText}
                title="Copy translation"
              >
                <ClipboardDocumentIcon class="w-5 h-5" />
                Copy
              </button>
              <button
                type="button"
                class="btn btn-soft btn-secondary btn-sm"
                onclick={clearText}
                title="Clear all text"
              >
                <TrashIcon class="w-5 h-5" />
                Clear
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  {:else if activeView === "history"}
    <div data-view="history" class="flex flex-col h-full overflow-hidden">
      <History onClose={closeHistory} theme={currentTheme} />
    </div>
  {/if}
</main>

<!-- Copy notification toast -->
{#if showCopyNotification}
  <div class="toast toast-top toast-end">
    <div class="alert alert-success" role="alert">
      <div class="flex items-center gap-2">
        <CheckCircleIcon class="w-5 h-5" />
        Translation copied to clipboard
      </div>
    </div>
  </div>
{/if}

<!-- Custom CSS goes in /src/styles.css */ -->
