<script setup lang="ts">
defineProps<{
  title: string;
  description: string;
}>();
</script>

<template>
  <div class="flex flex-col gap-1">
    <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
      {{ title }}
    </h2>
    <p class="text-sm text-gray-500 dark:text-gray-400">{{ description }}</p>
  </div>
</template>
