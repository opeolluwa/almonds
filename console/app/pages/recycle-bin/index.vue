<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopRecycleBin from "~/desktop/pages/recycle-bin/index.vue";
import MobileRecycleBin from "~/mobile/pages/recycle-bin/index.vue";
</script>

<template>
  <DesktopRecycleBin v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileRecycleBin />
  </NuxtLayout>
</template>
