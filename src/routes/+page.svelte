<script lang="ts">
  import "bootstrap-icons/font/bootstrap-icons.css";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Settings from "../lib/Settings.svelte";
  import History from "../lib/History.svelte";
  import ModelSelector from "../lib/ModelSelector.svelte";
  let originalText = $state("");
  let translatedText = $state("");
  let detectedLanguage = $state("");
  let isTranslating = $state(false);
  let config = $state<any>(null);
  let showSettings = $state(false);
  let showHistory = $state(false);
  let currentTheme = $state("auto");
  // Debouncing variables
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  const DEBOUNCE_DELAY = 500; // Reduced to 500ms for better responsiveness

  // Function to apply theme based on configuration
  function applyTheme(theme: string) {
    currentTheme = theme;
    if (theme === "auto") {
      // Remove any manually set theme classes
      document.documentElement.classList.remove("theme-light", "theme-dark");
      // Let the CSS media query handle it
    } else if (theme === "light") {
      document.documentElement.classList.remove("theme-dark");
      document.documentElement.classList.add("theme-light");
    } else if (theme === "dark") {
      document.documentElement.classList.remove("theme-light");
      document.documentElement.classList.add("theme-dark");
    }
  } // Function to load/refresh config
  async function loadConfig() {
    try {
      config = await invoke("get_config");
      // Apply theme from config
      if (config && config.theme) {
        applyTheme(config.theme);
      }
    } catch (e) {
      console.error("Failed to load config:", e);
    }
  }

  onMount(() => {
    // Load config
    const initializeApp = async () => {
      await loadConfig(); // Listen for clipboard text from global shortcut
      await listen("clipboard-text", (event) => {
        originalText = event.payload as string;
        // Use debounced translation to prevent conflicts with input events
        debouncedTranslateText();
      });

      // Listen for reset detected language from global shortcut
      await listen("reset-detected-language", () => {
        detectedLanguage = "";
        console.log("Detected language reset via global shortcut");
      });
    };

    initializeApp();

    // Set up system theme change listener
    const darkModeMediaQuery = window.matchMedia(
      "(prefers-color-scheme: dark)",
    );
    const handleThemeChange = () => {
      if (currentTheme === "auto") {
        // Only update when in auto mode
        document.documentElement.classList.remove("theme-light", "theme-dark");
      }
    };
    darkModeMediaQuery.addEventListener("change", handleThemeChange);

    // Set up global keyboard event listener
    document.addEventListener("keydown", handleKeydown);

    return () => {
      // Clean up the listeners when the component is destroyed
      darkModeMediaQuery.removeEventListener("change", handleThemeChange);
      document.removeEventListener("keydown", handleKeydown);

      // Clear any pending debounce timer
      if (debounceTimer !== null) {
        clearTimeout(debounceTimer);
      }
    };
  });
  async function translateText() {
    if (!originalText.trim()) {
      // Clear translation and detected language when text is empty
      translatedText = "";
      detectedLanguage = "";
      return;
    }

    isTranslating = true;
    try {
      const result = (await invoke("translate", {
        text: originalText,
      })) as {
        translated_text: string;
        detected_language: string;
      };
      translatedText = result.translated_text;
      detectedLanguage = result.detected_language;
    } catch (e) {
      console.error("Translation failed:", e);
      translatedText = "Translation failed: " + e;
    } finally {
      isTranslating = false;
    }
  }

  // Debounced version of translateText
  function debouncedTranslateText() {
    // Clear existing timer
    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    } // Set new timer
    debounceTimer = setTimeout(() => {
      translateText();
    }, DEBOUNCE_DELAY);
  }

  // Handle keyboard events
  function handleKeydown(event: KeyboardEvent) {
    // Check for Ctrl+C to copy translated text
    if (event.ctrlKey && event.key === "c") {
      const target = event.target as HTMLElement;
      const isInputElement =
        target.tagName === "INPUT" || target.tagName === "TEXTAREA";

      // Only intercept Ctrl+C if:
      // 1. We have translated text
      // 2. User is not in an input/textarea (to allow normal copy operation)
      // 3. Or user is in the readonly translated text area
      const isReadonlyTextarea =
        isInputElement && target.hasAttribute("readonly");

      if (translatedText.trim() && (!isInputElement || isReadonlyTextarea)) {
        event.preventDefault();
        copyToClipboard();
        console.log("Copied translated text to clipboard via Ctrl+C");
      }
    }
  }
  async function copyToClipboard() {
    try {
      await invoke("copy_to_clipboard", { text: translatedText });
    } catch (e) {
      console.error("Failed to copy to clipboard:", e);
    }
  }

  function clearText() {
    originalText = "";
    translatedText = "";
    detectedLanguage = "";
  }
  function openSettings() {
    console.log("Opening settings, showSettings before:", showSettings);
    showSettings = true;
    console.log("Opening settings, showSettings after:", showSettings);
  }

  function closeSettings() {
    showSettings = false;
    // Reload config after settings are closed
    invoke("get_config")
      .then((newConfig: any) => {
        config = newConfig;
        // Apply theme after settings are updated
        if (config && config.theme) {
          applyTheme(config.theme);
        }
      })
      .catch((e) => {
        console.error("Failed to reload config:", e);
      });
  }
  function openHistory() {
    showHistory = true;
  }

  function closeHistory() {
    showHistory = false;
  }
</script>

<main
  class="container"
  class:theme-light={currentTheme === "light"}
  class:theme-dark={currentTheme === "dark"}
>
  <div class="translation-container">
    <div class="text-panels">
      <div class="panel">
        <div class="panel-header">
          <h3>Original Text</h3>
          {#if detectedLanguage}
            <span class="language-tag">{detectedLanguage}</span>
          {/if}
        </div>
        <textarea
          bind:value={originalText}
          placeholder={`Enter text to translate or use ${config?.hotkey || "Ctrl+Alt+C"} to capture from clipboard...`}
          class="text-area"
          oninput={debouncedTranslateText}
        ></textarea>
      </div>

      <div class="panel">
        <div class="panel-header">
          <h3>Translation</h3>
          {#if isTranslating}
            <span class="loading">Translating...</span>
          {/if}
        </div>
        <textarea
          bind:value={translatedText}
          placeholder="Translation will appear here..."
          class="text-area"
          readonly
        ></textarea>
      </div>
    </div>
    <div class="controls">
      <!-- Navigation icons on the left -->
      <div class="nav-icons">
        <div
          class="nav-icon"
          role="button"
          tabindex="0"
          onclick={openHistory}
          onkeydown={(e) =>
            e.key === "Enter" || e.key === " " ? openHistory() : null}
          title="Translation History"
          aria-label="Open translation history"
        >
          <i class="bi bi-clock-history"></i>
        </div>
        <div
          class="nav-icon"
          role="button"
          tabindex="0"
          onclick={openSettings}
          onkeydown={(e) =>
            e.key === "Enter" || e.key === " " ? openSettings() : null}
          title="Settings"
          aria-label="Open settings"
        >
          <i class="bi bi-gear"></i>
        </div>
      </div>
      <!-- Model selector -->
      <div class="model-selector-container">
        <ModelSelector {config} />
      </div>

      <!-- Action buttons on the right -->
      <div class="action-buttons">
        <button
          onclick={translateText}
          disabled={!originalText.trim() || isTranslating}
          title="Translate text"
        >
          <i class="bi bi-globe"></i>Translate
        </button>
        <button
          onclick={copyToClipboard}
          disabled={!translatedText}
          title="Copy translation"
        >
          <i class="bi bi-clipboard"></i>Copy
        </button>
        <button onclick={clearText} class="clear-btn" title="Clear all text">
          <i class="bi bi-trash"></i>Clear
        </button>
      </div>
    </div>
  </div>
</main>

{#if showSettings}
  <Settings {config} onClose={closeSettings} onConfigChange={loadConfig} />
{/if}

{#if showHistory}
  <History onClose={closeHistory} theme={currentTheme} />
{/if}

<style>
  :root {
    font-family:
      "Segoe UI",
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f3f3f3;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  /* Global reset to prevent scrollbars */
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    overflow: hidden;
  }

  :global(#app) {
    height: 100vh;
    overflow: hidden;
  }
  .container {
    margin: 0;
    padding: 20px;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    box-sizing: border-box;
    position: relative;
  }
  .translation-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0px;
    overflow: hidden;
    min-height: 0;
    height: 100%;
  }
  .text-panels {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    flex: 1;
    overflow: hidden;
    min-height: 300px;
    max-height: calc(100vh - 100px);
  }

  .panel {
    display: flex;
    flex-direction: column;
    border: 1px solid #ddd;
    border-radius: 8px;
    background: white;
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: #f8f9fa;
    border-bottom: 1px solid #ddd;
  }

  .panel-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #333;
  }
  .language-tag {
    background: #379df1;
    color: white;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .loading {
    color: #666;
    font-style: italic;
    font-size: 0.9rem;
  }
  .text-area {
    flex: 1;
    border: none;
    padding: 16px;
    font-family: inherit;
    font-size: 14px;
    line-height: 1.5;
    resize: none;
    outline: none;
    background: transparent;
    min-height: 200px;
    height: 100%;
  }

  .text-area:focus {
    background: #fafbfc;
  }

  .text-area[readonly] {
    background: #f8f9fa;
    color: #495057;
  }
  .controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0;
    flex-shrink: 0;
  }

  .nav-icons {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .model-selector-container {
    flex: 1;
    display: flex;
    justify-content: center;
    margin: 0 16px;
  }

  .action-buttons {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .nav-icon {
    color: #666;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
  }

  .nav-icon i {
    font-size: 20px;
  }
  .nav-icon:hover {
    color: #379df1;
    transform: scale(1.1);
  }

  .action-buttons {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }
  button {
    border-radius: 6px;
    border: 1px solid #ddd;
    padding: 10px 16px;
    font-size: 14px;
    font-weight: 500;
    font-family: inherit;
    color: #333;
    background-color: #ffffff;
    transition: all 0.2s;
    cursor: pointer;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  button i {
    font-size: 16px;
  }
  button:hover:not(:disabled) {
    border-color: #379df1;
    background-color: #f8fdff;
    transform: translateY(-1px);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
  }

  button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .clear-btn {
    background-color: #ff6b6b;
    color: white;
    border-color: #ff6b6b;
  }

  .clear-btn:hover:not(:disabled) {
    background-color: #ff5252;
    border-color: #ff5252;
  } /* Dark theme styles - Apply for both system preference and manual setting */
  @media (prefers-color-scheme: dark) {
    :root:not(.theme-light) {
      color: #f6f6f6;
      background-color: #202020;
    }
    :root:not(.theme-light) .nav-icon {
      color: #ccc;
    }
    :root:not(.theme-light) .nav-icon:hover {
      color: #379df1;
    }

    :root:not(.theme-light) .panel {
      background: #2d2d2d;
      border-color: #444;
    }

    :root:not(.theme-light) .panel-header {
      background: #333;
      border-color: #444;
    }

    :root:not(.theme-light) .panel-header h3 {
      color: #f6f6f6;
    }

    :root:not(.theme-light) .text-area {
      color: #f6f6f6;
    }

    :root:not(.theme-light) .text-area:focus {
      background: #333;
    }

    :root:not(.theme-light) .text-area[readonly] {
      background: #2a2a2a;
      color: #ccc;
    }

    :root:not(.theme-light) button {
      color: #f6f6f6;
      background-color: #2d2d2d;
      border-color: #444;
    }
    :root:not(.theme-light) button:hover:not(:disabled) {
      background-color: #3a3a3a;
      border-color: #379df1;
    }

    :root:not(.theme-light) .clear-btn {
      background-color: #d63384;
      border-color: #d63384;
    }

    :root:not(.theme-light) .clear-btn:hover:not(:disabled) {
      background-color: #c02653;
    }
  }
  /* Manual dark theme - always apply regardless of system preference */
  .theme-dark {
    color: #f6f6f6;
    background-color: #202020;
  }
  .theme-dark .nav-icon {
    color: #ccc;
  }
  .theme-dark .nav-icon:hover {
    color: #379df1;
  }

  .theme-dark .panel {
    background: #2d2d2d;
    border-color: #444;
  }

  .theme-dark .panel-header {
    background: #333;
    border-color: #444;
  }

  .theme-dark .panel-header h3 {
    color: #f6f6f6;
  }

  .theme-dark .text-area {
    color: #f6f6f6;
  }

  .theme-dark .text-area:focus {
    background: #333;
  }

  .theme-dark .text-area[readonly] {
    background: #2a2a2a;
    color: #ccc;
  }

  .theme-dark button {
    color: #f6f6f6;
    background-color: #2d2d2d;
    border-color: #444;
  }
  .theme-dark button:hover:not(:disabled) {
    background-color: #3a3a3a;
    border-color: #379df1;
  }

  .theme-dark .clear-btn {
    background-color: #d63384;
    border-color: #d63384;
  }

  .theme-dark .clear-btn:hover:not(:disabled) {
    background-color: #c02653;
  }
  /* Manual light theme - always apply regardless of system preference */
  .theme-light {
    color: #333;
    background-color: #ffffff;
  }
  .theme-light .nav-icon {
    color: #333;
  }
  .theme-light .nav-icon:hover {
    color: #379df1;
  }

  .theme-light .panel {
    background: #ffffff;
    border-color: #ddd;
  }

  .theme-light .panel-header {
    background: #f8f8f8;
    border-color: #ddd;
  }

  .theme-light .panel-header h3 {
    color: #333;
  }

  .theme-light .text-area {
    color: #333;
  }

  .theme-light .text-area:focus {
    background: #ffffff;
  }

  .theme-light .text-area[readonly] {
    background: #f8f8f8;
    color: #666;
  }

  .theme-light button {
    color: #333;
    background-color: #ffffff;
    border-color: #ddd;
  }
  .theme-light button:hover:not(:disabled) {
    background-color: #f8fdff;
    border-color: #379df1;
  }

  .theme-light .clear-btn {
    background-color: #ff6b6b;
    color: white;
    border-color: #ff6b6b;
  }

  .theme-light .clear-btn:hover:not(:disabled) {
    background-color: #ff5252;
    border-color: #ff5252;
  }

  @media (max-width: 768px) {
    .text-panels {
      grid-template-columns: 1fr;
    }

    .controls {
      flex-direction: column;
      align-items: stretch;
    }

    button {
      width: 100%;
    }
  }
</style>
