<script lang="ts">
  import { FaceSmileIcon, RocketLaunchIcon } from "heroicons-svelte/24/outline"

  interface Props {
    onOpenSettings: () => void
  }

  let { onOpenSettings }: Props = $props()

  function handleGetStarted() {
    console.log("Get Started button clicked!")
    onOpenSettings()
  }
</script>

<div
  class="fixed inset-0 bg-base-200 flex items-center justify-center z-50"
  style="background-color: var(--color-app-bg);"
>
  <div class="text-center space-y-8 max-w-md px-6">
    <!-- Welcome Icon -->
    <div class="flex justify-center">
      <FaceSmileIcon class="w-24 h-24 text-primary" aria-hidden="true" />
    </div>

    <!-- Title -->
    <h1 class="text-3xl font-bold text-base-content">
      Welcome to GPTranslate!
    </h1>

    <!-- Message -->
    <div class="space-y-4">
      <p class="text-lg text-base-content/80">
        Let's get you set up! We recommend starting with <strong
          >Azure AI Translator</strong
        > - it's free for up to 2 million characters per month.
      </p>

      <p class="text-base text-base-content/70">
        Or choose from OpenAI, Azure OpenAI, or Ollama based on your needs.
      </p>
    </div>

    <!-- Action Button -->
    <button
      class="btn btn-primary btn-lg gap-2"
      onclick={handleGetStarted}
      type="button"
      aria-label="Open settings to configure a translation provider"
    >
      <RocketLaunchIcon class="w-6 h-6" />
      Get Started
    </button>

    <!-- Help Text -->
    <div class="text-sm text-base-content/60 space-y-2">
      <p>
        <strong>New to Azure?</strong>
        <a
          href="https://azure.microsoft.com/en-ca/pricing/purchase-options/azure-account?icid=azurefreeaccount"
          class="link link-accent"
          target="_blank">Create a free account</a
        > - no credit card required for the free tier!
      </p>
    </div>
  </div>
</div>
