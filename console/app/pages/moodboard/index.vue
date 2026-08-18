<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopMoodboard from "~/desktop/pages/moodboard/index.vue";
import MobileMoodboard from "~/mobile/pages/moodboard/index.vue";
</script>

<template>
  <DesktopMoodboard v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileMoodboard />
  </NuxtLayout>
</template>
