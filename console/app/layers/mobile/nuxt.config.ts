// app/layers/mobile/nuxt.config.ts
export default defineNuxtConfig({
  ssr: false,
  app: {
    head: {
      meta: [
        {
          charset: "utf-8",
        },
        {
          name: "viewport",
          content:
            "width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=no, viewport-fit=cover",
        },
        {
          name: "apple-mobile-web-app-capable",
          content: "yes",
        },
      ],
    },
  },
});
