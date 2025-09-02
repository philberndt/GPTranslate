<script lang="ts">
  interface Props {
    isSaving: boolean
    saveMessage: string
    onSave: () => void
    onReset: () => void
  }

  let { isSaving, saveMessage, onSave, onReset }: Props = $props()
</script>

<div class="">
  <div class="">
    {#if saveMessage}
      <span class="" role="status">
        <i class=""></i>
        {saveMessage}
      </span>
    {/if}
  </div>
  <div class="">
    <button type="button" class="" onclick={onReset}>
      <i class=""></i>Reset to Defaults
    </button>
    <button type="button" class="" onclick={onSave} disabled={isSaving}>
      {#if isSaving}
        <div class="" role="status" aria-hidden="true"></div>
        Saving...
      {:else}
        <i class=""></i>Save Settings
      {/if}
    </button>
  </div>
</div>

<style>
  /* CSS goes in /src/styles.css */
</style>
