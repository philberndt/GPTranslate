<script lang="ts">
  interface Props {
    config: any
    onConfigChange: (updates: any) => void
  }

  let { config, onConfigChange }: Props = $props()

  function updateConfig(field: string, value: any) {
    onConfigChange({ [field]: value })
  }
</script>

<!-- App Behavior Section -->
<div>
  <h4>App Behavior</h4>

  <div>
    <div>
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
    <div>
      <label for="hotkey">Global Hotkey</label>
      <input
        id="hotkey"
        type="text"
        value={config.hotkey}
        placeholder="Ctrl+Alt+C"
        oninput={(e) =>
          updateConfig("hotkey", (e.target as HTMLInputElement).value)}
      />
      <div>Example: Ctrl+Alt+C, Alt+Shift+T, etc.</div>
    </div>
  </div>

  <div>
    <div>
      <label for="reasoning-effort">Reasoning Effort</label>
      <select
        id="reasoning-effort"
        value={config.reasoning_effort || "medium"}
        onchange={(e) =>
          updateConfig(
            "reasoning_effort",
            (e.target as HTMLSelectElement).value
          )}
      >
        <option value="minimal">Minimal (GPT-5 only)</option>
        <option value="low">Low</option>
        <option value="medium">Medium</option>
        <option value="high">High</option>
      </select>
      <div>
        Applies to reasoning models (o1/o3/o4-mini/GPT-5). Not used by
        non-reasoning models.
      </div>
    </div>
  </div>

  <!-- Automatic Translation Settings -->
  <fieldset>
    <legend>Automatic Translation</legend>
    <div>
      <input
        id="auto-translate-enabled"
        type="checkbox"
        role="switch"
        checked={config.auto_translate_enabled}
        onchange={(e) =>
          updateConfig(
            "auto_translate_enabled",
            (e.target as HTMLInputElement).checked
          )}
      />
      <label for="auto-translate-enabled">
        Enable automatic translations
      </label>
    </div>

    <div>
      <div>
        <label for="debounce-ms">Debounce (ms)</label>
        <input
          id="debounce-ms"
          type="range"
          min="100"
          max="2000"
          step="50"
          disabled={!config.auto_translate_enabled}
          value={config.auto_translate_debounce_ms}
          oninput={(e) =>
            updateConfig(
              "auto_translate_debounce_ms",
              parseInt((e.target as HTMLInputElement).value)
            )}
        />
        <div>
          {config.auto_translate_debounce_ms} ms delay after typing stops
        </div>
      </div>
    </div>

    <div>
      <input
        id="auto-translate-on-paste"
        type="checkbox"
        disabled={!config.auto_translate_enabled}
        checked={config.auto_translate_on_paste}
        onchange={(e) =>
          updateConfig(
            "auto_translate_on_paste",
            (e.target as HTMLInputElement).checked
          )}
      />
      <label for="auto-translate-on-paste">
        Translate automatically when text is pasted
      </label>
    </div>
    <div>
      <input
        id="auto-translate-while-typing"
        type="checkbox"
        disabled={!config.auto_translate_enabled}
        checked={config.auto_translate_while_typing}
        onchange={(e) =>
          updateConfig(
            "auto_translate_while_typing",
            (e.target as HTMLInputElement).checked
          )}
      />
      <label for="auto-translate-while-typing">
        Translate automatically while typing
      </label>
    </div>
  </fieldset>

  <div>
    <input
      id="auto-start"
      type="checkbox"
      checked={config.auto_start}
      onchange={(e) =>
        updateConfig("auto_start", (e.target as HTMLInputElement).checked)}
    />
    <label for="auto-start"> Start with Windows </label>
  </div>

  <div>
    <input
      id="minimize-to-tray"
      type="checkbox"
      checked={config.minimize_to_tray}
      onchange={(e) =>
        updateConfig(
          "minimize_to_tray",
          (e.target as HTMLInputElement).checked
        )}
    />
    <label for="minimize-to-tray"> Minimize to system tray </label>
  </div>
</div>

<hr />

<!-- Custom Prompt Section -->
<div>
  <h4>Custom Translation Prompt</h4>

  <div>
    <label for="custom-prompt">Translation Instructions</label>
    <textarea
      id="custom-prompt"
      value={config.custom_prompt}
      placeholder="Enter custom instructions for the AI translator..."
      rows="8"
      oninput={(e) =>
        updateConfig("custom_prompt", (e.target as HTMLTextAreaElement).value)}
    ></textarea>
    <div>
      Customize how the AI translates text. You can use these variables in your
      prompt:
      <br />
      <code>&#123;detected_language&#125;</code> - The automatically detected
      source language
      <br />
      <code>&#123;target_language&#125;</code> - Your configured target language
    </div>
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
