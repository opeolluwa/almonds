<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopCreateSnippets from "~/desktop/pages/snippets/create-snippets.vue";
import MobileCreateSnippets from "~/mobile/pages/snippets/create-snippets.vue";
</script>

<template>
  <DesktopCreateSnippets v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileCreateSnippets />
  </NuxtLayout>
</template>
