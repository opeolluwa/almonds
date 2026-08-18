<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopResetPassword from "~/desktop/pages/auth/reset-password.vue";
import MobileResetPassword from "~/mobile/pages/auth/reset-password.vue";
</script>

<template>
  <DesktopResetPassword v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileResetPassword />
  </NuxtLayout>
</template>
