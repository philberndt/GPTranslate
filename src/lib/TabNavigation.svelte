<script lang="ts">
  interface Props {
    activeTab: string
    onTabChange: (tab: string) => void
  }

  let { activeTab, onTabChange }: Props = $props()
</script>

<!-- Tab Navigation (DaisyUI) -->
<div role="tablist">
  <button
    type="button"
    onclick={() => onTabChange("api")}
    role="tab"
    aria-selected={activeTab === "api"}
  >
    API Configuration
  </button>
  <button
    type="button"
    onclick={() => onTabChange("models")}
    role="tab"
    aria-selected={activeTab === "models"}
  >
    Model Management
  </button>
  <button
    type="button"
    onclick={() => onTabChange("behavior")}
    role="tab"
    aria-selected={activeTab === "behavior"}
  >
    App Behavior
  </button>
  <button
    type="button"
    onclick={() => onTabChange("about")}
    role="tab"
    aria-selected={activeTab === "about"}
  >
    About
  </button>
</div>

<style>
  /* No custom CSS added per instructions */
</style>
