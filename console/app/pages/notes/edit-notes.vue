<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopEditNotes from "~/desktop/pages/notes/edit-notes.vue";
import MobileEditNotes from "~/mobile/pages/notes/edit-notes.vue";
</script>

<template>
  <DesktopEditNotes v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileEditNotes />
  </NuxtLayout>
</template>
