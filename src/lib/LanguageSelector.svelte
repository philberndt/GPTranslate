<script lang="ts">
  import {
    LanguageManager,
    type Language,
    type LanguageSelectionEvent,
  } from "./languages"
  import { tick } from "svelte"

  interface Props {
    selectedLanguage: Language
    includeAutoDetect?: boolean
    onLanguageSelect: (event: LanguageSelectionEvent) => void
    placeholder?: string
    disabled?: boolean
    allowCustomLanguages?: boolean
  }

  let {
    selectedLanguage,
    includeAutoDetect = false,
    onLanguageSelect,
    placeholder = "Search languages...",
    disabled = false,
    allowCustomLanguages = true,
  }: Props = $props()

  let searchInput: HTMLInputElement
  let dropdownElement = $state<HTMLElement>()
  let isOpen = $state(false)
  let searchQuery = $state("")
  let selectedIndex = $state(-1)

  // Search results
  const searchResults = $derived.by(() => {
    return LanguageManager.search(searchQuery, includeAutoDetect).slice(0, 10) // Limit to 10 results for performance
  })

  // Show custom option if no exact matches and custom languages are allowed
  const showCustomOption = $derived.by(() => {
    if (!allowCustomLanguages || !searchQuery.trim()) return false
    return !searchResults.some(
      (lang) =>
        lang.english_name.toLowerCase() === searchQuery.toLowerCase().trim()
    )
  })

  // Display value for the input
  const displayValue = $derived.by(() => {
    if (isOpen) return searchQuery
    return LanguageManager.formatDisplayName(selectedLanguage, false)
  })

  function openDropdown() {
    if (disabled) return
    isOpen = true
    searchQuery = ""
    selectedIndex = -1
    tick().then(() => {
      searchInput?.focus()
      positionDropdown()
    })
  }

  function closeDropdown() {
    isOpen = false
    searchQuery = ""
    selectedIndex = -1
  }

  function selectLanguage(language: Language, isCustom = false) {
    onLanguageSelect({ language, isCustom })
    closeDropdown()
  }

  function createCustomLanguage() {
    if (!searchQuery.trim()) return
    const customLang = LanguageManager.createCustomLanguage(searchQuery.trim())
    selectLanguage(customLang, true)
  }

  function handleInputKeydown(event: KeyboardEvent) {
    const totalItems = searchResults.length + (showCustomOption ? 1 : 0)

    switch (event.key) {
      case "Escape":
        event.preventDefault()
        closeDropdown()
        break

      case "Enter":
        event.preventDefault()
        if (selectedIndex >= 0 && selectedIndex < searchResults.length) {
          selectLanguage(searchResults[selectedIndex])
        } else if (selectedIndex === searchResults.length && showCustomOption) {
          createCustomLanguage()
        }
        break

      case "ArrowDown":
        event.preventDefault()
        selectedIndex = Math.min(selectedIndex + 1, totalItems - 1)
        break

      case "ArrowUp":
        event.preventDefault()
        selectedIndex = Math.max(selectedIndex - 1, -1)
        break
    }
  }

  function handleInputBlur(event: FocusEvent) {
    // Delay closing to allow clicks on dropdown items
    setTimeout(() => {
      if (!dropdownElement?.contains(event.relatedTarget as Node)) {
        closeDropdown()
      }
    }, 150)
  }

  function positionDropdown() {
    if (!dropdownElement || !searchInput) return

    const inputRect = searchInput.getBoundingClientRect()
    const dropdownRect = dropdownElement.getBoundingClientRect()
    const viewportHeight = window.innerHeight

    // Position below input by default
    let top = inputRect.bottom + 2

    // If dropdown would go off-screen, position above input
    if (top + dropdownRect.height > viewportHeight - 20) {
      top = inputRect.top - dropdownRect.height - 2
    }

    dropdownElement.style.top = `${top}px`
    dropdownElement.style.left = `${inputRect.left}px`
    dropdownElement.style.width = `${inputRect.width}px`
  }

  // Update dropdown position when window resizes
  function handleWindowResize() {
    if (isOpen) {
      positionDropdown()
    }
  }
</script>

<svelte:window onresize={handleWindowResize} />

<div class:disabled>
  <div>
    <input
      bind:this={searchInput}
      type="text"
      value={displayValue}
      {placeholder}
      {disabled}
      readonly={!isOpen}
      onclick={openDropdown}
      oninput={(e) => (searchQuery = e.currentTarget.value)}
      onkeydown={handleInputKeydown}
      onblur={handleInputBlur}
    />

    <button
      type="button"
      onclick={openDropdown}
      {disabled}
      aria-label="Open language selector"
    >
    </button>
  </div>

  {#if isOpen}
    <div bind:this={dropdownElement}>
      {#if searchResults.length === 0 && !showCustomOption}
        <div>No languages found</div>
      {:else}
        {#each searchResults as language, index (language.code)}
          <button
            type="button"
            class:active={index === selectedIndex}
            onclick={() => selectLanguage(language)}
          >
            <div>
              <div>{language.english_name}</div>
              {#if language.native_name !== language.english_name && language.code !== "auto"}
                <div>{language.native_name}</div>
              {/if}
            </div>
          </button>
        {/each}

        {#if showCustomOption}
          <div></div>
          <button
            type="button"
            class:active={selectedIndex === searchResults.length}
            onclick={createCustomLanguage}
          >
            Add "{searchQuery}" as custom language
          </button>
        {/if}
      {/if}
    </div>
  {/if}
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
