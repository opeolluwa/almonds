<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopCreateNotes from "~/desktop/pages/notes/create-notes.vue";
import MobileCreateNotes from "~/mobile/pages/notes/create-notes.vue";
</script>

<template>
  <DesktopCreateNotes v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileCreateNotes />
  </NuxtLayout>
</template>
