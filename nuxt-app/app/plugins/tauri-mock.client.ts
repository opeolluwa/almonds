// Stubs Tauri global objects when running outside the Tauri webview
// (e.g. `nuxt dev` in a plain browser). Real Tauri injects these before any
// JS runs, so their absence is a reliable "not in Tauri" guard.
export default defineNuxtPlugin(async () => {
  if (typeof window === "undefined" || window.__TAURI_INTERNALS__) return;

  const { mockWindows, mockIPC, mockConvertFileSrc } =
    await import("@tauri-apps/api/mocks");

  mockWindows("main");
  mockConvertFileSrc("macos");

  mockIPC((cmd, _payload) => {
    console.warn(`[tauri-mock] invoke("${cmd}") — no backend`);
    // Commands that return lists must return [] so store getters don't crash
    // calling .filter()/.find() on null.
    const listCommands = [
      "list_workspaces",
      "get_all_notes",
      "get_recently_added_notes",
      "get_all_todos",
      "get_all_bookmarks",
      "get_all_reminders",
      "get_all_snippets",
      "list_moodboard_images",
      "get_notifications_by_type",
      "get_unsynced_workspaces",
      "get_unsynced_notes",
      "get_unsynced_todos",
      "get_unsynced_bookmarks",
      "get_unsynced_reminders",
      "get_unsynced_snippets",
    ];
    if (listCommands.includes(cmd)) return [];
    return null;
  });

  // plugin-os reads these synchronously from the global, not via IPC
  window.__TAURI_OS_PLUGIN_INTERNALS__ = {
    platform: "macos",
    os_type: "macos",
    family: "unix",
    version: "0.0.0",
    arch: "aarch64",
    eol: "\n",
    exe_extension: "",
  };
});
