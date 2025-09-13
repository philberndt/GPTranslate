// Language database with comprehensive list of supported languages

export interface Language {
  code: string;
  english_name: string;
  native_name: string;
  common_names?: string[]; // Alternative names for searching
}

export const LANGUAGES: Language[] = [
  { code: "auto", english_name: "Auto-detect", native_name: "Auto-detect" },
  { code: "en", english_name: "English", native_name: "English" },
  {
    code: "es",
    english_name: "Spanish",
    native_name: "Español",
    common_names: ["Spanish", "Castilian"],
  },
  { code: "fr", english_name: "French", native_name: "Français" },
  { code: "de", english_name: "German", native_name: "Deutsch" },
  { code: "it", english_name: "Italian", native_name: "Italiano" },
  { code: "pt", english_name: "Portuguese", native_name: "Português" },
  { code: "ru", english_name: "Russian", native_name: "Русский" },
  { code: "ja", english_name: "Japanese", native_name: "日本語" },
  { code: "ko", english_name: "Korean", native_name: "한국어" },
  {
    code: "zh-cn",
    english_name: "Chinese (Simplified)",
    native_name: "简体中文",
    common_names: ["Chinese", "Mandarin"],
  },
  {
    code: "zh-tw",
    english_name: "Chinese (Traditional)",
    native_name: "繁體中文",
  },
  { code: "ar", english_name: "Arabic", native_name: "العربية" },
  { code: "hi", english_name: "Hindi", native_name: "हिन्दी" },
  { code: "th", english_name: "Thai", native_name: "ไทย" },
  { code: "vi", english_name: "Vietnamese", native_name: "Tiếng Việt" },
  { code: "nl", english_name: "Dutch", native_name: "Nederlands" },
  { code: "sv", english_name: "Swedish", native_name: "Svenska" },
  { code: "da", english_name: "Danish", native_name: "Dansk" },
  { code: "no", english_name: "Norwegian", native_name: "Norsk" },
  { code: "fi", english_name: "Finnish", native_name: "Suomi" },
  { code: "pl", english_name: "Polish", native_name: "Polski" },
  { code: "cs", english_name: "Czech", native_name: "Čeština" },
  { code: "sk", english_name: "Slovak", native_name: "Slovenčina" },
  { code: "hu", english_name: "Hungarian", native_name: "Magyar" },
  { code: "ro", english_name: "Romanian", native_name: "Română" },
  { code: "bg", english_name: "Bulgarian", native_name: "Български" },
  { code: "hr", english_name: "Croatian", native_name: "Hrvatski" },
  { code: "sr", english_name: "Serbian", native_name: "Српски" },
  { code: "sl", english_name: "Slovenian", native_name: "Slovenščina" },
  { code: "et", english_name: "Estonian", native_name: "Eesti" },
  { code: "lv", english_name: "Latvian", native_name: "Latviešu" },
  { code: "lt", english_name: "Lithuanian", native_name: "Lietuvių" },
  { code: "uk", english_name: "Ukrainian", native_name: "Українська" },
  { code: "be", english_name: "Belarusian", native_name: "Беларуская" },
  { code: "mk", english_name: "Macedonian", native_name: "Македонски" },
  { code: "sq", english_name: "Albanian", native_name: "Shqip" },
  { code: "mt", english_name: "Maltese", native_name: "Malti" },
  { code: "is", english_name: "Icelandic", native_name: "Íslenska" },
  { code: "ga", english_name: "Irish", native_name: "Gaeilge" },
  { code: "cy", english_name: "Welsh", native_name: "Cymraeg" },
  { code: "eu", english_name: "Basque", native_name: "Euskera" },
  { code: "ca", english_name: "Catalan", native_name: "Català" },
  { code: "gl", english_name: "Galician", native_name: "Galego" },
  { code: "el", english_name: "Greek", native_name: "Ελληνικά" },
  { code: "tr", english_name: "Turkish", native_name: "Türkçe" },
  { code: "he", english_name: "Hebrew", native_name: "עברית" },
  {
    code: "fa",
    english_name: "Persian",
    native_name: "فارسی",
    common_names: ["Persian", "Farsi"],
  },
  { code: "ur", english_name: "Urdu", native_name: "اردو" },
  { code: "bn", english_name: "Bengali", native_name: "বাংলা" },
  { code: "gu", english_name: "Gujarati", native_name: "ગુજરાતી" },
  { code: "pa", english_name: "Punjabi", native_name: "ਪੰਜਾਬੀ" },
  { code: "ta", english_name: "Tamil", native_name: "தமிழ்" },
  { code: "te", english_name: "Telugu", native_name: "తెలుగు" },
  { code: "kn", english_name: "Kannada", native_name: "ಕನ್ನಡ" },
  { code: "ml", english_name: "Malayalam", native_name: "മലയാളം" },
  { code: "mr", english_name: "Marathi", native_name: "मराठी" },
  { code: "ne", english_name: "Nepali", native_name: "नेपाली" },
  { code: "si", english_name: "Sinhala", native_name: "සිංහල" },
  { code: "my", english_name: "Burmese", native_name: "မြန်မာ" },
  { code: "km", english_name: "Khmer", native_name: "ខ្មែរ" },
  { code: "lo", english_name: "Lao", native_name: "ລາວ" },
  { code: "ka", english_name: "Georgian", native_name: "ქართული" },
  { code: "am", english_name: "Amharic", native_name: "አማርኛ" },
  { code: "sw", english_name: "Swahili", native_name: "Kiswahili" },
  { code: "zu", english_name: "Zulu", native_name: "isiZulu" },
  { code: "af", english_name: "Afrikaans", native_name: "Afrikaans" },
  { code: "yo", english_name: "Yoruba", native_name: "Yorùbá" },
  { code: "ig", english_name: "Igbo", native_name: "Asụsụ Igbo" },
  { code: "ha", english_name: "Hausa", native_name: "Harshen Hausa" },
  { code: "ms", english_name: "Malay", native_name: "Bahasa Melayu" },
  { code: "id", english_name: "Indonesian", native_name: "Bahasa Indonesia" },
  {
    code: "tl",
    english_name: "Filipino",
    native_name: "Filipino",
    common_names: ["Filipino", "Tagalog"],
  },
  { code: "haw", english_name: "Hawaiian", native_name: "ʻŌlelo Hawaiʻi" },
  { code: "mi", english_name: "Maori", native_name: "Te Reo Māori" },
  { code: "sm", english_name: "Samoan", native_name: "Gagana Samoa" },
  { code: "to", english_name: "Tongan", native_name: "Lea Fakatonga" },
  { code: "fj", english_name: "Fijian", native_name: "Na Vosa Vakaviti" },
  { code: "az", english_name: "Azerbaijani", native_name: "Azərbaycan dili" },
  { code: "kk", english_name: "Kazakh", native_name: "Қазақ тілі" },
  { code: "ky", english_name: "Kyrgyz", native_name: "Кыргыз тили" },
  { code: "uz", english_name: "Uzbek", native_name: "Oʻzbek tili" },
  { code: "tg", english_name: "Tajik", native_name: "Тоҷикӣ" },
  { code: "tk", english_name: "Turkmen", native_name: "Türkmen dili" },
  { code: "mn", english_name: "Mongolian", native_name: "Монгол хэл" },
  { code: "bo", english_name: "Tibetan", native_name: "བོད་སྐད།" },
  { code: "dv", english_name: "Maldivian", native_name: "ދިވެހިބަސް" },
  { code: "ps", english_name: "Pashto", native_name: "پښتو" },
  { code: "sd", english_name: "Sindhi", native_name: "سنڌي" },
  { code: "ku", english_name: "Kurdish", native_name: "Kurdî" },
  { code: "yi", english_name: "Yiddish", native_name: "ייִדיש" },
  { code: "la", english_name: "Latin", native_name: "Latina" },
  { code: "eo", english_name: "Esperanto", native_name: "Esperanto" },
  { code: "jw", english_name: "Javanese", native_name: "Basa Jawa" },
  { code: "su", english_name: "Sundanese", native_name: "Basa Sunda" },
  {
    code: "ceb",
    english_name: "Cebuano",
    native_name: "Sinugbuanong Binisaya",
  },
  { code: "ny", english_name: "Chichewa", native_name: "ChiCheŵa" },
  { code: "co", english_name: "Corsican", native_name: "Corsu" },
  { code: "fy", english_name: "Frisian", native_name: "Frysk" },
  { code: "gd", english_name: "Scottish Gaelic", native_name: "Gàidhlig" },
  { code: "hm", english_name: "Hmong", native_name: "Hmoob" },
  { code: "lb", english_name: "Luxembourgish", native_name: "Lëtzebuergesch" },
  { code: "mg", english_name: "Malagasy", native_name: "Malagasy" },
  { code: "st", english_name: "Sesotho", native_name: "Sesotho" },
  { code: "sn", english_name: "Shona", native_name: "ChiShona" },
  { code: "so", english_name: "Somali", native_name: "Soomaaliga" },
  { code: "xh", english_name: "Xhosa", native_name: "isiXhosa" },
];

// Utility functions for language management
export class LanguageManager {
  // Find language by code
  static findByCode(code: string): Language | undefined {
    return LANGUAGES.find(
      (lang) => lang.code.toLowerCase() === code.toLowerCase(),
    );
  }

  // Find languages by search term (searches english_name, native_name, and common_names)
  static search(query: string, includeAutoDetect = true): Language[] {
    if (!query || !query.trim()) {
      return includeAutoDetect
        ? LANGUAGES
        : LANGUAGES.filter((lang) => lang.code !== "auto");
    }

    const searchTerm = query.toLowerCase().trim();
    const results = LANGUAGES.filter((lang) => {
      if (!includeAutoDetect && lang.code === "auto") return false;

      const matches = [
        lang.english_name.toLowerCase().includes(searchTerm),
        lang.native_name.toLowerCase().includes(searchTerm),
        lang.code.toLowerCase().includes(searchTerm),
        ...(lang.common_names || []).map((name) =>
          name.toLowerCase().includes(searchTerm),
        ),
      ];

      return matches.some(Boolean);
    });

    // Sort results by relevance (exact matches first, then starts-with, then contains)
    return results.sort((a, b) => {
      const getScore = (lang: Language): number => {
        const names = [
          lang.english_name,
          lang.native_name,
          ...(lang.common_names || []),
        ];
        const lowerNames = names.map((name) => name.toLowerCase());

        // Exact match
        if (lowerNames.some((name) => name === searchTerm)) return 100;
        // Starts with
        if (lowerNames.some((name) => name.startsWith(searchTerm))) return 50;
        // Contains
        return 10;
      };

      return getScore(b) - getScore(a);
    });
  }

  // Get all languages except auto-detect
  static getAllLanguages(): Language[] {
    return LANGUAGES.filter((lang) => lang.code !== "auto");
  }

  // Get auto-detect language
  static getAutoDetect(): Language {
    return LANGUAGES[0]; // Auto-detect is always first
  }

  // Format language display name
  static formatDisplayName(lang: Language, showNative = true): string {
    if (lang.code === "auto") return lang.english_name;
    if (!showNative) return lang.english_name;
    if (lang.english_name === lang.native_name) return lang.english_name;
    return `${lang.english_name} (${lang.native_name})`;
  }

  // Create a custom language (for user-defined languages not in the standard list)
  static createCustomLanguage(name: string): Language {
    return {
      code: `custom-${name.toLowerCase().replace(/\s+/g, "-")}`,
      english_name: name,
      native_name: name,
      common_names: [name],
    };
  }

  // Check if a language is custom (user-defined)
  static isCustomLanguage(lang: Language): boolean {
    return lang.code.startsWith("custom-");
  }

  // Get language suggestions based on favorites and commonly used languages
  static getSuggestions(favorites: string[] = []): Language[] {
    const favoriteLanguages = favorites
      .map((code) => this.findByCode(code))
      .filter(Boolean) as Language[];

    const commonLanguages = [
      "en",
      "es",
      "fr",
      "de",
      "it",
      "pt",
      "ru",
      "ja",
      "ko",
      "zh-cn",
      "ar",
      "hi",
    ]
      .map((code) => this.findByCode(code))
      .filter(Boolean) as Language[];

    // Combine favorites with common languages, remove duplicates
    const suggestions = [...favoriteLanguages];
    commonLanguages.forEach((lang) => {
      if (!suggestions.some((s) => s.code === lang.code)) {
        suggestions.push(lang);
      }
    });

    return suggestions;
  }
}

// Type for language selection events
export interface LanguageSelectionEvent {
  language: Language;
  isCustom: boolean;
}

// Helper function to normalize language names for comparison
export function normalizeLanguageName(name: string): string {
  return name
    .toLowerCase()
    .trim()
    .replace(/[^\w\s-]/g, "");
}
