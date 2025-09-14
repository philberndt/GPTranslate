<script lang="ts">
  import {
    SunIcon,
    KeyIcon,
    Cog6ToothIcon,
    SparklesIcon,
    ComputerDesktopIcon,
    PencilSquareIcon,
    AdjustmentsHorizontalIcon,
  } from "heroicons-svelte/24/outline"

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

<div class="ml-10 mr-10 overflow-hidden">
  <div class="card bg-base-100 border border-base-300/50">
    <div class="card-body space-y-8">
      <h5 class="card-title flex items-center gap-2 mb-2">
        <AdjustmentsHorizontalIcon class="w-5 h-5" />
        App Behavior
      </h5>

      <div class="grid md:grid-cols-3 gap-8">
        <!-- Theme Settings -->
        <div class="space-y-4">
          <h5 class="flex items-center gap-2 font-medium">
            <SunIcon class="w-5 h-5" />
            Theme Settings
          </h5>
          <div class="space-y-4 mx-8">
            <div class="form-control w-full">
              <label class="label" for="theme">
                <span class="label-text font-medium">Theme</span>
              </label>
              <select
                id="theme"
                class="select select-bordered bg-base-200 w-full"
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

        <!-- Global Hotkey -->
        <div class="space-y-4">
          <h5 class="flex items-center gap-2 font-medium">
            <KeyIcon class="w-5 h-5" />
            Global Hotkey
          </h5>
          <div class="space-y-4 mx-8">
            <div class="form-control w-full">
              <label class="label" for="hotkey">
                <span class="label-text font-medium">Hotkey Combination</span>
              </label>
              <input
                id="hotkey"
                type="text"
                class="input input-bordered bg-base-200 w-full"
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
        <div class="space-y-4">
          <h5 class="flex items-center gap-2 font-medium">
            <Cog6ToothIcon class="w-5 h-5" />
            AI Model Settings
          </h5>
          <div class="space-y-4 mx-8">
            <div class="form-control w-full">
              <label class="label" for="reasoning-effort">
                <span class="label-text font-medium">Reasoning Effort</span>
              </label>
              <select
                id="reasoning-effort"
                class="select select-bordered bg-base-200 w-full"
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
                <span class="label-text-alt text-base-content/70"
                  >Applies to reasoning models (o1/o3/o4-mini/GPT-5)</span
                >
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Secondary grid: Automatic Translation + Startup & System (2 columns) -->
      <div class="grid md:grid-cols-2 gap-8">
        <!-- Automatic Translation -->
        <div class="space-y-4">
          <h5 class="flex items-center gap-2 font-medium">
            <SparklesIcon class="w-5 h-5" />
            Automatic Translation
          </h5>
          <div class="space-y-4 mx-8">
            <div class="form-control">
              <label
                class="label cursor-pointer justify-start gap-3"
                for="auto-translate-enabled"
              >
                <input
                  id="auto-translate-enabled"
                  type="checkbox"
                  class="toggle toggle-xs"
                  checked={config.auto_translate_enabled}
                  onchange={(e) =>
                    updateConfig(
                      "auto_translate_enabled",
                      (e.target as HTMLInputElement).checked
                    )}
                />
                <span class="label-text font-medium"
                  >Enable automatic translations</span
                >
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
                class="range range-xs"
                style="--range-shdw:0 0;"
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
              <label
                class="label cursor-pointer justify-start gap-3"
                for="auto-translate-on-paste"
              >
                <input
                  id="auto-translate-on-paste"
                  type="checkbox"
                  class="toggle toggle-xs"
                  disabled={!config.auto_translate_enabled}
                  checked={config.auto_translate_on_paste}
                  onchange={(e) =>
                    updateConfig(
                      "auto_translate_on_paste",
                      (e.target as HTMLInputElement).checked
                    )}
                />
                <span class="label-text font-medium"
                  >Translate automatically when text is pasted</span
                >
              </label>
            </div>
            <div class="form-control">
              <label
                class="label cursor-pointer justify-start gap-3"
                for="auto-translate-while-typing"
              >
                <input
                  id="auto-translate-while-typing"
                  type="checkbox"
                  class="toggle toggle-xs"
                  disabled={!config.auto_translate_enabled}
                  checked={config.auto_translate_while_typing}
                  onchange={(e) =>
                    updateConfig(
                      "auto_translate_while_typing",
                      (e.target as HTMLInputElement).checked
                    )}
                />
                <span class="label-text font-medium"
                  >Translate automatically while typing</span
                >
              </label>
            </div>
          </div>
        </div>

        <!-- Startup & System -->
        <div class="space-y-4">
          <h5 class="flex items-center gap-2 font-medium">
            <ComputerDesktopIcon class="w-5 h-5" />
            Startup & System
          </h5>
          <div class="space-y-4 mx-8">
            <div class="form-control">
              <label
                class="label cursor-pointer justify-start gap-3"
                for="auto-start"
              >
                <input
                  id="auto-start"
                  type="checkbox"
                  class="toggle toggle-xs"
                  checked={config.auto_start}
                  onchange={(e) =>
                    updateConfig(
                      "auto_start",
                      (e.target as HTMLInputElement).checked
                    )}
                />
                <span class="label-text font-medium">Start with Windows</span>
              </label>
            </div>
            <div class="form-control">
              <label
                class="label cursor-pointer justify-start gap-3"
                for="minimize-to-tray"
              >
                <input
                  id="minimize-to-tray"
                  type="checkbox"
                  class="toggle toggle-xs"
                  checked={config.minimize_to_tray}
                  onchange={(e) =>
                    updateConfig(
                      "minimize_to_tray",
                      (e.target as HTMLInputElement).checked
                    )}
                />
                <span class="label-text font-medium"
                  >Minimize to system tray</span
                >
              </label>
            </div>
          </div>
        </div>
      </div>

      <div class="divider my-6"></div>

      <!-- Custom Translation Prompt (Full Width) -->
      <div class="space-y-4 w-full">
        <h5 class="card-title flex items-center gap-2 mb-2">
          <PencilSquareIcon class="w-5 h-5" />
          Custom Translation Prompt
        </h5>
        <div class="space-y-4 mx-8">
          <div class="form-control w-full">
            <label class="label" for="custom-prompt">
              <span class="label-text font-medium"
                >Translation Instructions</span
              >
            </label>
            <textarea
              id="custom-prompt"
              class="textarea textarea-bordered bg-base-200 h-48 w-full max-w-none resize-y"
              value={config.custom_prompt}
              placeholder="Enter custom instructions for the AI translator..."
              oninput={(e) =>
                updateConfig(
                  "custom_prompt",
                  (e.target as HTMLTextAreaElement).value
                )}
            ></textarea>
            <div class="label">
              <span class="label-text-alt text-base-content/70 space-y-1">
                Customize how the AI translates text. Variables:
                <br />
                <code
                  class="text-xs bg-base-300/40 px-1 py-0.5 rounded text-info font-medium"
                  >{`{detected_language}`}</code
                >
                - Detected source language
                <br />
                <code
                  class="text-xs bg-base-300/40 px-1 py-0.5 rounded text-info font-medium"
                  >{`{target_language}`}</code
                >
                - Your configured target language
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
