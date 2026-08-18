<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopScratchPad from "~/desktop/pages/scratch-pad/index.vue";
import MobileScratchPad from "~/mobile/pages/scratch-pad/index.vue";
</script>

<template>
  <DesktopScratchPad v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileScratchPad />
  </NuxtLayout>
</template>
