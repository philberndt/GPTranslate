<script lang="ts">
  interface Props {
    activeTab: string
    onTabChange: (tab: string) => void
  }

  let { activeTab, onTabChange }: Props = $props()
</script>

<!-- Tab Navigation (DaisyUI) -->
<div class="" role="tablist">
  <button
    type="button"
    class=""
    onclick="{() => onTabChange('api')}"
    role="tab"
    aria-selected="{activeTab === 'api'}"
  >
    API Configuration
  </button>
  <button
    type="button"
    class=""
    onclick="{() => onTabChange('models')}"
    role="tab"
    aria-selected="{activeTab === 'models'}"
  >
    Model Management
  </button>
  <button
    type="button"
    class=""
    onclick="{() => onTabChange('behavior')}"
    role="tab"
    aria-selected="{activeTab === 'behavior'}"
  >
    App Behavior
  </button>
  <button
    type="button"
    class=""
    onclick="{() => onTabChange('about')}"
    role="tab"
    aria-selected="{activeTab === 'about'}"
  >
    About
  </button>
</div>

<style>
  /* No custom CSS added per instructions */
</style>
