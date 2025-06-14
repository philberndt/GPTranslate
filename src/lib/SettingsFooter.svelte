<script lang="ts">
    interface Props {
        isSaving: boolean;
        saveMessage: string;
        onSave: () => void;
        onReset: () => void;
    }

    let { isSaving, saveMessage, onSave, onReset }: Props = $props();
</script>

<div class="settings-footer">
    {#if saveMessage}
        <div
            class="save-message"
            class:success={saveMessage.includes("successfully")}
            class:error={!saveMessage.includes("successfully")}
        >
            {saveMessage}
        </div>
    {/if}
    <div class="settings-actions">
        <button class="reset-btn" onclick={onReset}>
            <i class="bi bi-arrow-counterclockwise"></i>Reset to Defaults
        </button>
        <button class="save-btn" onclick={onSave} disabled={isSaving}>
            {#if isSaving}
                <i class="bi bi-arrow-clockwise spinning"></i>Saving...
            {:else}
                <i class="bi bi-check-lg"></i>Save Settings
            {/if}
        </button>
    </div>
</div>

<style>
    .settings-footer {
        padding: 20px 24px;
        border-top: 1px solid #e0e0e0;
        flex-shrink: 0;
    }

    .save-message {
        padding: 8px 12px;
        border-radius: 6px;
        margin-bottom: 12px;
        font-size: 14px;
    }

    .save-message.success {
        background: #d4edda;
        color: #155724;
        border: 1px solid #c3e6cb;
    }

    .save-message.error {
        background: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }

    .settings-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
    }

    .reset-btn,
    .save-btn {
        padding: 10px 20px;
        border: none;
        border-radius: 6px;
        font-family: inherit;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .reset-btn {
        background: #6c757d;
        color: white;
    }

    .reset-btn:hover {
        background: #5a6268;
    }

    .save-btn {
        background: #379df1;
        color: white;
    }

    .save-btn:hover:not(:disabled) {
        background: #2980e6;
    }

    .save-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .spinning {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    /* Dark mode */
    @media (prefers-color-scheme: dark) {
        .settings-footer {
            border-color: #444;
        }

        .save-message.success {
            background: #1e4620;
            color: #a3cfac;
            border-color: #2d5930;
        }

        .save-message.error {
            background: #4a1e23;
            color: #f5c6cb;
            border-color: #5c2329;
        }
    }

    @media (max-width: 768px) {
        .settings-actions {
            flex-direction: column;
        }

        .reset-btn,
        .save-btn {
            width: 100%;
            justify-content: center;
        }
    }
</style>
