// layers/desktop/nuxt.config.ts
export default defineNuxtConfig({
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
    "./assets/css/main.css"
  ],
})