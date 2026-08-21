import { useMediaQuery } from "@vueuse/core";

export const IS_WEB =
  typeof window !== "undefined" && !window.__TAURI_INTERNALS__;
export const IS_TAURI = !IS_WEB;
export const LUNAR_API = IS_WEB ? window?.lunar : undefined;
export const LUNAR_EXISTS = IS_WEB && window.lunar !== undefined;

export const isLargeScreen = useMediaQuery("(min-width: 1024px)");
export const isPreferredDark = useMediaQuery("(prefers-color-scheme: dark)");
export const isMobile = useMediaQuery("(max-width: 1023px)", {
  ssrWidth: 768,
});
