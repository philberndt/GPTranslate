// Tauri API wrappers with safe browser fallbacks.
// In the packaged Tauri app these will forward to the real APIs.
// In the browser dev server (where window.__TAURI__ is undefined) they become safe no-ops
// so the UI can run without a native backend and without noisy console errors.

import * as tauriCore from "@tauri-apps/api/core";
import * as tauriEvent from "@tauri-apps/api/event";

const hasTauri =
  typeof window !== "undefined" &&
  // Tauri v1 global
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  ((window as any).__TAURI__ !== undefined ||
    // Tauri v2 internals
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).__TAURI_INTERNALS__ !== undefined ||
    // Fallback: user agent hint when running inside the Tauri webview
    (typeof navigator !== "undefined" &&
      navigator.userAgent?.includes("Tauri")));

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const invoke = (...args: any[]) => {
  if (!hasTauri) {
    // Browser dev fallback: resolve with null and log once to help debugging
    console.warn(
      "Tauri invoke called in browser dev - returning null (no-op)",
      args[0],
    );
    return Promise.resolve(null);
  }
  // Forward to real implementation when available
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (tauriCore.invoke as any)(...args);
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const listen = (event: string, cb: (...args: any[]) => void) => {
  if (!hasTauri) {
    console.warn("Tauri listen called in browser dev - no-op", event);
    return Promise.resolve(() => {});
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (tauriEvent.listen as any)(event, cb);
};
