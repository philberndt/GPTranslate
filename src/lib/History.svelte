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

<div class="" data-theme={theme}>
  <div
    class=""
  >
    <h4 class="">
      <i class=""></i>Translation History
    </h4>
    <div class="">
      <button
        class=""
        onclick={deduplicateHistory}
        disabled={history.entries.length === 0}
        title="Remove duplicate entries"
      >
        <i class=""></i>
        Deduplicate
      </button>
      <button
        class=""
        onclick={clearHistory}
        disabled={history.entries.length === 0}
        title="Clear all history"
      >
        <i class=""></i>
        Clear All
      </button>
      <button
        class=""
        onclick={onClose}
        title="Close"
        aria-label="Close history"
      >
        <i class=""></i>
      </button>
    </div>
  </div>

  <div class="">
    {#if isLoading}
      <div class="">
        <div class="">
          <div
            class=""
            role="status"
          ></div>
          Loading history...
        </div>
      </div>
    {:else if error}
      <div class="" role="alert">
        <i class=""></i>
        Error: {error}
      </div>
    {:else if history.entries.length === 0}
      <div class="">
        <div class="">
          <i class=""></i>
          <p class="">No translation history found.</p>
        </div>
      </div>
    {:else}
      <div class="">
        {#each history.entries as entry (entry.id)}
          <div class="">
            <div
              class=""
            >
              <div class="">
                <span class="">{entry.detected_language}</span
                >
                <i class=""></i>
                <span class="">{entry.target_language}</span>
              </div>
              <div class="">
                <small class="">
                  {formatDate(entry.timestamp)}
                </small>
                <button
                  class=""
                  onclick={() => deleteHistoryEntry(entry.id)}
                  title="Delete this entry"
                  aria-label="Delete this entry"
                >
                  <i class=""></i>
                </button>
              </div>
            </div>

            <div class="">
              <div class="">
                <div class="">
                  <div
                    class=""
                  >
                    <h6 class="">Original</h6>
                    <button
                      class=""
                      onclick={() => copyToClipboard(entry.original_text)}
                      title="Copy original text"
                      aria-label="Copy original text"
                    >
                      <i class=""></i>
                    </button>
                  </div>
                  <div
                    class=""
                    title={entry.original_text}
                  >
                    {#each truncateText(entry.original_text, 100).split("\n") as line, i (i)}
                      {#if i > 0}<br />{/if}{line}
                    {/each}
                  </div>
                </div>

                <div class="">
                  <div
                    class=""
                  >
                    <h6 class="">Translation</h6>
                    <button
                      class=""
                      onclick={() => copyToClipboard(entry.translated_text)}
                      title="Copy translation"
                      aria-label="Copy translation"
                    >
                      <i class=""></i>
                    </button>
                  </div>
                  <div
                    class=""
                    title={entry.translated_text}
                  >
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

<style>
  /* CSS moved to /src/app.css */
</style>
