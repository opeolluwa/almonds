<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopReminders from "~/desktop/pages/reminders/index.vue";
import MobileReminders from "~/mobile/pages/reminders/index.vue";
</script>

<template>
  <DesktopReminders v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileReminders />
  </NuxtLayout>
</template>
