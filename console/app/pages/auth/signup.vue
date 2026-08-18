<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopSignup from "~/desktop/pages/auth/signup.vue";
import MobileSignup from "~/mobile/pages/auth/signup.vue";
</script>

<template>
  <DesktopSignup v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileSignup />
  </NuxtLayout>
</template>
