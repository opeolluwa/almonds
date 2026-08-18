declare interface Window {
  // Injected by Tauri core
  __TAURI_INTERNALS__?: Record<string, unknown>;
  // Injected by @tauri-apps/plugin-os
  __TAURI_OS_PLUGIN_INTERNALS__?: {
    eol: string;
    os_type: "linux" | "windows" | "macos" | "ios" | "android";
    platform:
      | "linux"
      | "macos"
      | "ios"
      | "freebsd"
      | "dragonfly"
      | "netbsd"
      | "openbsd"
      | "solaris"
      | "android"
      | "windows";
    family: "unix" | "windows";
    version: string;
    arch: string;
    exe_extension: string;
  };
  // Injected by @tauri-apps/api event module
  __TAURI_EVENT_PLUGIN_INTERNALS__?: {
    unregisterListener: (event: string, eventId: number) => void;
  };
}
