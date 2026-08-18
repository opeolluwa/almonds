<script setup lang="ts">
const { isMobile } = useDevice();
</script>

<template>
  <PlatformMobile v-if="isMobile" />
  <PlatformDesktop v-else />
</template>
