import process from "node:process";

// Tauri sets this when running `tauri dev`
const host = process.env.TAURI_DEV_HOST;

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",

  devtools: {
    enabled: false,
  },

  devServer: {
    host: host || "0.0.0.0",
    port: 3000,
  },

  components: [{ path: "~/components" }],

  app: {
    head: {
      meta: [
        {
          name: "viewport",
          content: "width=device-width, initial-scale=1, viewport-fit=cover",
        },
      ],
    },

    pageTransition: {
      name: "page",
      mode: "out-in",
    },

    layoutTransition: {
      name: "layout",
      mode: "out-in",
    },
  },

  css: [
    "highlight.js/styles/atom-one-dark.css",
    "@domternal/theme",
    "@/assets/css/main.css",
  ],

  ssr: false,

  modules: [
    // "@nuxt/a11y",
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

  hooks: {
    "imports:extend": (imports) => {
      for (let i = imports.length - 1; i >= 0; i--) {
        if (imports[i].name === "options") {
          imports.splice(i, 1);
        }
      }
    },
  },

  apollo: {
    clients: {
      default: {
        httpEndpoint:
          process.env.NUXT_PUBLIC_APOLLO_ENDPOINT ||
          "http://localhost:8000/orchard",
      },
    },
  },

  runtimeConfig: {
    public: {
      serverUrl:
        process.env.SERVER_URL ||
        process.env.NUXT_PUBLIC_SERVER_URL ||
        "http://localhost:8000",
    },
  },

  icon: {
    serverBundle: {
      collections: ["heroicons", "lucide", "ri"],
    },
  },

  vite: {
    clearScreen: false,

    envPrefix: ["VITE_", "TAURI_"],

    server: {
      strictPort: true,

      // Only pin HMR when the Tauri CLI hands us a LAN host (mobile dev);
      // `localhost` would resolve to the device itself. Otherwise let Nuxt
      // pick its own HMR port (24678+) — :3000 is already the HTTP server.
      ...(host && {
        hmr: {
          protocol: "ws",
          host,
          port: 24678,
        },
      }),

      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },

    optimizeDeps: {
      include: [
        "@nuxt/ui > prosemirror-state",
        "@nuxt/ui > prosemirror-transform",
        "@nuxt/ui > prosemirror-model",
        "@nuxt/ui > prosemirror-view",
        "@nuxt/ui > prosemirror-gapcursor",
        "rehackt",
      ],

      // PGlite ships its own WASM + FS assets.
      // Pre-bundling breaks `new PGlite("idb://lunar")` in dev.
      exclude: ["@electric-sql/pglite"],
    },

    worker: {
      format: "es",
    },
  },

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
});
