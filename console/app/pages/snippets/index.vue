<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopSnippets from "~/desktop/pages/snippets/index.vue";
import MobileSnippets from "~/mobile/pages/snippets/index.vue";
</script>

<template>
  <DesktopSnippets v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileSnippets />
  </NuxtLayout>
</template>
