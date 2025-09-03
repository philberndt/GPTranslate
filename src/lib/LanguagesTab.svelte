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
    if (!config?.target_language) {
      return LanguageManager.findByCode('en') || LanguageManager.createCustomLanguage('English')
    }
    const lang = LanguageManager.search(config.target_language, false).find(
      (l) =>
        l.english_name.toLowerCase() === config.target_language.toLowerCase()
    )
    return lang || LanguageManager.createCustomLanguage(config.target_language)
  })

  const alternativeLanguage = $derived.by(() => {
    if (!config?.alternative_target_language) {
      return LanguageManager.findByCode('es') || LanguageManager.createCustomLanguage('Spanish')
    }
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

<div class="space-y-6">
  <div>
    <h4 class="text-lg font-semibold text-base-content mb-4">Translation Languages</h4>
  </div>

  <!-- Primary Language Settings -->
  <div class="space-y-4">
    <div class="form-control w-full">
      <label class="label" for="primary-target">
        <span class="label-text font-medium">Primary Target Language</span>
      </label>
      <LanguageDropdown
        selectedLanguage={targetLanguage}
        favoriteLanguages={config.favorite_languages || []}
        includeAutoDetect={false}
        onLanguageSelect={handleTargetLanguageChange}
        label=""
      />
      <div class="label">
        <span class="label-text-alt text-base-content/70">The main language you want to translate to.</span>
      </div>
    </div>

    <div class="form-control w-full">
      <label class="label" for="alternative-target">
        <span class="label-text font-medium">Alternative Target Language</span>
      </label>
      <LanguageDropdown
        selectedLanguage={alternativeLanguage}
        favoriteLanguages={config.favorite_languages || []}
        includeAutoDetect={false}
        onLanguageSelect={handleAlternativeLanguageChange}
        label=""
      />
      <div class="label">
        <span class="label-text-alt text-base-content/70">Used when the detected language is the same as your primary target.</span>
      </div>
    </div>
  </div>

  <!-- Smart Translation Logic Explanation -->
  <div class="alert alert-info">
    <div>
      <h6 class="font-semibold">How Smart Language Selection Works</h6>
      <p class="text-sm mt-2">
        GPTranslate automatically chooses the best target language based on what
        it detects:
      </p>
      <ul class="list-disc list-inside text-sm mt-2 space-y-1">
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
  </div>

  <!-- Favorite Languages Management -->
  <div>
    <FavoriteLanguagesManager
      favoriteLanguageCodes={config.favorite_languages || []}
      onFavoritesUpdate={handleFavoritesUpdate}
    />
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
