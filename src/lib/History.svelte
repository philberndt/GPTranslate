<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"
  import { onMount } from "svelte"
  import { XMarkIcon } from "heroicons-svelte/24/outline"

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


</script>

<div data-theme={theme} class="min-h-screen bg-base-100 p-6">
  <div class="max-w-6xl mx-auto">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <h4 class="text-2xl font-bold text-base-content">Translation History</h4>
      <div class="flex gap-2">
        <button
          class="btn btn-soft btn-sm"
          onclick={deduplicateHistory}
          disabled={history.entries.length === 0}
          title="Remove duplicate entries"
        >
          Deduplicate
        </button>
        <button
          class="btn btn-soft btn-warning btn-sm"
          onclick={clearHistory}
          disabled={history.entries.length === 0}
          title="Clear all history"
        >
          Clear All
        </button>
        <button class="btn btn-soft btn-circle btn-sm" onclick={onClose} title="Close" aria-label="Close history">
          <XMarkIcon class="w-5 h-5" />
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="space-y-4">
      {#if isLoading}
        <div class="card bg-base-100 border border-base-300/50">
          <div class="card-body text-center">
            <div class="loading loading-spinner loading-md" role="status"></div>
            <p class="text-base-content/70">Loading history...</p>
          </div>
        </div>
      {:else if error}
        <div class="alert alert-error" role="alert">
          <span>Error: {error}</span>
        </div>
      {:else if history.entries.length === 0}
        <div class="card bg-base-100 border border-base-300/50">
          <div class="card-body text-center">
            <p class="text-base-content/70">No translation history found.</p>
          </div>
        </div>
      {:else}
        <div class="space-y-3">
          {#each history.entries as entry (entry.id)}
            <div class="card bg-base-100 border border-base-300/50">
              <div class="card-body p-4">
                <div class="flex items-center justify-between mb-3">
                  <div class="flex items-center gap-2">
                    <span class="badge badge-outline badge-sm">{entry.detected_language}</span>
                    <span class="text-base-content/50">→</span>
                    <span class="badge badge-outline badge-sm">{entry.target_language}</span>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="text-xs text-base-content/60">{formatDate(entry.timestamp)}</span>
                    <div class="dropdown dropdown-end">
                      <button class="btn btn-soft btn-xs">⋮</button>
                      <ul class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
                        <li><button onclick={() => copyToClipboard(entry.original_text)}>Copy Original</button></li>
                        <li><button onclick={() => copyToClipboard(entry.translated_text)}>Copy Translation</button></li>
                        <li><button class="text-error" onclick={() => deleteHistoryEntry(entry.id)}>Delete</button></li>
                      </ul>
                    </div>
                  </div>
                </div>
                <div class="grid md:grid-cols-2 gap-4">
                  <div>
                    <h6 class="text-sm font-semibold text-base-content/70 mb-1">Original</h6>
                    <p class="text-sm bg-base-200 p-3 rounded border border-base-300">{entry.original_text}</p>
                  </div>
                  <div>
                    <h6 class="text-sm font-semibold text-base-content/70 mb-1">Translation</h6>
                    <p class="text-sm bg-base-200 p-3 rounded border border-base-300">{entry.translated_text}</p>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
