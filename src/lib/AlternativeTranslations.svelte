<script lang="ts">
  import { invoke } from "./tauri"
  import { onMount } from "svelte"

  // Props
  interface Props {
    translatedText: string
    targetLanguage: string
    onTextUpdate: (newText: string) => void
  }

  let { translatedText, targetLanguage, onTextUpdate }: Props = $props()

  // State
  let selectedText = $state("")
  let isLoading = $state(false)
  let alternatives = $state<string[]>([])
  let showPopup = $state(false)
  let popupPosition = $state({ x: 0, y: 0 })
  let selectionRange: Range | null = null
  let popupElement = $state<HTMLElement | null>(null)
  let translatedTextElement = $state<HTMLElement | null>(null)

  // Handle text selection
  function handleTextSelection() {
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
      translatedTextElement &&
      translatedTextElement.contains(range.commonAncestorContainer)
    ) {
      selectedText = text
      selectionRange = range.cloneRange()
      showPopupNearSelection(range)
      loadAlternatives()
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

    try {
      const result = (await invoke("get_alternative_translations", {
        selectedText,
        targetLanguage,
      })) as { alternatives: string[] }
      alternatives = result.alternatives || []
      console.log("Loaded alternatives:", alternatives)
    } catch (error) {
      console.error("Failed to load alternatives:", error)
      alternatives = []
    } finally {
      isLoading = false
    }
  }

  // Replace selected text with alternative
  function replaceWithAlternative(alternative: string) {
    if (!selectionRange || !translatedTextElement) return

    // Create a new text node with the alternative
    const textNode = document.createTextNode(alternative)

    // Replace the selected range content
    selectionRange.deleteContents()
    selectionRange.insertNode(textNode)

    // Update the component's translated text
    const newText =
      translatedTextElement.innerText || translatedTextElement.textContent || ""
    onTextUpdate(newText)

    // Clear selection and hide popup
    window.getSelection()?.removeAllRanges()
    hidePopup()

    // Show brief animation/feedback
    showReplacementFeedback()
  }

  // Show replacement feedback (brief green flash)
  function showReplacementFeedback() {
    if (!translatedTextElement) return

    translatedTextElement.style.backgroundColor = "#d4edda"
    translatedTextElement.style.transition = "background-color 0.3s ease"

    setTimeout(() => {
      if (translatedTextElement) {
        translatedTextElement.style.backgroundColor = ""
        setTimeout(() => {
          if (translatedTextElement) {
            translatedTextElement.style.transition = ""
          }
        }, 300)
      }
    }, 200)
  }

  // Hide popup
  function hidePopup() {
    showPopup = false
    selectedText = ""
    alternatives = []
    selectionRange = null
  }

  // Handle clicks outside popup
  function handleClickOutside(event: MouseEvent) {
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
  class=""
  role="textbox"
  aria-readonly="true"
  tabindex="0"
>
  {#if translatedText}
    {#each translatedText.split("\n") as line, i (i)}
      {#if i > 0}<br />{/if}{line}
    {/each}
  {:else}
    <span class="">Translation will appear here...</span>
  {/if}
</div>

<!-- Alternative translations popup -->
{#if showPopup}
  <div bind:this={popupElement} class="">
    <div class="">
      <div class="">
        <h6 class="">
          <i class=""></i>
          Alternative Translations
        </h6>
      </div>
      <div class="">
        {#if isLoading}
          <div class="">
            <div
              class=""
              role="status"
            >
              <span class="">Loading alternatives...</span>
            </div>
            <div class="">Finding alternatives...</div>
          </div>
        {:else if alternatives.length > 0}
          <div class="">
            Click to replace "{selectedText}":
          </div>
          <div class="">
            {#each alternatives as alternative, index (index)}
              <button
                type="button"
                class=""
                onclick={() => replaceWithAlternative(alternative)}
                title="Click to replace with this alternative"
              >
                {alternative}
              </button>
            {/each}
          </div>
        {:else}
          <div class="">
            <i class=""></i>
            <div class="">No alternatives found</div>
          </div>
        {/if}
      </div>
      <div class="">
        <button
          type="button"
          class=""
          onclick={hidePopup}
        >
          <i class=""></i> Close
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* CSS moved to /src/app.css */
</style>
