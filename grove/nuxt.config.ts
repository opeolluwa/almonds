// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  css: ['~/assets/css/main.css'],

  colorMode: {
    preference: 'light',
    fallback: 'dark',
    globalName: '__NUXT_COLOR_MODE__',
    componentName: 'ColorScheme',
    classPrefix: '',
    classSuffix: '',
    storage: 'localStorage',
    storageKey: 'nuxt-color-mode',
  },

  app: {
    head: {
      link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        {
          rel: 'stylesheet',
          href: 'https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800;900&display=swap',
        },
      ],
    },
  },
  site: {
    url: "https://opeolluwa.github.io/almonds",
    name: "Wild almond",
  },
  modules: [
    '@nuxt/content',
    '@nuxt/eslint',
    '@nuxt/hints',
    '@nuxt/image',
    '@nuxt/ui',
    '@nuxtjs/device',
    '@nuxtjs/eslint-module',
    '@nuxtjs/seo',
    '@pinia/nuxt',
  ],
})
