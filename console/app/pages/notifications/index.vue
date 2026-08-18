<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopNotifications from "~/desktop/pages/notifications/index.vue";
import MobileNotifications from "~/mobile/pages/notifications/index.vue";
</script>

<template>
  <DesktopNotifications v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileNotifications />
  </NuxtLayout>
</template>
