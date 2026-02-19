<script setup lang="ts">
const { init } = useAccentColor();
const { init: initFontSize } = useFontSize();
const { init: initDarkTheme } = useDarkTheme();
onMounted(() => {
  init();
  initFontSize();
  initDarkTheme();
});
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <AppNotification />
  </UApp>
</template>
