<script lang="ts">
  import { LanguageManager, type Language } from "./languages"
  import LanguageDropdown from "./LanguageDropdown.svelte"

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

<div class="">
  <hr class="" />

  <h5 class="">
    <i class=""></i>
    Favorite Languages
  </h5>

  <!-- Always visible Add Favorite Language section -->
  <div class="">
    <h6 class="">
      <i class=""></i>
      Add Favorite Language
    </h6>
    <div class="">
      <div class="">
        <LanguageDropdown
          selectedLanguage={selectedLanguageForAdd}
          favoriteLanguages={[]}
          includeAutoDetect={false}
          onLanguageSelect={handleLanguageSelection}
          label=""
          placeholder="Add a favorite language..."
        />
      </div>
      <div class="">
        <button
          type="button"
          class=""
          onclick={handleAddButtonClick}
          disabled={disabled ||
            !selectedLanguageForAdd.code ||
            favoriteLanguageCodes.includes(selectedLanguageForAdd.code)}
        >
          <i class=""></i>
          Add
        </button>
      </div>
    </div>
  </div>

  {#if favoriteLanguages.length === 0}
    <div class="">
      <i class=""></i>
      <p class="">
        No favorite languages yet. Add some for quick access!
      </p>
    </div>
  {:else}
    <div class="">
      {#each favoriteLanguages as language, index (language.code)}
        <div class="" class:disabled>
          <div class="">
            <div class="">
              <span class="">
                {LanguageManager.formatDisplayName(language)}
              </span>
              {#if LanguageManager.isCustomLanguage(language)}
                <span class="">Custom</span>
              {/if}
            </div>
          </div>

          <div class="">
            <div class="" role="group">
              <button
                type="button"
                class=""
                onclick={() => moveUp(index)}
                disabled={disabled || index === 0}
                title="Move up"
                aria-label="Move up"
              >
                <i class=""></i>
              </button>

              <button
                type="button"
                class=""
                onclick={() => moveDown(index)}
                disabled={disabled || index === favoriteLanguages.length - 1}
                title="Move down"
                aria-label="Move down"
              >
                <i class=""></i>
              </button>

              <button
                type="button"
                class=""
                onclick={() => removeFavorite(language.code)}
                {disabled}
                title="Remove from favorites"
                aria-label="Remove from favorites"
              >
                <i class=""></i>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>

    <div class="">
      <small class="">
        <i class=""></i>
        Favorite languages appear first in language dropdowns for quick access. Use
        the arrows to reorder them by preference.
      </small>
    </div>
  {/if}
</div>

<style>
  /* CSS moved to /src/app.css */
</style>
