import { resolve } from "path"

const isMobile =
  process.env.TAURI_PLATFORM === "android" ||
  process.env.TAURI_PLATFORM === "ios"

export default defineNuxtConfig({
  srcDir: "app",
  extends: [
    "./app/layers/shared",
    isMobile ? "./app/layers/mobile" : "./app/layers/desktop",
  ],

  alias: {
    "@desktop": resolve(__dirname, "app/layers/desktop"),
    "@mobile": resolve(__dirname, "app/layers/mobile"),
    "@shared": resolve(__dirname, "app/layers/shared"),
    
  },

  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  ssr: false,

  modules: [
    "@nuxtjs/apollo",
    "@nuxt/eslint",
    "@nuxt/hints",
    "@nuxt/image",
    "@nuxt/ui",
    "@nuxtjs/device",
    "@nuxtjs/google-fonts",
    "@nuxtjs/i18n",
    "@pinia/nuxt",
    "pinia-plugin-persistedstate/nuxt",
    "@vueuse/nuxt",
  ],

  css: [
    "highlight.js/styles/atom-one-dark.css",
    "@domternal/theme",
    // vant/lib/index.css moved to app/layers/mobile/nuxt.config.ts
    // "./layers/shared/assets/css/main.css",
  ],

  colorMode: {
    preference: "system",
    fallback: "light",
    globalName: "__NUXT_COLOR_MODE__",
    componentName: "ColorScheme",
    classPrefix: "",
    classSuffix: "",
    storage: "localStorage",
    storageKey: "nuxt-color-mode",
  },

  devServer: { host: "0" },

  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: { strictPort: true },
  },

  ignore: ["**/src-tauri/**"],
})