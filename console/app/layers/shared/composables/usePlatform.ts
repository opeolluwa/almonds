export function usePlatform() {
  const platformName = useState<string>("platform", () => "web");

  if (typeof window !== "undefined" && window.__TAURI_INTERNALS__) {
    import("@tauri-apps/plugin-os").then(({ platform }) => {
      platformName.value = platform();
    });
  }

  return {
    platformName,
    isMobile: computed(
      () => platformName.value === "android" || platformName.value === "ios",
    ),
    isWeb: computed(() => platformName.value === "web"),
    isDesktop: computed(
      () =>
        platformName.value === "macos" ||
        platformName.value === "windows" ||
        platformName.value === "linux",
    ),
  };
}
