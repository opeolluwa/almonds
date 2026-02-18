<script setup lang="ts">
import type { BookmarkTag } from "~/stores/bookmarks";

defineProps<{
  tags: { label: string; value: BookmarkTag | "all" }[];
  tagIcons: Record<BookmarkTag, string>;
  modelValue: BookmarkTag | "all";
  totalCount: number;
  tagCounts: Record<BookmarkTag, number>;
}>();

const emit = defineEmits<{ "update:modelValue": [value: BookmarkTag | "all"] }>();
</script>

<template>
  <div>
    <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
      Collections
    </h2>
    <div class="flex flex-col gap-2">
      <button
        class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm font-medium w-full text-left transition-colors"
        :class="
          modelValue === 'all'
            ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300'
            : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
        "
        @click="emit('update:modelValue', 'all')"
      >
        <UIcon name="heroicons:folder-solid" class="size-4" />
        <span>All Bookmarks</span>
        <span class="ml-auto text-xs">{{ totalCount }}</span>
      </button>

      <button
        v-for="tag in tags.slice(1)"
        :key="tag.value"
        class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm w-full text-left transition-colors"
        :class="
          modelValue === tag.value
            ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
            : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
        "
        @click="emit('update:modelValue', tag.value as BookmarkTag)"
      >
        <UIcon :name="tagIcons[tag.value as BookmarkTag]" class="size-4" />
        <span>{{ tag.label }}</span>
        <span class="ml-auto text-xs text-gray-400">
          {{ tagCounts[tag.value as BookmarkTag] }}
        </span>
      </button>
    </div>
  </div>
</template>
