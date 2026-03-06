<script setup lang="ts">
import type { Bookmark } from "~/stores/bookmarks";

defineProps<{ workspace: Workspace }>();

const emit = defineEmits<{
  delete: [identifier: string];
  update: [identifier: string];
}>();

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
</script>

<template>
  <div
    class="group bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow flex items-center gap-4"
  >
    <UIcon name="heroicons:briefcase" class="size-5 text-accent-500 shrink-0" />
    <div class="flex-1 min-w-0">
      <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">
        {{ workspace.name }}
      </h3>
      <div class="text-xs text-gray-400 truncate block">
        {{ workspace.description }}
      </div>
    </div>
    <span
      class="px-2 py-1 rounded-full bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300 text-xs font-medium capitalize shrink-0"
    >
      <!-- {{ workspace.tag }} -->
    </span>
    <p class="text-xs text-gray-400 shrink-0 hidden sm:block">
      {{ formatDate(workspace.createdAt) }}
    </p>
    <div
      class="opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity flex items-center gap-1"
    >
      <button
        class="text-gray-400 hover:text-accent-500 transition-colors"
        title="Preview"
        @click="emit('update', workspace.identifier)"
      >
        <UIcon name="heroicons:eye" class="size-4" />
      </button>
      <button
        class="text-gray-400 hover:text-red-500 transition-colors"
        title="Delete"
        @click="emit('delete', workspace.identifier)"
      >
        <UIcon name="heroicons:trash" class="size-4" />
      </button>
    </div>
  </div>
</template>
