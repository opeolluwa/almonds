<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopConfirmOtp from "~/desktop/pages/auth/confirm-otp.vue";
import MobileConfirmOtp from "~/mobile/pages/auth/confirm-otp.vue";
</script>

<template>
  <DesktopConfirmOtp v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileConfirmOtp />
  </NuxtLayout>
</template>
