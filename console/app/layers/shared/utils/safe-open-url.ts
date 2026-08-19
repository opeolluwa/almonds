export async function safeOpenUrl(url: string): Promise<void> {
  if (typeof window !== "undefined" && window.__TAURI_INTERNALS__) {
    const { openUrl } = await import("@tauri-apps/plugin-opener");
    return openUrl(url);
  }
  window.open(url, "_blank");
}
