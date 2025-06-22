use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TranslationEntry {
    pub id: String,
    pub original_text: String,
    pub translated_text: String,
    pub detected_language: String,
    pub target_language: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TranslationHistory {
    pub entries: Vec<TranslationEntry>,
}

impl TranslationHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: TranslationEntry) {
        // Insert at the beginning to keep newest entries first
        self.entries.insert(0, entry);

        // Keep only the last 100 entries to prevent the file from growing too large
        if self.entries.len() > 100 {
            self.entries.truncate(100);
        }
    }
}

fn get_history_file_path() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    let config_dir = home_dir.join(".gptranslate");

    // Create the directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir.join("history.json"))
}

pub fn load_history() -> Result<TranslationHistory> {
    let history_path = get_history_file_path()?;

    if !history_path.exists() {
        return Ok(TranslationHistory::new());
    }

    let contents = fs::read_to_string(history_path)?;
    let history: TranslationHistory =
        serde_json::from_str(&contents).unwrap_or_else(|_| TranslationHistory::new());

    Ok(history)
}

pub fn save_history(history: &TranslationHistory) -> Result<()> {
    let history_path = get_history_file_path()?;
    let contents = serde_json::to_string_pretty(history)?;
    fs::write(history_path, contents)?;
    Ok(())
}

pub fn add_translation_to_history(
    original_text: String,
    translated_text: String,
    detected_language: String,
    target_language: String,
) -> Result<()> {
    // Skip saving if the text is too short (likely incomplete typing)
    if original_text.trim().len() < 10 {
        return Ok(());
    }

    let mut history = load_history()?;

    // Check for near-duplicate entries (same original text with minor differences)
    if let Some(last_entry) = history.entries.first() {
        // Calculate similarity between current and last entry
        let similarity = calculate_text_similarity(&original_text, &last_entry.original_text);

        // If texts are very similar (> 80% similar), update the existing entry instead of creating new one
        if similarity > 0.8
            && last_entry.detected_language == detected_language
            && last_entry.target_language == target_language
        {
            // Update the existing entry with the newer, likely more complete text
            if original_text.len() > last_entry.original_text.len() {
                history.entries[0] = TranslationEntry {
                    id: last_entry.id.clone(), // Keep the same ID
                    original_text,
                    translated_text,
                    detected_language,
                    target_language,
                    timestamp: Utc::now(), // Update timestamp
                };
                save_history(&history)?;
            }
            return Ok(());
        }
    }

    // Create new entry only if it's sufficiently different
    let entry = TranslationEntry {
        id: uuid::Uuid::new_v4().to_string(),
        original_text,
        translated_text,
        detected_language,
        target_language,
        timestamp: Utc::now(),
    };

    history.add_entry(entry);
    save_history(&history)?;

    Ok(())
}

// Simple text similarity calculation using Levenshtein distance
fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
    let len1 = text1.len();
    let len2 = text2.len();

    if len1 == 0 && len2 == 0 {
        return 1.0;
    }

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let distance = levenshtein_distance(text1, text2);
    let max_len = len1.max(len2) as f64;

    1.0 - (distance as f64 / max_len)
}

// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1]; // Initialize first row and column
    for (i, item) in matrix.iter_mut().enumerate().take(len1 + 1) {
        item[0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    // Fill the matrix
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}

pub fn get_translation_history() -> Result<TranslationHistory> {
    load_history()
}

pub fn clear_translation_history() -> Result<()> {
    let history = TranslationHistory::new();
    save_history(&history)?;
    Ok(())
}

pub fn deduplicate_history() -> Result<()> {
    let mut history = load_history()?;
    let mut deduplicated_entries = Vec::new();

    for entry in history.entries {
        // Check if we already have a very similar entry
        let is_duplicate = deduplicated_entries
            .iter()
            .any(|existing: &TranslationEntry| {
                let similarity =
                    calculate_text_similarity(&entry.original_text, &existing.original_text);
                similarity > 0.8
                    && entry.detected_language == existing.detected_language
                    && entry.target_language == existing.target_language
            });

        if !is_duplicate {
            deduplicated_entries.push(entry);
        }
    }

    // Sort by timestamp (newest first)
    deduplicated_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    history.entries = deduplicated_entries;
    save_history(&history)?;

    Ok(())
}

pub fn delete_history_entry(entry_id: String) -> Result<()> {
    let mut history = load_history()?;

    // Find and remove the entry with the matching ID
    history.entries.retain(|entry| entry.id != entry_id);

    save_history(&history)?;
    Ok(())
}

pub fn fix_target_language_in_history() -> Result<()> {
    let mut history = load_history()?;

    for entry in &mut history.entries {
        // If the target language is "English" but the translated text appears to be Norwegian,
        // update the target language to "Norwegian"
        if entry.target_language == "English" && entry.detected_language == "English" {
            // Simple heuristic: check if translated text contains Norwegian characters or common Norwegian words
            let norwegian_indicators = [
                "å", "æ", "ø", "Å", "Æ", "Ø", // Norwegian letters
                " og ", " av ", " på ", " med ", " til ", " for ", " ikke ", " kan ", " det ",
                " er ", // Common Norwegian words
            ];

            let has_norwegian_indicators = norwegian_indicators
                .iter()
                .any(|&indicator| entry.translated_text.contains(indicator));

            // Also check for Norwegian sentence patterns (words ending in common Norwegian suffixes)
            let norwegian_suffixes = ["ene", "ing", "het", "else"];
            let has_norwegian_suffixes = norwegian_suffixes.iter().any(|&suffix| {
                entry
                    .translated_text
                    .split_whitespace()
                    .any(|word| word.to_lowercase().ends_with(suffix))
            });

            if has_norwegian_indicators || has_norwegian_suffixes {
                entry.target_language = "Norwegian".to_string();
            }
        }
    }

    save_history(&history)?;
    Ok(())
}
