<script lang="ts">
  import "bootstrap/dist/css/bootstrap.min.css"
  import "bootstrap-icons/font/bootstrap-icons.css"
  import "../app.css"
  import "bootstrap/dist/js/bootstrap.bundle.min.js"
  import { Modal } from "bootstrap"
  import { invoke, listen } from "../lib/tauri"
  import { onMount } from "svelte"
  import Settings from "../lib/Settings.svelte"
  import History from "../lib/History.svelte"
  import ModelSelector from "../lib/ModelSelector.svelte"
  import AlternativeTranslations from "../lib/AlternativeTranslations.svelte"

  import CompactLanguageDropdown from "../lib/CompactLanguageDropdown.svelte"
  import { LanguageManager, type Language } from "../lib/languages"
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
  const RESET_PROTECTION_DELAY = 1000 // Prevent resets for 1 second after translation

  // Function to apply theme based on configuration (case-insensitive, rely solely on Bootstrap color modes)
  function applyTheme(theme: string) {
    const t = (theme || "auto").toLowerCase()
    currentTheme = t
    if (t === "auto") {
      updateAutoTheme()
    } else if (t === "light" || t === "dark") {
      document.documentElement.setAttribute("data-bs-theme", t)
    } else {
      currentTheme = "auto"
      updateAutoTheme()
    }
  }
  function updateAutoTheme() {
    if (typeof window === "undefined") return
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)"
    ).matches
    if (prefersDark) {
      document.documentElement.setAttribute("data-bs-theme", "dark")
    } else {
      // Use default light variables by removing attribute
      document.documentElement.removeAttribute("data-bs-theme")
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
        applyTheme(config.theme)
      }

      // Sync language state with config
      syncLanguageState()
    } catch (e) {
      console.error("Failed to load config:", e)
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
      })) as {
        translated_text: string
        detected_language: string
        target_language: string
      }

      console.log("Translation result:", result)

      translatedText = result.translated_text

      // Set target language from the response
      if (result.target_language && result.target_language.trim() !== "") {
        targetLanguage = result.target_language
        console.log("Set target language to:", result.target_language)
      } else {
        targetLanguage = ""
      }

      // Only set detectedLanguage if it's a valid, non-empty string and not "unknown" variants
      if (
        result.detected_language &&
        result.detected_language.trim() !== "" &&
        result.detected_language.toLowerCase() !== "unknown" &&
        result.detected_language.toLowerCase() !== "unknowm"
      ) {
        detectedLanguage = result.detected_language
        console.log("Set detected language to:", result.detected_language)
      } else {
        detectedLanguage = ""
        console.log(
          "Cleared detected language, received:",
          result.detected_language
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
    const settingsModal = document.getElementById("settingsModal")
    if (settingsModal) {
      const modal = new Modal(settingsModal)
      // When settings modal is closed, reload config to pick up any changes
      settingsModal.addEventListener(
        "hidden.bs.modal",
        () => {
          loadConfig()
        },
        { once: true }
      )
      modal.show()
    }
  }

  function openHistory() {
    activeView = "history"
  }
  function closeHistory() {
    activeView = "translate"
  }

  function handleTranslatedTextUpdate(newText: string) {
    translatedText = newText
    console.log("Translated text updated:", newText)
  }
</script>

<main>
  {#if activeView === "translate"}
    <div data-view="translate">
      <div>
        <div>
          <div>
            <div>
              <div>
                <div>
                  <h5>Original Text</h5>
                </div>
                <div>
                  <div>
                    {#if config}
                      <CompactLanguageDropdown
                        selectedLanguage="{sourceLanguage}"
                        favoriteLanguages="{config.favorite_languages || []}"
                        includeAutoDetect="{true}"
                        onLanguageSelect="{handleSourceLanguageChange}"
                        label=""
                      />
                    {/if}
                    {#if detectedLanguage && sourceLanguage.code === "auto"}
                      <i></i>
                      <span>{detectedLanguage}</span>
                    {/if}
                  </div>
                </div>
              </div>
            </div>
            <div>
              <textarea
                bind:value="{originalText}"
                placeholder="{`Enter text to translate or use ${config?.hotkey || 'Ctrl+Alt+C'} to capture from clipboard...`}"

                oninput="{() => {
                  if (
                    config?.auto_translate_enabled &&
                    config?.auto_translate_while_typing
                  ) {
                    debouncedTranslateText()
                  }
                }}"
              ></textarea>
            </div>
          </div>
        </div>
        <div>
          <div>
            <div>
              <div>
                <div>
                  <div>
                    <span>To:</span>
                    {#if config}
                      <CompactLanguageDropdown
                        selectedLanguage="{primaryTargetLanguage}"
                        favoriteLanguages="{config.favorite_languages || []}"
                        includeAutoDetect="{false}"
                        onLanguageSelect="{handlePrimaryTargetLanguageChange}"
                        label=""
                      />
                    {/if}
                  </div>
                </div>
                <div class="">
                  <div class="">
                    {#if targetLanguage}
                      <span class="">{targetLanguage}</span>
                    {/if}
                    {#if isTranslating}
                      <div
                        class=""
                        role="status"
                      >
                        <span class="">Translating...</span>
                      </div>
                    {/if}
                  </div>
                </div>
              </div>
            </div>
            <div class="">
              <AlternativeTranslations
                {translatedText}
                targetLanguage="{targetLanguage ||
                  primaryTargetLanguage.english_name}"
                onTextUpdate="{handleTranslatedTextUpdate}"
              />
            </div>
          </div>
        </div>
      </div>
      <div class="">
        <!-- Navigation icons on the left -->
        <div class="">
          <button
            type="button"
            class=""
            onclick="{openHistory}"
            title="Translation History"
            aria-label="Open translation history"
          >
            <i></i>
          </button>
          <button
            type="button"

            onclick="{openSettings}"
            title="Settings"
            aria-label="Open settings"
          >
            <i class=""></i>
          </button>
        </div>
        <!-- Model selector -->
        <div class="">
          <ModelSelector {config} onModelChange="{handleModelChange}" />
        </div>

        <!-- Action buttons on the right -->
        <div class="" role="group" aria-label="Translation actions">
          <button
            type="button"

            onclick="{translateText}"
            disabled="{!originalText.trim() || isTranslating}"
            title="Translate text"
          >
            <i class=""></i>Translate
          </button>
          <button
            type="button"

            onclick="{() => {
              copyToClipboard()
              showCopyNotificationMessage()
            }}"
            disabled="{!translatedText}"
            title="Copy translation"
          >
            <i></i>Copy
          </button>
          <button
            type="button"
            class=""
            onclick="{clearText}"
            title="Clear all text"
          >
            <i class=""></i>Clear
          </button>
        </div>
      </div>
    </div>
  {:else if activeView === "history"}
    <div class="" data-view="history">
      <History onClose="{closeHistory}" theme="{currentTheme}" />
    </div>
  {/if}
</main>

<!-- Settings Modal -->
<Settings {config} />

<!-- Copy notification toast -->
{#if showCopyNotification}
  <div class="">
    <div class="" role="alert">
      <div class="">
        <i class=""></i>
        Translation copied to clipboard
      </div>
    </div>
  </div>
{/if}

<style>
  /* CSS moved to /src/app.css */
</style>
