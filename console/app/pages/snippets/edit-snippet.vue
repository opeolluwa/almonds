<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopEditSnippet from "~/desktop/pages/snippets/edit-snippet.vue";
import MobileEditSnippet from "~/mobile/pages/snippets/edit-snippet.vue";
</script>

<template>
  <DesktopEditSnippet v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileEditSnippet />
  </NuxtLayout>
</template>
