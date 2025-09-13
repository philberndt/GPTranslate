<script lang="ts">
  import {
    XMarkIcon,
    WrenchScrewdriverIcon,
    CpuChipIcon,
    GlobeAltIcon,
    CogIcon,
    InformationCircleIcon,
  } from "heroicons-svelte/24/outline"

  // Props
  let { onClose, theme } = $props<{
    onClose: () => void
    theme: string
  }>()

  // Tab management
  let activeTab = $state("api")

  function setActiveTab(tab: string) {
    activeTab = tab
  }
</script>

<div data-theme={theme} class="min-h-screen bg-base-100">
  <div class="max-w-6xl mx-auto p-6">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <h1 class="text-3xl font-bold text-base-content">Settings</h1>
      </div>
      <button
        class="btn btn-soft btn-circle btn-sm"
        onclick={onClose}
        title="Close settings"
        aria-label="Close settings"
      >
        <XMarkIcon class="w-5 h-5" />
      </button>
    </div>

    <!-- Navigation Tabs -->
    <div role="tablist" class="tabs tabs-lift mb-6">
      <input
        id="tab-api"
        type="radio"
        name="settings-tabs"
        class="tab"
        aria-label="API Configuration"
        checked={activeTab === "api"}
        onchange={() => setActiveTab("api")}
      />
      <label for="tab-api" class="tab">
        <WrenchScrewdriverIcon class="w-5 h-5 inline mr-2" /> API Configuration
      </label>
      <input
        id="tab-models"
        type="radio"
        name="settings-tabs"
        class="tab"
        aria-label="Models"
        checked={activeTab === "models"}
        onchange={() => setActiveTab("models")}
      />
      <label for="tab-models" class="tab">
        <CpuChipIcon class="w-5 h-5 inline mr-2" /> Models
      </label>
      <input
        id="tab-languages"
        type="radio"
        name="settings-tabs"
        class="tab"
        aria-label="Languages"
        checked={activeTab === "languages"}
        onchange={() => setActiveTab("languages")}
      />
      <label for="tab-languages" class="tab">
        <GlobeAltIcon class="w-5 h-5 inline mr-2" /> Languages
      </label>
      <input
        id="tab-behavior"
        type="radio"
        name="settings-tabs"
        class="tab"
        aria-label="Behavior"
        checked={activeTab === "behavior"}
        onchange={() => setActiveTab("behavior")}
      />
      <label for="tab-behavior" class="tab">
        <CogIcon class="w-5 h-5 inline mr-2" /> Behavior
      </label>
      <input
        id="tab-about"
        type="radio"
        name="settings-tabs"
        class="tab"
        aria-label="About"
        checked={activeTab === "about"}
        onchange={() => setActiveTab("about")}
      />
      <label for="tab-about" class="tab">
        <InformationCircleIcon class="w-5 h-5 inline mr-2" /> About
      </label>
    </div>

    <!-- Tab Content -->
    <div class="card bg-base-100 border border-base-content/20">
      <div class="card-body p-6">
        {#if activeTab === "api"}
          <div class="space-y-4">
            <h2 class="text-xl font-semibold text-base-content">
              API Configuration
            </h2>
            <p class="text-base-content/70">
              Configure your translation API settings here.
            </p>

            <div class="form-control w-full max-w-xs">
              <label class="label" for="provider">
                <span class="label-text">Provider</span>
              </label>
              <select class="select select-bordered bg-base-200" id="provider">
                <option selected>OpenAI</option>
                <option>Azure OpenAI</option>
              </select>
            </div>

            <div class="form-control w-full">
              <label class="label" for="api-key">
                <span class="label-text">API Key</span>
              </label>
              <input
                type="password"
                placeholder="Enter your API key"
                class="input input-bordered bg-base-200"
                id="api-key"
              />
            </div>
          </div>
        {:else if activeTab === "models"}
          <div class="space-y-4">
            <h2 class="text-xl font-semibold text-base-content">
              Model Management
            </h2>
            <p class="text-base-content/70">
              Manage your available translation models.
            </p>

            <div class="alert alert-info">
              <span>Model management features will be available here.</span>
            </div>
          </div>
        {:else if activeTab === "languages"}
          <div class="space-y-4">
            <h2 class="text-xl font-semibold text-base-content">
              Language Settings
            </h2>
            <p class="text-base-content/70">
              Configure your preferred translation languages.
            </p>

            <div class="form-control w-full max-w-xs">
              <label class="label" for="primary-lang">
                <span class="label-text">Primary Target Language</span>
              </label>
              <select
                id="primary-lang"
                class="select select-bordered bg-base-200"
              >
                <option selected>English</option>
                <option>Spanish</option>
                <option>French</option>
                <option>German</option>
              </select>
            </div>

            <div class="form-control w-full max-w-xs">
              <label class="label" for="alt-lang">
                <span class="label-text">Alternative Target Language</span>
              </label>
              <select id="alt-lang" class="select select-bordered bg-base-200">
                <option>Spanish</option>
                <option selected>French</option>
                <option>German</option>
                <option>Italian</option>
              </select>
            </div>
          </div>
        {:else if activeTab === "behavior"}
          <div class="space-y-4">
            <h2 class="text-xl font-semibold text-base-content">
              App Behavior
            </h2>
            <p class="text-base-content/70">
              Customize how the application behaves.
            </p>

            <div class="form-control">
              <label class="cursor-pointer label">
                <span class="label-text">Auto-copy translations</span>
                <input type="checkbox" class="toggle toggle-primary" />
              </label>
            </div>

            <div class="form-control">
              <label class="cursor-pointer label">
                <span class="label-text">Keep app open after translation</span>
                <input type="checkbox" class="toggle toggle-primary" checked />
              </label>
            </div>

            <div class="form-control">
              <label class="cursor-pointer label">
                <span class="label-text">Enable global hotkey</span>
                <input type="checkbox" class="toggle toggle-primary" checked />
              </label>
            </div>
          </div>
        {:else if activeTab === "about"}
          <div class="space-y-4">
            <h2 class="text-xl font-semibold text-base-content">
              About GPTranslate
            </h2>
            <p class="text-base-content/70">Version information and credits.</p>

            <div class="stats shadow">
              <div class="stat">
                <div class="stat-title">Version</div>
                <div class="stat-value text-lg">1.0.0</div>
                <div class="stat-desc">GPTranslate Desktop</div>
              </div>
            </div>

            <div class="alert alert-info">
              <span>Built with Tauri, SvelteKit, and DaisyUI</span>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
