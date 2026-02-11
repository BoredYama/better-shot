/// <reference types="vite/client" />

declare const __APP_VERSION__: string;

interface Window {
  __TAURI_INTERNALS__?: Record<string, unknown>;
}
