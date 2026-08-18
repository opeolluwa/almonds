<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopAcceptInvitation from "~/desktop/pages/auth/accept-invitation.vue";
import MobileAcceptInvitation from "~/mobile/pages/auth/accept-invitation.vue";
</script>

<template>
  <DesktopAcceptInvitation v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobileAcceptInvitation />
  </NuxtLayout>
</template>
