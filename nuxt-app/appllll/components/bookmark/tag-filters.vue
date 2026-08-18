<script setup lang="ts">
import type { BookmarkTag } from "~/stores/bookmarks";

defineProps<{
  tags: { label: string; value: BookmarkTag | "all" }[];
  modelValue: BookmarkTag | "all";
}>();

const emit = defineEmits<{
  "update:modelValue": [value: BookmarkTag | "all"];
}>();
</script>

<template>
  <div class="flex gap-2 mb-4 flex-wrap">
    <button
      v-for="tag in tags"
      :key="tag.value"
      class="px-3 py-1 rounded-full text-xs font-medium transition-colors"
      :class="
        modelValue === tag.value
          ? 'bg-accent-500 text-white'
          : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
      "
      @click="emit('update:modelValue', tag.value)"
    >
      {{ tag.label }}
    </button>
  </div>
</template>
