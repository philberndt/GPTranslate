<script lang="ts">
  interface Props {
    isSaving: boolean
    saveMessage: string
    onSave: () => void
    onReset: () => void
  }

  let { isSaving, saveMessage, onSave, onReset }: Props = $props()
</script>

<div class="w-full max-w-3xl mx-auto px-4">
  <div class="flex items-center justify-between gap-4">
    {#if saveMessage}
      <span role="status" class="text-sm">{saveMessage}</span>
    {:else}
      <span></span>
    {/if}

    <div class="flex gap-2">
      <button type="button" onclick={onReset} class="btn btn-outline btn-sm">
        Reset to Defaults
      </button>
      <button
        type="button"
        onclick={onSave}
        disabled={isSaving}
        class="btn btn-primary btn-sm"
      >
        {#if isSaving}
          <span role="status" aria-hidden="true" class="loading loading-spinner"
          ></span>
          Saving...
        {:else}
          Save Settings
        {/if}
      </button>
    </div>
  </div>
</div>
<!-- Custom CSS goes in /src/styles.css */ -->
