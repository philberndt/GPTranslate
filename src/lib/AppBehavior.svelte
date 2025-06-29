<script lang="ts">
    interface Props {
        config: any;
        onConfigChange: (updates: any) => void;
    }

    let { config, onConfigChange }: Props = $props();

    function updateConfig(field: string, value: any) {
        onConfigChange({ [field]: value });
    }
</script>

<!-- App Behavior Section -->
<section class="settings-section">
    <h3><i class="bi bi-sliders"></i>App Behavior</h3>

    <div class="form-group">
        <label for="target-language">Target Language</label>
        <input
            id="target-language"
            type="text"
            value={config.target_language}
            placeholder="English, Spanish, French, etc."
            oninput={(e) =>
                updateConfig(
                    "target_language",
                    (e.target as HTMLInputElement).value,
                )}
        />
        <small>
            Specify the default language to translate to. This language will be
            used in the custom prompt as &#123;target_language&#125;.
        </small>
    </div>

    <div class="form-group">
        <label for="alternative-target-language"
            >Alternative Target Language</label
        >
        <input
            id="alternative-target-language"
            type="text"
            value={config.alternative_target_language}
            placeholder="Norwegian, Spanish, German, etc."
            oninput={(e) =>
                updateConfig(
                    "alternative_target_language",
                    (e.target as HTMLInputElement).value,
                )}
        />
        <small>
            Language to use when the detected language is the same as the target
            language. For example, if you normally translate to English, but the
            input is already English, it will translate to this alternative
            language instead.
        </small>
    </div>

    <div class="form-group">
        <label for="theme">Theme</label>
        <select
            id="theme"
            value={config.theme}
            onchange={(e) =>
                updateConfig("theme", (e.target as HTMLSelectElement).value)}
        >
            <option value="auto">Auto (System)</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
        </select>
    </div>

    <div class="form-group">
        <label for="hotkey">Global Hotkey</label>
        <input
            id="hotkey"
            type="text"
            value={config.hotkey}
            placeholder="Ctrl+Alt+C"
            oninput={(e) =>
                updateConfig("hotkey", (e.target as HTMLInputElement).value)}
        />
        <small> Example: Ctrl+Alt+C, Alt+Shift+T, etc. </small>
    </div>

    <div class="checkbox-group">
        <label class="checkbox-label">
            <input
                type="checkbox"
                checked={config.auto_start}
                onchange={(e) =>
                    updateConfig(
                        "auto_start",
                        (e.target as HTMLInputElement).checked,
                    )}
            />
            <span class="checkmark"></span>
            Start with Windows
        </label>
    </div>

    <div class="checkbox-group">
        <label class="checkbox-label">
            <input
                type="checkbox"
                checked={config.minimize_to_tray}
                onchange={(e) =>
                    updateConfig(
                        "minimize_to_tray",
                        (e.target as HTMLInputElement).checked,
                    )}
            />
            <span class="checkmark"></span>
            Minimize to system tray
        </label>
    </div>
</section>

<!-- Custom Prompt Section -->
<section class="settings-section">
    <h3>
        <i class="bi bi-chat-text"></i>Custom Translation Prompt
    </h3>

    <div class="form-group">
        <label for="custom-prompt">Translation Instructions</label>
        <textarea
            id="custom-prompt"
            value={config.custom_prompt}
            placeholder="Enter custom instructions for the AI translator..."
            class="prompt-textarea"
            rows="8"
            oninput={(e) =>
                updateConfig(
                    "custom_prompt",
                    (e.target as HTMLTextAreaElement).value,
                )}
        ></textarea>
        <small>
            Customize how the AI translates text. You can use these variables in
            your prompt:
            <br />
            <code>&#123;detected_language&#125;</code> - The automatically
            detected source language
            <br />
            <code>&#123;target_language&#125;</code> - Your configured target language
        </small>
    </div>
</section>

<style>
    .settings-section {
        margin: 24px 0;
        min-width: 0;
    }

    .settings-section h3 {
        font-size: 1.1rem;
        color: #333;
        margin: 0 0 16px 0;
        display: flex;
        align-items: center;
        gap: 8px;
        padding-bottom: 8px;
        border-bottom: 1px solid #e0e0e0;
    }

    .form-group {
        margin-bottom: 16px;
        min-width: 0;
    }

    .form-group label {
        display: block;
        font-weight: 500;
        margin-bottom: 4px;
        color: #333;
    }

    .form-group input,
    .form-group select {
        width: 100%;
        max-width: 100%;
        padding: 10px 12px;
        border: 1px solid #ddd;
        border-radius: 6px;
        font-family: inherit;
        font-size: 14px;
        transition: border-color 0.2s;
        box-sizing: border-box;
        min-width: 0;
    }

    .form-group input:focus,
    .form-group select:focus {
        outline: none;
        border-color: #379df1;
        box-shadow: 0 0 0 2px rgba(55, 157, 241, 0.1);
    }

    .form-group small {
        color: #666;
        font-size: 12px;
        margin-top: 4px;
        display: block;
    }

    .form-group small code {
        background: #f1f3f4;
        color: #d73a49;
        padding: 2px 4px;
        border-radius: 3px;
        font-family: "Consolas", "Monaco", "Courier New", monospace;
        font-size: 11px;
    }

    .checkbox-group {
        margin-bottom: 12px;
    }

    .checkbox-label {
        display: flex;
        align-items: center;
        cursor: pointer;
        font-weight: normal;
        user-select: none;
    }

    .checkbox-label input[type="checkbox"] {
        width: auto;
        margin-right: 8px;
    }

    .prompt-textarea {
        width: 100%;
        max-width: 100%;
        padding: 12px;
        border: 1px solid #ddd;
        border-radius: 6px;
        font-family: "Consolas", "Monaco", "Courier New", monospace;
        font-size: 13px;
        line-height: 1.4;
        resize: vertical;
        min-height: 120px;
        transition: border-color 0.2s;
        box-sizing: border-box;
    }

    .prompt-textarea:focus {
        outline: none;
        border-color: #379df1;
        box-shadow: 0 0 0 2px rgba(55, 157, 241, 0.1);
    }

    /* Dark mode */
    @media (prefers-color-scheme: dark) {
        .settings-section h3 {
            color: #f6f6f6;
            border-color: #444;
        }

        .form-group label {
            color: #f6f6f6;
        }

        .form-group input,
        .form-group select {
            background: #3a3a3a;
            border-color: #555;
            color: #f6f6f6;
        }

        .form-group input:focus,
        .form-group select:focus {
            border-color: #379df1;
            background: #404040;
        }

        .form-group small {
            color: #ccc;
        }

        .form-group small code {
            background: #444;
            color: #7dd3fc;
        }

        .prompt-textarea {
            background: #404040;
            border-color: #444;
            color: #f6f6f6;
        }

        .prompt-textarea:focus {
            border-color: #379df1;
            background: #4a4a4a;
        }
    }

    @media (max-width: 768px) {
        .settings-section {
            margin: 16px 0;
        }
    }
</style>
