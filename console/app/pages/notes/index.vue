<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopNotes from "~/desktop/pages/notes/index.vue";
import MobileNotes from "~/mobile/pages/notes/index.vue";
</script>

<template>
  <DesktopNotes v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileNotes />
  </NuxtLayout>
</template>
