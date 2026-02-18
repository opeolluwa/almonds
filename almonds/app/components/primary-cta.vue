<script setup lang="ts">
defineProps<{
  label: string;
  icon?: string;
  to: string;
}>();
</script>

<template>
  <div class="flex items-center justify-end">
    <button
      class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
      @click="navigateTo(to)"
    >
      <UIcon v-if="icon" :name="icon" class="size-4" />
      {{ label }}
    </button>
  </div>
</template>
