<script>
    // This component displays the application icon
    export let size = 24; // Default size in pixels
    export let className = ""; // Additional CSS classes
</script>

<img
    src="/logo_app.svg"
    alt="GPTranslate Logo"
    class="app-icon {className}"
    style="width: {size}px; height: {size}px;"
/>

<style>
    .app-icon {
        margin-right: 8px;
        display: inline-block;
        vertical-align: middle;
    }

    /* Remove margin when used with additional classes (like settings-logo) */
    .app-icon.settings-logo {
        margin-right: 0;
    }
</style>
