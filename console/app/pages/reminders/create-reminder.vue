<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopCreateReminder from "~/desktop/pages/reminders/create-reminder.vue";
import MobileCreateReminder from "~/mobile/pages/reminders/create-reminder.vue";
</script>

<template>
  <DesktopCreateReminder v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileCreateReminder />
  </NuxtLayout>
</template>
