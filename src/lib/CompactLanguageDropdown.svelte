<script lang="ts">
  import { LanguageManager, type Language } from "./languages"

  interface Props {
    selectedLanguage: Language
    favoriteLanguages?: string[]
    includeAutoDetect?: boolean
    onLanguageSelect: (language: Language) => void
    label?: string
    disabled?: boolean
  }

  let {
    selectedLanguage,
    favoriteLanguages = [],
    includeAutoDetect = false,
    onLanguageSelect,
    label = "Language",
    disabled = false,
  }: Props = $props()

  // Get favorite languages and fallback to common suggestions
  const favorites = $derived.by(() => {
    const favLangs = favoriteLanguages
      .map((code) => LanguageManager.findByCode(code))
      .filter(Boolean) as Language[]

    if (favLangs.length === 0) {
      // Fallback to common languages if no favorites set
      return LanguageManager.getSuggestions()
    }
    return favLangs.slice(0, 5) // Limit to 5 for compact view
  })

  function handleLanguageChange(event: Event) {
    const target = event.target as HTMLSelectElement
    const selectedCode = target.value
    const language = LanguageManager.findByCode(selectedCode)

    if (language) {
      onLanguageSelect(language)
    }
  }
</script>

<div class="">
  <select
    class=""
    value={selectedLanguage.code}
    onchange={handleLanguageChange}
    {disabled}
    aria-label={label}
  >
    {#if includeAutoDetect}
      <option value="auto">Auto-detect</option>
    {/if}

    {#each favorites as language (language.code)}
      <option value={language.code}>
        {language.english_name}
      </option>
    {/each}
  </select>
</div>

<style>
  /* CSS goes in /src/styles.css */
</style>
