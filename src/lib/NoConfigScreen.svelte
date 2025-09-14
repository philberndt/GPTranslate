<script lang="ts">
  import { CogIcon, ExclamationTriangleIcon } from "heroicons-svelte/24/outline"

  interface Props {
    onOpenSettings: () => void
  }

  let { onOpenSettings }: Props = $props()
</script>

<div class="fixed inset-0 bg-base-200 flex items-center justify-center z-50">
  <div class="text-center space-y-8 max-w-md px-6">
    <!-- Warning Icon -->
    <div class="flex justify-center">
      <ExclamationTriangleIcon 
        class="w-24 h-24 text-warning" 
        aria-hidden="true" 
      />
    </div>

    <!-- Title -->
    <h1 class="text-3xl font-bold text-base-content">
      Welcome to GPTranslate!
    </h1>

    <!-- Message -->
    <div class="space-y-4">
      <p class="text-lg text-base-content/80">
        To get started, you need to configure at least one translation API provider.
      </p>
      
      <p class="text-base text-base-content/70">
        Choose from OpenAI, Azure OpenAI, Azure Translator, or Ollama to begin translating text.
      </p>
    </div>

    <!-- Action Button -->
    <button 
      class="btn btn-primary btn-lg gap-2"
      onclick={onOpenSettings}
      type="button"
    >
      <CogIcon class="w-6 h-6" />
      Configure APIs
    </button>

    <!-- Help Text -->
    <p class="text-sm text-base-content/60">
      Don't have an API key? Visit your provider's website to get started.
    </p>
  </div>
</div>