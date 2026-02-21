<script setup lang="ts">
const { init } = useAccentColor();
const { init: initFontSize } = useFontSize();
const { init: initDarkTheme } = useDarkTheme();
const { setupRequired, checkSetup } = useUserSetup();

onMounted(async () => {
  init();
  initFontSize();
  initDarkTheme();
  await checkSetup();
});
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <AppNotification />
    <UserSetupModal v-if="setupRequired" />
  </UApp>
</template>
