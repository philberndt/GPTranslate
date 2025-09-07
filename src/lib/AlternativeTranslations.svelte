<script lang="ts">
  import { invoke } from "./tauri"
  import { onMount } from "svelte"

  // Import Heroicons
  import {
    LanguageIcon,
    XMarkIcon,
    ExclamationTriangleIcon,
  } from "heroicons-svelte/24/outline"

  // Props
  interface Props {
    translatedText: string
    targetLanguage: string
    onTextUpdate: (newText: string) => void
    isTranslating?: boolean
  }

  let {
    translatedText,
    targetLanguage,
    onTextUpdate,
    isTranslating = false,
  }: Props = $props()

  // State
  let selectedText = $state("")
  let isLoading = $state(false)
  let alternatives = $state<string[]>([])
  let error = $state<any>(null)
  let showPopup = $state(false)
  let popupPosition = $state({ x: 0, y: 0 })
  let selectionRange: Range | null = null
  let popupElement = $state<HTMLElement | null>(null)
  let translatedTextElement = $state<HTMLElement | null>(null)
  let overlayContainer = $state<HTMLElement | null>(null)
  // Declarative overlay rects to avoid direct DOM manipulation
  type OverlayRect = {
    left: number
    top: number
    width: number
    height: number
    id: number
  }
  let overlayRects = $state<OverlayRect[]>([])
  let overlayVisible = $state(false)
  let isReplacing = $state(false)
  let justSelected = $state(false)

  // Handle text selection
  function handleTextSelection(event: MouseEvent) {
    // Skip if we're in the middle of replacing text
    if (isReplacing) return

    // Ignore mouseup events that originate inside the alternatives popup
    if (popupElement && popupElement.contains(event.target as Node)) {
      return
    }

    const selection = window.getSelection()
    if (!selection || selection.rangeCount === 0) {
      hidePopup()
      return
    }

    const range = selection.getRangeAt(0)
    const text = selection.toString().trim()

    // Only proceed if text is selected and within our translated text area
    if (
      text &&
      text.length > 0 &&
      translatedTextElement &&
      translatedTextElement.contains(range.commonAncestorContainer)
    ) {
      selectedText = text
      selectionRange = range.cloneRange()
      justSelected = true
      showPopupNearSelection(range)
      loadAlternatives()
      console.log(`Selected text for alternatives: "${text}"`)

      // Reset justSelected flag after a short delay
      setTimeout(() => {
        justSelected = false
      }, 300)
    } else {
      hidePopup()
    }
  }

  // Position and show popup near selection
  function showPopupNearSelection(range: Range) {
    const rect = range.getBoundingClientRect()
    const viewportWidth = window.innerWidth
    const viewportHeight = window.innerHeight

    // Calculate popup position
    let x = rect.left + rect.width / 2
    let y = rect.bottom + 8

    // Adjust if popup would go off-screen
    const popupWidth = 300 // Estimated popup width
    const popupHeight = 200 // Estimated popup height

    if (x + popupWidth / 2 > viewportWidth) {
      x = viewportWidth - popupWidth / 2 - 10
    }
    if (x - popupWidth / 2 < 0) {
      x = popupWidth / 2 + 10
    }
    if (y + popupHeight > viewportHeight) {
      y = rect.top - popupHeight - 8
    }

    popupPosition = { x: x - popupWidth / 2, y }
    showPopup = true
  }

  // Load alternative translations
  async function loadAlternatives() {
    if (!selectedText || !targetLanguage) return

    isLoading = true
    alternatives = []
    error = null

    try {
      const result = (await invoke("get_alternative_translations", {
        selectedText,
        targetLanguage,
      })) as { alternatives: string[] }

      alternatives = result.alternatives || []
    } catch (err) {
      console.error("Failed to load alternatives:", err)
      alternatives = []
      error = err
    } finally {
      isLoading = false
    }
  }

  // Replace selected text with alternative
  function replaceWithAlternative(alternative: string, event?: MouseEvent) {
    // Prevent event propagation to avoid triggering text selection again
    if (event) {
      event.preventDefault()
      event.stopPropagation()
    }

    if (!selectedText || !translatedText) return

    // Set replacing flag to prevent selection handling
    isReplacing = true

    try {
      // Simple string replacement - more reliable with Svelte reactivity
      const newText = translatedText.replace(selectedText, alternative)

      // Show highlight BEFORE DOM/state changes and selection clearing
      showReplacementFeedback()

      // Update the component's translated text through the prop callback
      onTextUpdate(newText)

      console.log(`Replaced "${selectedText}" with "${alternative}"`)
    } catch (error) {
      console.error("Error during text replacement:", error)
    }

    // Now clear selection and hide popup
    window.getSelection()?.removeAllRanges()
    hidePopup()

    // Reset replacing flag after a short delay
    setTimeout(() => {
      isReplacing = false
    }, 100)
  }

  // Show replacement feedback (green flash on the replaced range; fallback to whole area)
  function showReplacementFeedback() {
    // Try precise range highlight first (rendered declaratively)
    if (translatedTextElement && selectionRange) {
      const rects = Array.from(selectionRange.getClientRects())
      const hostRect = translatedTextElement.getBoundingClientRect()
      if (rects.length > 0) {
        let idCounter = 0
        overlayRects = rects.map((r) => ({
          left:
            r.left - hostRect.left + (translatedTextElement?.scrollLeft || 0),
          top: r.top - hostRect.top + (translatedTextElement?.scrollTop || 0),
          width: r.width,
          height: r.height,
          id: idCounter++,
        }))
        overlayVisible = true
        // Fade out then clear
        setTimeout(() => {
          overlayVisible = false
          setTimeout(() => {
            overlayRects = []
          }, 260)
        }, 220)
        return
      }
    }

    // Fallback: flash whole element background
    if (!translatedTextElement) return
    translatedTextElement.style.backgroundColor = "rgba(34, 197, 94, 0.25)"
    translatedTextElement.style.transition = "background-color 300ms ease"
    setTimeout(() => {
      if (translatedTextElement) {
        translatedTextElement.style.backgroundColor = ""
        setTimeout(() => {
          if (translatedTextElement) translatedTextElement.style.transition = ""
        }, 320)
      }
    }, 200)
  }

  // Hide popup
  function hidePopup() {
    showPopup = false
    selectedText = ""
    alternatives = []
    selectionRange = null
    console.log("Alternative translations popup hidden")
  }

  // Handle clicks outside popup
  function handleClickOutside(event: MouseEvent) {
    // Don't handle click outside if we're in the middle of replacing text or just selected text
    if (isReplacing || justSelected) return

    if (
      showPopup &&
      popupElement &&
      !popupElement.contains(event.target as Node)
    ) {
      // Also check if click is within the translated text (to allow reselection)
      if (
        !translatedTextElement ||
        !translatedTextElement.contains(event.target as Node)
      ) {
        hidePopup()
        window.getSelection()?.removeAllRanges()
      }
    } else if (popupElement && popupElement.contains(event.target as Node)) {
      // Click happened inside the popup; don't let it cascade into selection logic elsewhere
      return
    }
  }

  // Handle escape key
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && showPopup) {
      hidePopup()
      window.getSelection()?.removeAllRanges()
    }
  }

  onMount(() => {
    // Add event listeners
    document.addEventListener("mouseup", handleTextSelection)
    document.addEventListener("click", handleClickOutside)
    document.addEventListener("keydown", handleKeydown)

    return () => {
      // Clean up event listeners
      document.removeEventListener("mouseup", handleTextSelection)
      document.removeEventListener("click", handleClickOutside)
      document.removeEventListener("keydown", handleKeydown)
    }
  })
</script>

<!-- Translated text area with selection support -->
<div
  bind:this={translatedTextElement}
  class="textarea textarea-bordered w-full h-full min-h-0 cursor-text p-3 whitespace-pre-wrap bg-base-200 border-base-300 focus:border-primary/30 focus:bg-base-200 overflow-auto relative"
  role="textbox"
  aria-readonly="true"
  tabindex="0"
>
  {#if translatedText}
    {#each (translatedText || "").split("\n") as line, i (i)}
      {#if i > 0}<br />{/if}{line}
    {/each}
  {:else}
    <span class="text-base-content/60 italic"
      >Translation will appear here...</span
    >
  {/if}

  <!-- Loading indicator overlay -->
  {#if isTranslating}
    <div
      class="absolute inset-0 flex items-center justify-center bg-base-200/80 z-10"
    >
      <div class="loading loading-ring loading-lg"></div>
    </div>
  {/if}
  <!-- Overlay container for highlight effects -->
  <div
    bind:this={overlayContainer}
    class="absolute inset-0 pointer-events-none z-40"
  >
    {#each overlayRects as r (r.id)}
      <div
        class="absolute rounded"
        style="
          left: {r.left}px;
          top: {r.top}px;
          width: {r.width}px;
          height: {r.height}px;
          background-color: rgba(34, 197, 94, 0.5);
          opacity: {overlayVisible ? 1 : 0};
          transition: opacity 250ms ease;
        "
      ></div>
    {/each}
  </div>
</div>

<!-- Alternative translations popup -->
{#if showPopup}
  <div
    bind:this={popupElement}
    class="card bg-base-100 border border-base-300/50 w-80 fixed z-50"
    style="left: {popupPosition.x}px; top: {popupPosition.y}px;"
  >
    <div class="card-body p-4">
      <div class="card-title text-base">
        <LanguageIcon class="w-5 h-5" />
        Alternative Translations
      </div>
      <div class="space-y-3">
        {#if isLoading}
          <div class="flex items-center gap-3">
            <div class="loading loading-spinner loading-sm" role="status">
              <span class="sr-only">Loading alternatives...</span>
            </div>
            <div class="text-sm">Finding alternatives...</div>
          </div>
        {:else if alternatives.length > 0}
          <div class="text-sm text-base-content/70">
            Click to replace "{selectedText}":
          </div>
          <div class="space-y-2 max-h-40 overflow-y-auto">
            {#each alternatives as alternative, index (index)}
              <button
                type="button"
                class="btn btn-soft btn-sm w-full text-left justify-start"
                onpointerdown={(e) => {
                  // Prevent the browser from altering selection before we replace
                  isReplacing = true
                  e.preventDefault()
                  e.stopPropagation()
                }}
                onclick={(event) => replaceWithAlternative(alternative, event)}
                title="Click to replace with this alternative"
              >
                {alternative}
              </button>
            {/each}
          </div>
        {:else}
          <div class="flex items-center gap-3 text-warning">
            <ExclamationTriangleIcon class="w-5 h-5" />
            <div class="text-sm">
              {#if error && typeof error === "string" && error.includes("Azure Translator cannot generate alternatives")}
                Azure Translator cannot generate alternatives. Please configure
                a fallback AI provider in Settings → API Configuration.
              {:else}
                No alternatives found
              {/if}
            </div>
          </div>
        {/if}
      </div>
      <div class="card-actions justify-end pt-2">
        <button type="button" class="btn btn-soft btn-sm" onclick={hidePopup}>
          <XMarkIcon class="w-4 h-4" />
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Custom CSS goes in /src/styles.css */ -->
