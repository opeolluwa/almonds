<script setup lang="ts">
withDefaults(
  defineProps<{
    icon?: string;
  }>(),
  { icon: "heroicons:plus" },
);
</script>

<template>
  <button
    class="md:hidden absolute bottom-12 right-8 z-50 flex items-center justify-center w-14 h-14 bg-primary-500 text-white rounded-full shadow-xl active:scale-95 transition-transform"
    aria-label="Add"
  >
    <UIcon :name="icon" class="size-6" />
  </button>
</template>
