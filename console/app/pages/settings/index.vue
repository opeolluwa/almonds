<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopSettings from "~/desktop/pages/settings/index.vue";
import MobileSettings from "~/mobile/pages/settings/index.vue";
</script>

<template>
  <DesktopSettings v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileSettings />
  </NuxtLayout>
</template>
