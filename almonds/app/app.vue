<script setup lang="ts">
const { init } = useAccentColor();
onMounted(init);
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
