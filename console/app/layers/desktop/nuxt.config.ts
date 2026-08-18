import { resolve } from "@tauri-apps/api/path";

// layers/desktop/nuxt.config.ts
export default defineNuxtConfig({

  alias: {
    "@shared": resolve(__dirname, "../shared"),
    
  },
})