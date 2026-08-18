<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopCalendar from "~/desktop/pages/calendar/index.vue";
import MobileCalendar from "~/mobile/pages/calendar/index.vue";
</script>

<template>
  <DesktopCalendar v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileCalendar />
  </NuxtLayout>
</template>
