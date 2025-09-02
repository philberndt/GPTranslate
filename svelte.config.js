// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static"
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte"

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  compilerOptions: {
    // Enable Svelte DevTools in development
    dev: process.env.NODE_ENV !== "production",
  },
  kit: {
    adapter: adapter({
      fallback: "index.html",
      precompress: false,
    }),
    prerender: {
      handleHttpError: ({ path, referrer, message }) => {
        // ignore deliberate link to shiny 404 page
        if (
          path === "/not-found" &&
          referrer === "/blog/how-is-sveltekit-so-fast"
        ) {
          return
        }

        // otherwise fail the build
        throw new Error(message)
      },
    },
  },
}

export default config
