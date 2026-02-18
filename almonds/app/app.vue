<script setup lang="ts">
const { init } = useAccentColor();
const { init: initFontSize } = useFontSize();
onMounted(() => {
  init();
  initFontSize();
});
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
