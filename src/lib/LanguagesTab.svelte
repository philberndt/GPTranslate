<script lang="ts">
  import { LanguageManager, type Language } from "./languages"
  import LanguageDropdown from "./LanguageDropdown.svelte"
  import FavoriteLanguagesManager from "./FavoriteLanguagesManager.svelte"
  import {
    LanguageIcon,
    InformationCircleIcon,
  } from "heroicons-svelte/24/outline"

  interface Props {
    config: any
    onConfigUpdate: (newConfig: any) => Promise<void>
  }

  let { config, onConfigUpdate }: Props = $props()

  // Get current language objects from config
  const targetLanguage = $derived.by(() => {
    if (!config?.target_language) {
      return (
        LanguageManager.findByCode("en") ||
        LanguageManager.createCustomLanguage("English")
      )
    }
    const lang = LanguageManager.search(config.target_language, false).find(
      (l) =>
        l.english_name.toLowerCase() === config.target_language.toLowerCase()
    )
    return lang || LanguageManager.createCustomLanguage(config.target_language)
  })

  const alternativeLanguage = $derived.by(() => {
    if (!config?.alternative_target_language) {
      return (
        LanguageManager.findByCode("es") ||
        LanguageManager.createCustomLanguage("Spanish")
      )
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

<div class="ml-10 mr-10 overflow-hidden">
  <!-- Primary Language Settings -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body space-y-6">
      <h5 class="card-title flex items-center gap-2 mb-2">
        <LanguageIcon class="w-5 h-5" />
        Languages
      </h5>
      <div class="space-y-6 mt-4 mx-8">
        <div class="grid md:grid-cols-3 gap-4">
          <div class="form-control w-full">
            <label class="label" for="source-language">
              <span class="label-text font-medium mb-2">Source Language</span>
            </label>
            <LanguageDropdown
              selectedLanguage={config.user_source_language ?
                LanguageManager.search(config.user_source_language, false).find(
                  (l) =>
                    l.english_name.toLowerCase() ===
                    config.user_source_language.toLowerCase()
                ) ||
                LanguageManager.createCustomLanguage(
                  config.user_source_language
                )
              : LanguageManager.getAutoDetect()}
              favoriteLanguages={config.favorite_languages || []}
              includeAutoDetect={true}
              onLanguageSelect={async (lang) => {
                const newConfig = {
                  ...config,
                  user_source_language:
                    lang.code === "auto" ? null : lang.english_name,
                }
                await onConfigUpdate(newConfig)
              }}
              label=""
            />
            <div class="label">
              <span class="label-text-alt text-wrap text-base-content/30">
                Language of the text you enter. Leave on Auto-detect unless you
                need to force a specific language.
              </span>
            </div>
          </div>
          <div class="form-control w-full">
            <label class="label" for="primary-target">
              <span class="label-text font-medium mb-2"
                >Primary Target Language</span
              >
            </label>
            <LanguageDropdown
              selectedLanguage={targetLanguage}
              favoriteLanguages={config.favorite_languages || []}
              includeAutoDetect={false}
              onLanguageSelect={handleTargetLanguageChange}
              label=""
            />
            <div class="label">
              <span class="label-text-alt text-wrap text-base-content/30">
                The main language you want to translate to.
              </span>
            </div>
          </div>
          <div class="form-control w-full">
            <label class="label" for="alternative-target">
              <span class="label-text font-medium mb-2"
                >Alternative Target Language</span
              >
            </label>
            <LanguageDropdown
              selectedLanguage={alternativeLanguage}
              favoriteLanguages={config.favorite_languages || []}
              includeAutoDetect={false}
              onLanguageSelect={handleAlternativeLanguageChange}
              label=""
            />
            <div class="label">
              <span class="label-text-alt text-wrap text-base-content/30">
                Used when the detected language matches your primary target.
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Smart Translation Logic Explanation -->
      <div class="alert alert-soft mt-6">
        <div class="space-y-2 w-full">
          <h5
            class="flex items-center gap-2 font-semibold text-base-content/70"
          >
            <InformationCircleIcon class="w-5 h-5" />
            How Smart Language Selection Works
          </h5>
          <div class="mx-8 space-y-2 text-base-content/50">
            <p class="text-sm">
              GPTranslate automatically chooses the best target language based
              on what it detects:
            </p>
            <ul class="list-disc list-inside text-sm space-y-1">
              <li>
                <strong>If detected language ≠ primary target:</strong> Uses your
                primary target language
              </li>
              <li>
                <strong>If detected language = primary target:</strong> Uses your
                alternative target language
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
      </div>
    </div>
  </div>

  <!-- Favorite Languages Management -->
  <FavoriteLanguagesManager
    favoriteLanguageCodes={config.favorite_languages || []}
    onFavoritesUpdate={handleFavoritesUpdate}
  />
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
