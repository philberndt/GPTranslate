<script lang="ts">
  import { LanguageManager, type Language } from "./languages"
  import LanguageDropdown from "./LanguageDropdown.svelte"
  import FavoriteLanguagesManager from "./FavoriteLanguagesManager.svelte"

  interface Props {
    config: any
    onConfigUpdate: (newConfig: any) => Promise<void>
  }

  let { config, onConfigUpdate }: Props = $props()

  // Get current language objects from config
  const targetLanguage = $derived.by(() => {
    const lang = LanguageManager.search(config.target_language, false).find(
      (l) =>
        l.english_name.toLowerCase() === config.target_language.toLowerCase()
    )
    return lang || LanguageManager.createCustomLanguage(config.target_language)
  })

  const alternativeLanguage = $derived.by(() => {
    const lang = LanguageManager.search(
      config.alternative_target_language,
      false
    ).find(
      (l) =>
        l.english_name.toLowerCase() ===
        config.alternative_target_language.toLowerCase()
    )
    return (
      lang ||
      LanguageManager.createCustomLanguage(config.alternative_target_language)
    )
  })

  async function handleTargetLanguageChange(language: Language) {
    const newConfig = {
      ...config,
      target_language: language.english_name,
    }
    await onConfigUpdate(newConfig)
  }

  async function handleAlternativeLanguageChange(language: Language) {
    const newConfig = {
      ...config,
      alternative_target_language: language.english_name,
    }
    await onConfigUpdate(newConfig)
  }

  async function handleFavoritesUpdate(codes: string[]) {
    const newConfig = {
      ...config,
      favorite_languages: codes,
    }
    await onConfigUpdate(newConfig)
  }
</script>

<div class="">
  <div class="">
    <div class="">
      <h4 class="">
        <i class=""></i>
        Translation Languages
      </h4>
    </div>
  </div>

  <!-- Primary Language Settings -->
  <div class="">
    <div class="">
      <label for="primary-target" class="">
        <i class=""></i>
        Primary Target Language
      </label>
      <LanguageDropdown
        selectedLanguage={targetLanguage}
        favoriteLanguages={config.favorite_languages || []}
        includeAutoDetect={false}
        onLanguageSelect={handleTargetLanguageChange}
        label=""
      />
      <div class="">
        The main language you want to translate to.
      </div>
    </div>

    <div class="">
      <label for="alternative-target" class="">
        <i class=""></i>
        Alternative Target Language
      </label>
      <LanguageDropdown
        selectedLanguage={alternativeLanguage}
        favoriteLanguages={config.favorite_languages || []}
        includeAutoDetect={false}
        onLanguageSelect={handleAlternativeLanguageChange}
        label=""
      />
      <div class="">
        Used when the detected language is the same as your primary target.
      </div>
    </div>
  </div>

  <!-- Smart Translation Logic Explanation -->
  <div class="">
    <h6 class="">
      <i class=""></i>
      How Smart Language Selection Works
    </h6>
    <p class="">
      GPTranslate automatically chooses the best target language based on what
      it detects:
    </p>
    <ul class="">
      <li>
        <strong>If detected language ≠ primary target:</strong> Uses your primary
        target language
      </li>
      <li>
        <strong>If detected language = primary target:</strong> Uses your alternative
        target language
      </li>
      <li>
        <strong>Example:</strong> Detected Spanish + Primary English → Translates
        to English
      </li>
      <li>
        <strong>Example:</strong> Detected English + Primary English → Translates
        to your alternative language
      </li>
    </ul>
  </div>

  <!-- Favorite Languages Management -->
  <div class="">
    <FavoriteLanguagesManager
      favoriteLanguageCodes={config.favorite_languages || []}
      onFavoritesUpdate={handleFavoritesUpdate}
    />
  </div>
</div>

<style>
  /* CSS moved to /src/app.css */
</style>
