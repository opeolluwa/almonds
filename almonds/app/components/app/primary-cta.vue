<script setup lang="ts">
defineProps<{
  label: string;
  icon?: string;
  to: string;
}>();
</script>

<template>
  <!-- Desktop: labelled button -->
  <div class="hidden md:flex items-center justify-end">
    <button
      class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
      @click="navigateTo(to)"
    >
      <UIcon v-if="icon" :name="icon" class="size-4" />
      {{ label }}
    </button>
  </div>

  <!-- Mobile: icon-only round FAB -->
  <button
    class="md:hidden flex items-center justify-center w-14 h-14 bg-accent-500 text-white rounded-full shadow-xl active:scale-95 transition-transform"
    :aria-label="label"
    @click="navigateTo(to)"
  >
    <UIcon v-if="icon" :name="icon" class="size-6" />
  </button>
</template>
