<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopLogin from "~/desktop/pages/auth/login.vue";
import MobileLogin from "~/mobile/pages/auth/login.vue";
</script>

<template>
  <DesktopLogin v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileLogin />
  </NuxtLayout>
</template>
