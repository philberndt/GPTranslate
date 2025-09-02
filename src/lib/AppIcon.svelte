<script>
  // This component displays the application icon
  export let size = 24 // Default size in pixels
  // Additional CSS classes can be added via the 'className' prop
  export let className = ""
</script>

<img
  src="/logo_app.svg"
  alt="GPTranslate Logo"
  class={className}
  style="width: {size}px; height: {size}px;"
/>

<!-- Custom CSS goes in /src/styles.css -->
