import { resolve } from "path";

// const isMobile = process.env.NUXT_APP_TARGET === "mobile";
const isMobile = true;
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

  app: {
    head: {
      meta: [
        {
          name: "viewport",
          content:
            "width=device-width, initial-scale=1, viewport-fit=cover, interactive-widget=resizes-content",
        },
      ],
    },
  },

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
    "./assets/css/main.css",
    "highlight.js/styles/atom-one-dark.css",
    "@domternal/theme",
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
  icon: {
    serverBundle: {
      collections: ["heroicons", "lucide", "ri"],
    },
  },

  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: { strictPort: true },
    optimizeDeps: {
      include: [
        "@nuxt/ui > prosemirror-state",
        "@nuxt/ui > prosemirror-transform",
        "@nuxt/ui > prosemirror-model",
        "@nuxt/ui > prosemirror-view",
        "@nuxt/ui > prosemirror-gapcursor",
        "rehackt",
      ],
      // PGlite ships its own wasm + FS assets; pre-bundling it breaks
      // `new PGlite("idb://lunar")` in dev ("Invalid FS bundle size").
      exclude: ["@electric-sql/pglite"],
    },
  },

  ignore: ["**/src-tauri/**"],
});
