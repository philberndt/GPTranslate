<script lang="ts">
  import { LanguageManager, type Language } from "./languages"

  interface Props {
    selectedLanguage: Language
    favoriteLanguages?: string[]
    includeAutoDetect?: boolean
    onLanguageSelect: (language: Language) => void
    label?: string
    disabled?: boolean
    placeholder?: string
  }

  let {
    selectedLanguage,
    favoriteLanguages = [],
    includeAutoDetect = false,
    onLanguageSelect,
    label = "Language",
    disabled = false,
    placeholder = "",
  }: Props = $props()

  const selectId = 'language-select-' + Math.random().toString(36).slice(2)

  // Get favorite languages and fallback to common suggestions
  const favorites = $derived.by(() => {
    const favLangs = favoriteLanguages
      .map((code) => LanguageManager.findByCode(code))
      .filter(Boolean) as Language[]

    if (favLangs.length === 0) {
      // Fallback to common languages if no favorites set
      return LanguageManager.getSuggestions()
    }
    return favLangs
  })

  // Get other languages (not favorites), sorted alphabetically
  const otherLanguages = $derived.by(() => {
    const allLanguages =
      includeAutoDetect ?
        LanguageManager.search("", true)
      : LanguageManager.getAllLanguages()

    const favoritesCodes = favorites.map((lang) => lang.code)
    return allLanguages
      .filter((lang) => !favoritesCodes.includes(lang.code))
      .sort((a, b) => a.english_name.localeCompare(b.english_name))
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

<div class="form-control w-full">
  {#if label}
    <label class="label" for={selectId}>
      <span class="label-text">{label}</span>
    </label>
  {/if}

  <select
    id={selectId}
    class="select select-bordered w-full bg-base-200 border-base-300 focus:border-primary/30"
    value={selectedLanguage.code}
    onchange={handleLanguageChange}
    {disabled}
  >
    {#if placeholder}
      <option value="" disabled selected={selectedLanguage.code === ""}>{placeholder}</option>
    {/if}

    {#if includeAutoDetect}
      <option value="auto">Auto-detect</option>
      {#if favorites.length > 0 || otherLanguages.length > 0}
        <option disabled>────────────</option>
      {/if}
    {/if}

    {#if favorites.length > 0}
      <optgroup label="Favorites">
        {#each favorites as language (language.code)}
          <option value={language.code}>{LanguageManager.formatDisplayName(language)}</option>
        {/each}
      </optgroup>
    {/if}

    {#if otherLanguages.length > 0}
      <optgroup label="All Languages">
        {#each otherLanguages as language (language.code)}
          <option value={language.code}>{LanguageManager.formatDisplayName(language)}</option>
        {/each}
      </optgroup>
    {/if}
  </select>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
