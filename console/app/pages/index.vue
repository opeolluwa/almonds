<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopIndex from "~/desktop/pages/index.vue";
import MobileIndex from "~/mobile/pages/index.vue";
</script>

<template>
  <DesktopIndex v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileIndex />
  </NuxtLayout>
</template>
