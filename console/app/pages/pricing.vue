<script setup lang="ts">
const { isMobile } = useDevice();
import DesktopPricing from "~/desktop/pages/pricing.vue";
import MobilePricing from "~/mobile/pages/pricing.vue";
</script>

<template>
  <DesktopPricing v-if="!isMobile" />
  <NuxtLayout v-else name="mobile-default">
    <MobilePricing />
  </NuxtLayout>
</template>
