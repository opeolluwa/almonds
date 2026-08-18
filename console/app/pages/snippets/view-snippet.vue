<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopViewSnippet from "~/desktop/pages/snippets/view-snippet.vue";
import MobileViewSnippet from "~/mobile/pages/snippets/view-snippet.vue";
</script>

<template>
  <DesktopViewSnippet v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileViewSnippet />
  </NuxtLayout>
</template>
