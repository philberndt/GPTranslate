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
<div class="">
  <h4 class=""><i class=""></i>App Behavior</h4>

  <div class="">
    <div class="">
      <label for="theme" class="">Theme</label>
      <select
        id="theme"
        class=""
        value={config.theme}
        onchange={(e) =>
          updateConfig("theme", (e.target as HTMLSelectElement).value)}
      >
        <option value="auto">Auto (System)</option>
        <option value="light">Light</option>
        <option value="dark">Dark</option>
      </select>
    </div>
    <div class="">
      <label for="hotkey" class="">Global Hotkey</label>
      <input
        id="hotkey"
        type="text"
        class=""
        value={config.hotkey}
        placeholder="Ctrl+Alt+C"
        oninput={(e) =>
          updateConfig("hotkey", (e.target as HTMLInputElement).value)}
      />
      <div class="">Example: Ctrl+Alt+C, Alt+Shift+T, etc.</div>
    </div>
  </div>

  <div class="">
    <div class="">
      <label for="reasoning-effort" class="">Reasoning Effort</label>
      <select
        id="reasoning-effort"
        class=""
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
      <div class="">
        Applies to reasoning models (o1/o3/o4-mini/GPT-5). Not used by
        non-reasoning models.
      </div>
    </div>
  </div>

  <!-- Automatic Translation Settings -->
  <fieldset class="">
    <legend class=""><i class=""></i>Automatic Translation</legend>
    <div class="">
      <input
        id="auto-translate-enabled"
        type="checkbox"
        class=""
        role="switch"
        checked={config.auto_translate_enabled}
        onchange={(e) =>
          updateConfig(
            "auto_translate_enabled",
            (e.target as HTMLInputElement).checked
          )}
      />
      <label class="" for="auto-translate-enabled">
        Enable automatic translations
      </label>
    </div>

    <div class="">
      <div class="">
        <label for="debounce-ms" class="">Debounce (ms)</label>
        <input
          id="debounce-ms"
          type="range"
          class=""
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
        <div class="">
          {config.auto_translate_debounce_ms} ms delay after typing stops
        </div>
      </div>
    </div>

    <div class="">
      <input
        id="auto-translate-on-paste"
        type="checkbox"
        class=""
        disabled={!config.auto_translate_enabled}
        checked={config.auto_translate_on_paste}
        onchange={(e) =>
          updateConfig(
            "auto_translate_on_paste",
            (e.target as HTMLInputElement).checked
          )}
      />
      <label class="" for="auto-translate-on-paste">
        Translate automatically when text is pasted
      </label>
    </div>
    <div class="">
      <input
        id="auto-translate-while-typing"
        type="checkbox"
        class=""
        disabled={!config.auto_translate_enabled}
        checked={config.auto_translate_while_typing}
        onchange={(e) =>
          updateConfig(
            "auto_translate_while_typing",
            (e.target as HTMLInputElement).checked
          )}
      />
      <label class="" for="auto-translate-while-typing">
        Translate automatically while typing
      </label>
    </div>
  </fieldset>

  <div class="">
    <input
      id="auto-start"
      type="checkbox"
      class=""
      checked={config.auto_start}
      onchange={(e) =>
        updateConfig("auto_start", (e.target as HTMLInputElement).checked)}
    />
    <label class="" for="auto-start"> Start with Windows </label>
  </div>

  <div class="">
    <input
      id="minimize-to-tray"
      type="checkbox"
      class=""
      checked={config.minimize_to_tray}
      onchange={(e) =>
        updateConfig(
          "minimize_to_tray",
          (e.target as HTMLInputElement).checked
        )}
    />
    <label class="" for="minimize-to-tray"> Minimize to system tray </label>
  </div>
</div>

<hr class="" />

<!-- Custom Prompt Section -->
<div class="">
  <h4 class="">
    <i class=""></i>Custom Translation Prompt
  </h4>

  <div class="">
    <label for="custom-prompt" class="">Translation Instructions</label>
    <textarea
      id="custom-prompt"
      class=""
      value={config.custom_prompt}
      placeholder="Enter custom instructions for the AI translator..."
      rows="8"
      oninput={(e) =>
        updateConfig("custom_prompt", (e.target as HTMLTextAreaElement).value)}
    ></textarea>
    <div class="">
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

<style>
  /* CSS goes in /src/styles.css */
</style>
