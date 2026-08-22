<script setup lang="ts">
defineProps<{
  label: string;
  icon?: string;
  to: string;
}>();
</script>

<template>
  <!-- Desktop: labeled button -->
  <NuxtLink
    :to="to"
    class="hidden md:flex items-center gap-2 py-2 px-4 bg-primary-500 text-white rounded-lg text-sm font-medium hover:bg-primary-600 transition-colors"
  >
    <UIcon v-if="icon" :name="icon" class="size-4" />
    {{ label }}
  </NuxtLink>

  <!-- Mobile: icon-only round FAB -->
  <AppFab :icon="icon" :aria-label="label" @click="navigateTo(to)" />
</template>
