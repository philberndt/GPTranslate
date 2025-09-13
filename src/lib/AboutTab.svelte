<script lang="ts">
  import AppIcon from "./AppIcon.svelte"
  interface Props {
    version: string
  }
  let { version }: Props = $props()
</script>

<!-- About Section -->
<section class="flex flex-col items-center gap-4">
  <AppIcon size={56} className="" />
  <h2 class="text-2xl font-bold">GPTranslate</h2>
  <span class="text-sm text-base-content/70">v{version}</span>
  <h4 class="text-lg font-semibold mt-2">About</h4>
  <div class="flex flex-col gap-2 mt-2">
    <div>
      <strong>Developer:</strong>
      <span>Phil Berndt</span>
    </div>
    <div>
      <strong>Email:</strong>
      <span>phil@berndt.no</span>
    </div>
    <div>
      <strong>Website:</strong>
      <a
        href="https://gptranslate.berndt.no"
        target="_blank"
        rel="noopener noreferrer"
        class="link"
      >
        gptranslate.berndt.no
      </a>
    </div>
  </div>
</section>

<!-- Custom CSS goes in /src/styles.css */ -->
