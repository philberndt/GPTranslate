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

<div>
  <div>
    <div>
      <h4>Translation Languages</h4>
    </div>
  </div>

  <!-- Primary Language Settings -->
  <div>
    <div>
      <label for="primary-target"> Primary Target Language </label>
      <LanguageDropdown
        selectedLanguage={targetLanguage}
        favoriteLanguages={config.favorite_languages || []}
        includeAutoDetect={false}
        onLanguageSelect={handleTargetLanguageChange}
        label=""
      />
      <div>The main language you want to translate to.</div>
    </div>

    <div>
      <label for="alternative-target"> Alternative Target Language </label>
      <LanguageDropdown
        selectedLanguage={alternativeLanguage}
        favoriteLanguages={config.favorite_languages || []}
        includeAutoDetect={false}
        onLanguageSelect={handleAlternativeLanguageChange}
        label=""
      />
      <div>
        Used when the detected language is the same as your primary target.
      </div>
    </div>
  </div>

  <!-- Smart Translation Logic Explanation -->
  <div>
    <h6>How Smart Language Selection Works</h6>
    <p>
      GPTranslate automatically chooses the best target language based on what
      it detects:
    </p>
    <ul>
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
  <div>
    <FavoriteLanguagesManager
      favoriteLanguageCodes={config.favorite_languages || []}
      onFavoritesUpdate={handleFavoritesUpdate}
    />
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
