<script lang="ts">
  interface Props {
    isSaving: boolean
    saveMessage: string
    onSave: () => void
    onReset: () => void
  }

  let { isSaving, saveMessage, onSave, onReset }: Props = $props()
</script>

<div>
  <div>
    {#if saveMessage}
      <span role="status">
        {saveMessage}
      </span>
    {/if}
  </div>
  <div>
    <button type="button" onclick={onReset}> Reset to Defaults </button>
    <button type="button" onclick={onSave} disabled={isSaving}>
      {#if isSaving}
        <div role="status" aria-hidden="true"></div>
        Saving...
      {:else}
        Save Settings
      {/if}
    </button>
  </div>
</div>

<!-- Custom CSS goes in /src/styles.css */ -->
