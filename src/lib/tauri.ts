// Tauri v2 direct exports for runtime in the packaged app (no browser fallback)
// The app ships as a Tauri executable, so we import APIs directly.

export { invoke } from "@tauri-apps/api/core";
export { listen } from "@tauri-apps/api/event";
