<script lang="ts">
  interface Props {
    config: any
    onConfigChange: (updates: any) => void
    theme: string
    onThemeChange: (theme: string) => void
  }

  let { config, onConfigChange, theme, onThemeChange }: Props = $props()

  function updateConfig(field: string, value: any) {
    onConfigChange({ [field]: value })
  }
</script>

<!-- App Behavior Section -->
<div class="space-y-6">
  <h3 class="text-lg font-semibold mb-4">App Behavior</h3>

  <!-- Theme Section -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h4 class="card-title text-lg">Theme Settings</h4>

      <div class="form-control w-full">
        <label class="label" for="theme">
          <span class="label-text font-medium">Theme</span>
        </label>
        <select
          id="theme"
          class="select select-bordered bg-base-200 w-full max-w-sm"
          value={theme}
          onchange={(e) => {
            const target = e.target as HTMLSelectElement
            onThemeChange(target.value)
          }}
        >
          <option value="auto">Auto (System)</option>
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
        <div class="label">
          <span class="label-text-alt text-base-content/70"
            >Theme will apply immediately</span
          >
        </div>
      </div>
    </div>
  </div>

  <!-- Global Hotkey Section -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h4 class="card-title text-lg">Global Hotkey</h4>

      <div class="form-control w-full">
        <label class="label" for="hotkey">
          <span class="label-text font-medium">Hotkey Combination</span>
        </label>
        <input
          id="hotkey"
          type="text"
          class="input input-bordered bg-base-200 w-full max-w-sm"
          value={config.hotkey}
          placeholder="Ctrl+Alt+C"
          oninput={(e) =>
            updateConfig("hotkey", (e.target as HTMLInputElement).value)}
        />
        <div class="label">
          <span class="label-text-alt text-base-content/70"
            >Example: Ctrl+Alt+C, Alt+Shift+T, etc.</span
          >
        </div>
      </div>
    </div>
  </div>

  <!-- AI Model Settings -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h4 class="card-title text-lg">AI Model Settings</h4>

      <div class="form-control w-full">
        <label class="label" for="reasoning-effort">
          <span class="label-text font-medium">Reasoning Effort</span>
        </label>
        <select
          id="reasoning-effort"
          class="select select-bordered bg-base-200 w-full max-w-sm"
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
        <div class="label">
          <span class="label-text-alt text-base-content/70">
            Applies to reasoning models (o1/o3/o4-mini/GPT-5). Not used by
            non-reasoning models.
          </span>
        </div>
      </div>
    </div>
  </div>

  <!-- Automatic Translation Settings -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h4 class="card-title text-lg">Automatic Translation</h4>

      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text font-medium"
            >Enable automatic translations</span
          >
          <input
            id="auto-translate-enabled"
            type="checkbox"
            class="toggle toggle-primary"
            checked={config.auto_translate_enabled}
            onchange={(e) =>
              updateConfig(
                "auto_translate_enabled",
                (e.target as HTMLInputElement).checked
              )}
          />
        </label>
      </div>

      <div class="form-control w-full">
        <label class="label" for="debounce-ms">
          <span class="label-text font-medium">Debounce delay</span>
          <span class="label-text-alt text-base-content/70"
            >{config.auto_translate_debounce_ms} ms</span
          >
        </label>
        <input
          id="debounce-ms"
          type="range"
          class="range range-primary"
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
        <div class="label">
          <span class="label-text-alt text-base-content/70"
            >Delay after typing stops</span
          >
        </div>
      </div>

      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text font-medium"
            >Translate automatically when text is pasted</span
          >
          <input
            id="auto-translate-on-paste"
            type="checkbox"
            class="toggle toggle-primary"
            disabled={!config.auto_translate_enabled}
            checked={config.auto_translate_on_paste}
            onchange={(e) =>
              updateConfig(
                "auto_translate_on_paste",
                (e.target as HTMLInputElement).checked
              )}
          />
        </label>
      </div>

      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text font-medium"
            >Translate automatically while typing</span
          >
          <input
            id="auto-translate-while-typing"
            type="checkbox"
            class="toggle toggle-primary"
            disabled={!config.auto_translate_enabled}
            checked={config.auto_translate_while_typing}
            onchange={(e) =>
              updateConfig(
                "auto_translate_while_typing",
                (e.target as HTMLInputElement).checked
              )}
          />
        </label>
      </div>
    </div>
  </div>

  <!-- Startup Settings -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h4 class="card-title text-lg">Startup & System</h4>

      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text font-medium">Start with Windows</span>
          <input
            id="auto-start"
            type="checkbox"
            class="toggle toggle-primary"
            checked={config.auto_start}
            onchange={(e) =>
              updateConfig(
                "auto_start",
                (e.target as HTMLInputElement).checked
              )}
          />
        </label>
      </div>

      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text font-medium">Minimize to system tray</span>
          <input
            id="minimize-to-tray"
            type="checkbox"
            class="toggle toggle-primary"
            checked={config.minimize_to_tray}
            onchange={(e) =>
              updateConfig(
                "minimize_to_tray",
                (e.target as HTMLInputElement).checked
              )}
          />
        </label>
      </div>
    </div>
  </div>

  <!-- Custom Translation Prompt -->
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body">
      <h4 class="card-title text-lg">Custom Translation Prompt</h4>

      <div class="form-control">
        <label class="label" for="custom-prompt">
          <span class="label-text font-medium">Translation Instructions</span>
        </label>
        <textarea
          id="custom-prompt"
          class="textarea textarea-bordered bg-base-200 h-32"
          value={config.custom_prompt}
          placeholder="Enter custom instructions for the AI translator..."
          oninput={(e) =>
            updateConfig(
              "custom_prompt",
              (e.target as HTMLTextAreaElement).value
            )}
        ></textarea>
        <div class="label">
          <span class="label-text-alt text-base-content/70">
            Customize how the AI translates text. You can use these variables:<br
            />
            <code class="text-xs bg-base-300/50 px-1 py-0.5 rounded"
              >&#123;detected_language&#125;</code
            >
            - The automatically detected source language<br />
            <code class="text-xs bg-base-300/50 px-1 py-0.5 rounded"
              >&#123;target_language&#125;</code
            > - Your configured target language
          </span>
        </div>
      </div>
    </div>
  </div>
</div>
