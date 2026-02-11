<template>
<NuxtLayout name="default">
  <template #main_content>
    <h1 class="text-2xl font-bold mb-4">Welcome to Wild Almonds!</h1>
    <p class="text-gray-700">This is the main content area. You can customize it as needed.</p>
  </template>

  <template #side_content>
    <h2 class="text-xl font-semibold mb-3">Side Content</h2>
    <p class="text-gray-600">This is the side content area. You can add widgets, links, or any additional information here.</p>
  </template>
</NuxtLayout>

</template>

<script setup lang="ts">
definePageMeta({
  layout: false,    
});
</script>