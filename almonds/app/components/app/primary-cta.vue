<script setup lang="ts">
defineProps<{
  label: string;
  icon?: string;
  to: string;
}>();
</script>

<template>
  <!-- Desktop: labelled button -->
  <NuxtLink
    :to="to"
    class="hidden md:flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
  >
    <UIcon v-if="icon" :name="icon" class="size-4" />
    {{ label }}
  </NuxtLink>

  <!-- Mobile: icon-only round FAB -->
  <NuxtLink
    :to="to"
    class="md:hidden flex items-center justify-center w-14 h-14 bg-accent-500 text-white rounded-full shadow-xl active:scale-95 transition-transform"
    :aria-label="label"
  >
    <UIcon v-if="icon" :name="icon" class="size-6" />
  </NuxtLink>
</template>
