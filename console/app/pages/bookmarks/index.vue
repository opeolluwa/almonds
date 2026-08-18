<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopBookmarks from "~/desktop/pages/bookmarks/index.vue";
import MobileBookmarks from "~/mobile/pages/bookmarks/index.vue";
</script>

<template>
  <DesktopBookmarks v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileBookmarks />
  </NuxtLayout>
</template>
