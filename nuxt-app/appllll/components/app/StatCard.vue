<script setup lang="ts">
defineProps<{
  label: string;
  value: string | number;
  icon: string;
}>();
</script>

<template>
  <div
    class="border border-gray-100 dark:border-white/5 rounded-2xl p-5 flex flex-col gap-4"
  >
    <div class="flex items-center justify-between">
      <span class="text-sm text-gray-500 dark:text-white/40 font-medium">{{
        label
      }}</span>
      <div class="w-8 h-8 rounded-xl flex items-center justify-center">
        <UIcon :name="icon" class="size-4 text-brand" />
      </div>
    </div>
    <div>
      <p class="text-3xl font-bold text-gray-900 dark:text-white">
        {{ value }}
      </p>
    </div>
  </div>
</template>
