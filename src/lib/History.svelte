<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { onMount } from "svelte"

  // Theme provided by parent for forcing light/dark; default auto
  export let theme: string = "auto"
  // Close handler for full-screen history view
  export let onClose: () => void

  interface TranslationEntry {
    id: string
    original_text: string
    translated_text: string
    detected_language: string
    target_language: string
    timestamp: string
  }

  interface TranslationHistory {
    entries: TranslationEntry[]
  }

  let history: TranslationHistory = { entries: [] }
  let isLoading = true
  let error = ""

  onMount(async () => {
    await loadHistory()
  })

  async function loadHistory() {
    try {
      isLoading = true
      error = ""
      history = await invoke("get_translation_history_cmd")
    } catch (e) {
      console.error("Failed to load history:", e)
      error = e as string
    } finally {
      isLoading = false
    }
  }
  async function clearHistory() {
    if (confirm("Are you sure you want to clear all translation history?")) {
      try {
        await invoke("clear_translation_history_cmd")
        await loadHistory()
      } catch (e) {
        console.error("Failed to clear history:", e)
        error = e as string
      }
    }
  }

  async function deduplicateHistory() {
    if (
      confirm(
        "This will remove duplicate and very similar entries from your translation history. Continue?"
      )
    ) {
      try {
        await invoke("deduplicate_history_cmd")
        await loadHistory()
      } catch (e) {
        console.error("Failed to deduplicate history:", e)
        error = e as string
      }
    }
  }

  async function deleteHistoryEntry(entryId: string) {
    if (confirm("Are you sure you want to delete this history entry?")) {
      try {
        await invoke("delete_history_entry_cmd", { entryId })
        await loadHistory()
      } catch (e) {
        console.error("Failed to delete history entry:", e)
        error = e as string
      }
    }
  }

  async function copyToClipboard(text: string) {
    try {
      await invoke("copy_to_clipboard", { text })
    } catch (e) {
      console.error("Failed to copy to clipboard:", e)
    }
  }

  function formatDate(timestamp: string): string {
    return new Date(timestamp).toLocaleString()
  }

  function truncateText(text: string, maxLength: number = 50): string {
    if (text.length <= maxLength) return text
    return text.substring(0, maxLength) + "..."
  }
</script>

<div data-theme={theme}>
  <div>
    <h4>Translation History</h4>
    <div>
      <button
        onclick={deduplicateHistory}
        disabled={history.entries.length === 0}
        title="Remove duplicate entries"
      >
        Deduplicate
      </button>
      <button
        onclick={clearHistory}
        disabled={history.entries.length === 0}
        title="Clear all history"
      >
        Clear All
      </button>
      <button onclick={onClose} title="Close" aria-label="Close history">
      </button>
    </div>
  </div>

  <div>
    {#if isLoading}
      <div>
        <div>
          <div role="status"></div>
          Loading history...
        </div>
      </div>
    {:else if error}
      <div role="alert">
        Error: {error}
      </div>
    {:else if history.entries.length === 0}
      <div>
        <div>
          <p>No translation history found.</p>
        </div>
      </div>
    {:else}
      <div>
        {#each history.entries as entry (entry.id)}
          <div>
            <div>
              <div>
                <span>{entry.detected_language}</span>

                <span>{entry.target_language}</span>
              </div>
              <div>
                <small>
                  {formatDate(entry.timestamp)}
                </small>
                <button
                  onclick={() => deleteHistoryEntry(entry.id)}
                  title="Delete this entry"
                  aria-label="Delete this entry"
                >
                </button>
              </div>
            </div>

            <div>
              <div>
                <div>
                  <div>
                    <h6>Original</h6>
                    <button
                      onclick={() => copyToClipboard(entry.original_text)}
                      title="Copy original text"
                      aria-label="Copy original text"
                    >
                    </button>
                  </div>
                  <div title={entry.original_text}>
                    {#each truncateText(entry.original_text, 100).split("\n") as line, i (i)}
                      {#if i > 0}<br />{/if}{line}
                    {/each}
                  </div>
                </div>

                <div>
                  <div>
                    <h6>Translation</h6>
                    <button
                      onclick={() => copyToClipboard(entry.translated_text)}
                      title="Copy translation"
                      aria-label="Copy translation"
                    >
                    </button>
                  </div>
                  <div title={entry.translated_text}>
                    {#each truncateText(entry.translated_text, 100).split("\n") as line, i (i)}
                      {#if i > 0}<br />{/if}{line}
                    {/each}
                  </div>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
