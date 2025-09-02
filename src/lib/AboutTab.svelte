<script lang="ts">
  import AppIcon from "./AppIcon.svelte"
  interface Props {
    version: string
  }
  let { version }: Props = $props()
</script>

<!-- About Section -->
<div class="">
  <div class="">
    <div class="">
      <AppIcon size={56} className="mb-3" />
      <h2 class="">GPTranslate</h2>
      <span class="">v{version}</span>
      <h4 class=""><i class=""></i>About</h4>

      <div class="">
        <div class="">
          <div class="">
            <strong class="">Developer:</strong>
            <span>Phil Berndt</span>
          </div>
        </div>
        <div class="">
          <div class="">
            <strong class="">Email:</strong>
            <span>phil@berndt.no</span>
          </div>
        </div>
        <div class="">
          <div class="">
            <strong class="">Website:</strong>
            <a
              href="https://gptranslate.berndt.no"
              target="_blank"
              rel="noopener noreferrer"
              class=""
            >
              gptranslate.berndt.no
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  /* CSS moved to /src/app.css */
</style>
