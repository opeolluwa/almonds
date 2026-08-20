<script setup lang="ts">
definePageMeta({ layout: "default" });

onMounted(() => {
  navigateTo("/auth/login");
});
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-screen p-6">
    <h1 class="text-lg font-semibold">Home</h1>
  </div>
</template>
