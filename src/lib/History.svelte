<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    export let onClose: () => void;
    export let theme: string = "auto";

    interface TranslationEntry {
        id: string;
        original_text: string;
        translated_text: string;
        detected_language: string;
        target_language: string;
        timestamp: string;
    }

    interface TranslationHistory {
        entries: TranslationEntry[];
    }

    let history: TranslationHistory = { entries: [] };
    let isLoading = true;
    let error = "";

    onMount(async () => {
        await loadHistory();
    });

    async function loadHistory() {
        try {
            isLoading = true;
            error = "";
            history = await invoke("get_translation_history_cmd");
        } catch (e) {
            console.error("Failed to load history:", e);
            error = e as string;
        } finally {
            isLoading = false;
        }
    }
    async function clearHistory() {
        if (
            confirm("Are you sure you want to clear all translation history?")
        ) {
            try {
                await invoke("clear_translation_history_cmd");
                await loadHistory();
            } catch (e) {
                console.error("Failed to clear history:", e);
                error = e as string;
            }
        }
    }

    async function deduplicateHistory() {
        if (
            confirm(
                "This will remove duplicate and very similar entries from your translation history. Continue?",
            )
        ) {
            try {
                await invoke("deduplicate_history_cmd");
                await loadHistory();
            } catch (e) {
                console.error("Failed to deduplicate history:", e);
                error = e as string;
            }
        }
    }

    async function deleteHistoryEntry(entryId: string) {
        if (confirm("Are you sure you want to delete this history entry?")) {
            try {
                await invoke("delete_history_entry_cmd", { entryId });
                await loadHistory();
            } catch (e) {
                console.error("Failed to delete history entry:", e);
                error = e as string;
            }
        }
    }

    async function copyToClipboard(text: string) {
        try {
            await invoke("copy_to_clipboard", { text });
        } catch (e) {
            console.error("Failed to copy to clipboard:", e);
        }
    }

    function formatDate(timestamp: string): string {
        return new Date(timestamp).toLocaleString();
    }

    function truncateText(text: string, maxLength: number = 50): string {
        if (text.length <= maxLength) return text;
        return text.substring(0, maxLength) + "...";
    }
</script>

<div
    class="history-overlay"
    onclick={onClose}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onkeydown={(e) => e.key === "Escape" && onClose()}
>
    <div
        class="history-dialog"
        role="document"
        class:theme-light={theme === "light"}
        class:theme-dark={theme === "dark"}
    >
        <div class="history-header">
            <h2>Translation History</h2>
            <div class="header-buttons">
                <button
                    class="dedupe-btn"
                    onclick={deduplicateHistory}
                    disabled={history.entries.length === 0}
                    title="Remove duplicate entries"
                >
                    <i class="bi bi-collection"></i>
                    Deduplicate
                </button>
                <button
                    class="clear-btn"
                    onclick={clearHistory}
                    disabled={history.entries.length === 0}
                    title="Clear all history"
                >
                    <i class="bi bi-trash"></i>
                    Clear All
                </button>
                <button
                    class="close-btn"
                    onclick={onClose}
                    title="Close"
                    aria-label="Close history"
                >
                    <i class="bi bi-x-lg"></i>
                </button>
            </div>
        </div>

        <div class="history-content">
            {#if isLoading}
                <div class="loading">Loading history...</div>
            {:else if error}
                <div class="error">Error: {error}</div>
            {:else if history.entries.length === 0}
                <div class="empty">No translation history found.</div>
            {:else}
                <div class="history-list">
                    {#each history.entries as entry (entry.id)}
                        <div class="history-item">
                            <div class="history-item-header">
                                <div class="languages">
                                    <span class="language-tag"
                                        >{entry.detected_language}</span
                                    >
                                    <i class="bi bi-arrow-right"></i>
                                    <span class="language-tag"
                                        >{entry.target_language}</span
                                    >
                                </div>
                                <div class="header-right">
                                    <div class="timestamp">
                                        {formatDate(entry.timestamp)}
                                    </div>
                                    <button
                                        class="delete-entry-btn"
                                        onclick={() =>
                                            deleteHistoryEntry(entry.id)}
                                        title="Delete this entry"
                                        aria-label="Delete this entry"
                                    >
                                        <i class="bi bi-trash3"></i>
                                    </button>
                                </div>
                            </div>

                            <div class="text-panels">
                                <div class="text-panel">
                                    <div class="text-label">Original</div>
                                    <div
                                        class="text-content"
                                        title={entry.original_text}
                                    >
                                        {#each truncateText(entry.original_text, 100).split('\n') as line, i}
                                            {#if i > 0}<br />{/if}{line}
                                        {/each}
                                    </div>
                                    <button
                                        class="copy-btn"
                                        onclick={() =>
                                            copyToClipboard(
                                                entry.original_text,
                                            )}
                                        title="Copy original text"
                                        aria-label="Copy original text"
                                    >
                                        <i class="bi bi-clipboard"></i>
                                    </button>
                                </div>

                                <div class="text-panel">
                                    <div class="text-label">Translation</div>
                                    <div
                                        class="text-content"
                                        title={entry.translated_text}
                                    >
                                        {#each truncateText(
                                            entry.translated_text,
                                            100,
                                        ).split('\n') as line, i}
                                            {#if i > 0}<br />{/if}{line}
                                        {/each}
                                    </div>
                                    <button
                                        class="copy-btn"
                                        onclick={() =>
                                            copyToClipboard(
                                                entry.translated_text,
                                            )}
                                        title="Copy translation"
                                        aria-label="Copy translation"
                                    >
                                        <i class="bi bi-clipboard"></i>
                                    </button>
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .history-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .history-dialog {
        background: white;
        border-radius: 12px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
        width: 90%;
        max-width: 800px;
        max-height: 80vh;
        display: flex;
        flex-direction: column;
    }

    .history-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        border-bottom: 1px solid #e0e0e0;
        flex-shrink: 0;
    }

    .history-header h2 {
        margin: 0;
        color: #333;
        font-size: 1.5rem;
    }

    .header-buttons {
        display: flex;
        gap: 8px;
    }
    .dedupe-btn,
    .clear-btn,
    .close-btn {
        border: 1px solid #ddd;
        border-radius: 6px;
        padding: 8px 12px;
        background: white;
        color: #666;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        gap: 4px;
        font-size: 14px;
    }

    .dedupe-btn {
        background: #4a90e2;
        color: white;
        border-color: #4a90e2;
    }

    .dedupe-btn:hover:not(:disabled) {
        background: #357abd;
        border-color: #357abd;
    }

    .dedupe-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .clear-btn {
        background: #ff6b6b;
        color: white;
        border-color: #ff6b6b;
    }

    .clear-btn:hover:not(:disabled) {
        background: #ff5252;
        border-color: #ff5252;
    }

    .clear-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .close-btn:hover {
        border-color: #999;
        color: #333;
    }

    .history-content {
        flex: 1;
        overflow-y: auto;
        padding: 20px;
    }

    .loading,
    .error,
    .empty {
        text-align: center;
        padding: 40px;
        color: #666;
    }

    .error {
        color: #d63384;
    }

    .history-list {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .history-item {
        border: 1px solid #ddd;
        border-radius: 8px;
        padding: 16px;
        background: #fafbfc;
    }
    .history-item-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 12px;
    }

    .header-right {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .languages {
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .language-tag {
        background: #379df1;
        color: white;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 0.8rem;
        font-weight: 500;
    }

    .timestamp {
        color: #666;
        font-size: 0.85rem;
    }

    .text-panels {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 16px;
    }

    .text-panel {
        border: 1px solid #ddd;
        border-radius: 6px;
        background: white;
        padding: 12px;
        position: relative;
    }

    .text-label {
        font-weight: 500;
        font-size: 0.85rem;
        color: #666;
        margin-bottom: 8px;
    }

    .text-content {
        font-size: 0.9rem;
        line-height: 1.4;
        color: #333;
        margin-bottom: 8px;
        min-height: 40px;
        white-space: pre-wrap;
        word-wrap: break-word;
    }

    .copy-btn {
        position: absolute;
        top: 8px;
        right: 8px;
        background: none;
        border: 1px solid #ddd;
        border-radius: 4px;
        padding: 4px 6px;
        color: #666;
        cursor: pointer;
        transition: all 0.2s;
        font-size: 12px;
    }
    .copy-btn:hover {
        border-color: #379df1;
        color: #379df1;
        background: #f8fdff;
    }

    .delete-entry-btn {
        background: none;
        border: 1px solid #ddd;
        border-radius: 4px;
        padding: 4px 6px;
        color: #666;
        cursor: pointer;
        transition: all 0.2s;
        font-size: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .delete-entry-btn:hover {
        border-color: #ff6b6b;
        color: #ff6b6b;
        background: #fff5f5;
    }

    /* Dark theme styles */
    @media (prefers-color-scheme: dark) {
        :root:not(.theme-light) .history-dialog {
            background: #2d2d2d;
        }

        :root:not(.theme-light) .history-header {
            border-color: #444;
        }

        :root:not(.theme-light) .history-header h2 {
            color: #f6f6f6;
        }
        :root:not(.theme-light) .dedupe-btn,
        :root:not(.theme-light) .clear-btn,
        :root:not(.theme-light) .close-btn {
            background: #333;
            border-color: #444;
            color: #ccc;
        }

        :root:not(.theme-light) .dedupe-btn:hover:not(:disabled) {
            background: #4a90e2;
            border-color: #4a90e2;
            color: white;
        }

        :root:not(.theme-light) .close-btn:hover {
            border-color: #666;
            color: #f6f6f6;
        }

        :root:not(.theme-light) .history-item {
            background: #333;
            border-color: #444;
        }

        :root:not(.theme-light) .text-panel {
            background: #2a2a2a;
            border-color: #444;
        }

        :root:not(.theme-light) .text-label {
            color: #ccc;
        }

        :root:not(.theme-light) .text-content {
            color: #f6f6f6;
        }

        :root:not(.theme-light) .copy-btn {
            background: #333;
            border-color: #444;
            color: #ccc;
        }
        :root:not(.theme-light) .copy-btn:hover {
            background: #3a3a3a;
            border-color: #379df1;
            color: #379df1;
        }

        :root:not(.theme-light) .delete-entry-btn {
            background: #333;
            border-color: #444;
            color: #ccc;
        }

        :root:not(.theme-light) .delete-entry-btn:hover {
            background: #2a2a2a;
            border-color: #ff6b6b;
            color: #ff6b6b;
        }

        :root:not(.theme-light) .timestamp {
            color: #aaa;
        }

        :root:not(.theme-light) .loading,
        :root:not(.theme-light) .empty {
            color: #aaa;
        }
    } /* Manual dark theme */
    .history-dialog.theme-dark {
        background: #2d2d2d;
    }

    .history-dialog.theme-dark .history-header {
        border-color: #444;
    }

    .history-dialog.theme-dark .history-header h2 {
        color: #f6f6f6;
    }
    .history-dialog.theme-dark .dedupe-btn,
    .history-dialog.theme-dark .clear-btn,
    .history-dialog.theme-dark .close-btn {
        background: #333;
        border-color: #444;
        color: #ccc;
    }

    .history-dialog.theme-dark .dedupe-btn:hover:not(:disabled) {
        background: #4a90e2;
        border-color: #4a90e2;
        color: white;
    }
    .history-dialog.theme-dark .close-btn:hover {
        border-color: #666;
        color: #f6f6f6;
    }

    .history-dialog.theme-dark .history-item {
        background: #333;
        border-color: #444;
    }

    .history-dialog.theme-dark .text-panel {
        background: #2a2a2a;
        border-color: #444;
    }

    .history-dialog.theme-dark .text-label {
        color: #ccc;
    }

    .history-dialog.theme-dark .text-content {
        color: #f6f6f6;
    }

    .history-dialog.theme-dark .copy-btn {
        background: #333;
        border-color: #444;
        color: #ccc;
    }

    .history-dialog.theme-dark .copy-btn:hover {
        background: #3a3a3a;
        border-color: #379df1;
        color: #379df1;
    }

    .history-dialog.theme-dark .delete-entry-btn {
        background: #333;
        border-color: #444;
        color: #ccc;
    }

    .history-dialog.theme-dark .delete-entry-btn:hover {
        background: #2a2a2a;
        border-color: #ff6b6b;
        color: #ff6b6b;
    }

    .history-dialog.theme-dark .timestamp {
        color: #aaa;
    }

    .history-dialog.theme-dark .loading,
    .history-dialog.theme-dark .empty {
        color: #aaa;
    }

    @media (max-width: 768px) {
        .history-dialog {
            width: 95%;
            max-height: 90vh;
        }

        .text-panels {
            grid-template-columns: 1fr;
        }

        .history-header {
            padding: 16px;
        }

        .history-content {
            padding: 16px;
        }
    }
</style>
