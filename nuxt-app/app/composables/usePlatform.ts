export function usePlatform() {
  const platformName = useState<string>("platform", () => "web");

  if (typeof window === "undefined") return platformName;

  if (window.__TAURI_INTERNALS__) {
    import("@tauri-apps/plugin-os").then(({ platform }) => {
      platformName.value = platform();
    });
  }

  return platformName;
}
