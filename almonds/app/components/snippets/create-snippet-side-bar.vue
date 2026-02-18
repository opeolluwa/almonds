<script setup lang="ts">
const snippetStore = useSnippetStore();
const recentSnippets = computed(() => snippetStore.recent);

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
  });
}
</script>

<template>
  <div v-if="snippetStore.recentLoading" class="grid gap-2">
    <USkeleton class="h-4 w-full" />
    <USkeleton class="h-4 w-3/4" />
  </div>

  <UEmpty
    v-else-if="recentSnippets.length === 0"
    size="sm"
    icon="heroicons:code-bracket"
    title="No recent snippets"
    description="Recently added snippets will appear here."
    :ui="{ avatar: 'bg-green-300/20' }"
  />

  <div v-else class="flex flex-col gap-2">
    <div
      v-for="snippet in recentSnippets"
      :key="snippet.identifier"
      class="py-2 px-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
    >
      <p class="font-medium text-gray-700 dark:text-gray-300 truncate text-xs">
        {{ snippet.title ?? "Untitled" }}
      </p>
      <p class="text-xs text-gray-400 mt-0.5">{{ formatDate(snippet.createdAt) }}</p>
    </div>
  </div>
</template>
