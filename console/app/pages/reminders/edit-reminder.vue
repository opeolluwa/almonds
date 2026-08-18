<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopEditReminder from "~/desktop/pages/reminders/edit-reminder.vue";
import MobileEditReminder from "~/mobile/pages/reminders/edit-reminder.vue";
</script>

<template>
  <DesktopEditReminder v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileEditReminder />
  </NuxtLayout>
</template>
