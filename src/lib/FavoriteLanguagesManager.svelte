<script lang="ts">
  import { LanguageManager, type Language } from "./languages"
  import LanguageDropdown from "./LanguageDropdown.svelte"
  import { ChevronUpIcon, ChevronDownIcon, XMarkIcon } from "heroicons-svelte/24/outline"

  interface Props {
    favoriteLanguageCodes: string[]
    onFavoritesUpdate: (codes: string[]) => void
    disabled?: boolean
  }

  let {
    favoriteLanguageCodes,
    onFavoritesUpdate,
    disabled = false,
  }: Props = $props()

  // Convert codes to language objects
  const favoriteLanguages = $derived.by(() => {
    return favoriteLanguageCodes
      .map((code) => LanguageManager.findByCode(code))
      .filter(Boolean) as Language[]
  })

  // Create a special "placeholder" language for initial state
  let selectedLanguageForAdd = $state({
    code: "",
    english_name: "Add a favorite language...",
    native_name: "Add a favorite language...",
  } as Language)

  function handleAddButtonClick() {
    const newCode = selectedLanguageForAdd.code

    // Don't add if no selection made, already in favorites, or auto-detect
    if (
      !newCode ||
      favoriteLanguageCodes.includes(newCode) ||
      newCode === "auto"
    ) {
      return
    }

    const newFavorites = [...favoriteLanguageCodes, newCode]
    onFavoritesUpdate(newFavorites)

    // Reset to placeholder after adding
    selectedLanguageForAdd = {
      code: "",
      english_name: "Add a favorite language...",
      native_name: "Add a favorite language...",
    } as Language
  }

  function handleLanguageSelection(language: Language) {
    selectedLanguageForAdd = language
  }

  function removeFavorite(codeToRemove: string) {
    const newFavorites = favoriteLanguageCodes.filter(
      (code) => code !== codeToRemove
    )
    onFavoritesUpdate(newFavorites)
  }

  function moveUp(index: number) {
    if (index <= 0) return
    const newFavorites = [...favoriteLanguageCodes]
    ;[newFavorites[index - 1], newFavorites[index]] = [
      newFavorites[index],
      newFavorites[index - 1],
    ]
    onFavoritesUpdate(newFavorites)
  }

  function moveDown(index: number) {
    if (index >= favoriteLanguageCodes.length - 1) return
    const newFavorites = [...favoriteLanguageCodes]
    ;[newFavorites[index], newFavorites[index + 1]] = [
      newFavorites[index + 1],
      newFavorites[index],
    ]
    onFavoritesUpdate(newFavorites)
  }
</script>

<div class="space-y-6">
  <h4 class="text-lg font-semibold text-base-content">Favorite Languages</h4>

  <!-- Add Favorite Language -->
  <div class="grid grid-cols-1 gap-3 sm:grid-cols-[1fr_auto] items-end">
    <div class="form-control w-full">
      <label class="label" for="add-favorite">
        <span class="label-text font-medium">Add Favorite Language</span>
      </label>
      <LanguageDropdown
        selectedLanguage={selectedLanguageForAdd}
        favoriteLanguages={[]}
        includeAutoDetect={false}
        onLanguageSelect={handleLanguageSelection}
        label=""
        placeholder="Add a favorite language..."
      />
    </div>
    <div class="sm:pb-1">
      <button
        type="button"
        class="btn btn-soft btn-primary"
        onclick={handleAddButtonClick}
        disabled={disabled || !selectedLanguageForAdd.code || favoriteLanguageCodes.includes(selectedLanguageForAdd.code)}
      >
        Add
      </button>
    </div>
  </div>

  {#if favoriteLanguages.length === 0}
    <div class="alert alert-info">
      <span>No favorite languages yet. Add some for quick access!</span>
    </div>
  {:else}
    <div class="space-y-2">
      {#each favoriteLanguages as language, index (language.code)}
        <div class="flex items-center justify-between p-3 rounded-lg border border-base-300/50 bg-base-200/40" class:opacity-50={disabled} class:pointer-events-none={disabled}>
          <div class="flex items-center gap-2">
            <span class="badge badge-outline">{LanguageManager.formatDisplayName(language)}</span>
            {#if LanguageManager.isCustomLanguage(language)}
              <span class="badge badge-ghost badge-sm">Custom</span>
            {/if}
          </div>
          <div>
            <div class="btn-group" role="group">
              <button
                type="button"
                class="btn btn-ghost btn-sm"
                onclick={() => moveUp(index)}
                disabled={disabled || index === 0}
                title="Move up"
                aria-label="Move up"
              >
                <ChevronUpIcon class="w-4 h-4" />
              </button>

              <button
                type="button"
                class="btn btn-ghost btn-sm"
                onclick={() => moveDown(index)}
                disabled={disabled || index === favoriteLanguages.length - 1}
                title="Move down"
                aria-label="Move down"
              >
                <ChevronDownIcon class="w-4 h-4" />
              </button>

              <button
                type="button"
                class="btn btn-ghost btn-sm text-error"
                onclick={() => removeFavorite(language.code)}
                {disabled}
                title="Remove from favorites"
                aria-label="Remove from favorites"
              >
                <XMarkIcon class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>

    <p class="text-xs text-base-content/70">
      Favorite languages appear first in language dropdowns. Use arrows to reorder.
    </p>
  {/if}
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
