<script lang="ts">
  interface Props {
    activeTab: string
    onTabChange: (tab: string) => void
  }

  let { activeTab, onTabChange }: Props = $props()
</script>

<!-- Tab Navigation -->
<ul class="" role="tablist">
  <li class="" role="presentation">
    <button
      class=""
      type="button"
      onclick={() => onTabChange("api")}
      role="tab"
      aria-selected={activeTab === "api"}
    >
      <i class=""></i>
      API Configuration
    </button>
  </li>
  <li class="" role="presentation">
    <button
      class=""
      type="button"
      onclick={() => onTabChange("models")}
      role="tab"
      aria-selected={activeTab === "models"}
    >
      <i class=""></i>
      Model Management
    </button>
  </li>
  <li class="" role="presentation">
    <button
      class=""
      type="button"
      onclick={() => onTabChange("behavior")}
      role="tab"
      aria-selected={activeTab === "behavior"}
    >
      <i class=""></i>
      App Behavior
    </button>
  </li>
  <li class="" role="presentation">
    <button
      class=""
      type="button"
      onclick={() => onTabChange("about")}
      role="tab"
      aria-selected={activeTab === "about"}
    >
      <i class=""></i>
      About
    </button>
  </li>
</ul>

<style>
  /* CSS moved to /src/app.css */
</style>
