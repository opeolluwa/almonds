<script setup lang="ts">
import {
  useCreateSnippet,
  type CreateSnippetPayload,
} from "~/composables/snippets/useCreateSnippet";
import { useRecentlyAddedSnippet } from "~/composables/snippets/useRecentlyAddedSnippet";

const { loading, fetchRecentSnippets, error } = useRecentlyAddedSnippet();
const snippetStore = useSnippetStore();
const recentSnippets = computed(() => snippetStore.recent);
onMounted(async () => {
  await fetchRecentSnippets();
  const { loading, createSnippet } = useCreateSnippet();
  const payload: CreateSnippetPayload = {
    title: "test",
    language: "Rust",
    code: `const { loading, fetchRecentSnippets, error } = useRecentlyAddedSnippet();
const snippetStore = useSnippetStore();
const recentSnippets = computed(() => snippetStore.recent);`,
    description: "lot",
  };

  await createSnippet(payload);
});
</script>
<template>
  <div v-if="loading">
    <div class="grid gap-2">
      <USkeleton class="h-4 w-62.5" />
      <USkeleton class="h-4 w-50" />
    </div>
  </div>
  <UEmpty
    v-if="recentSnippets.length == 0"
    size="sm"
    icon="heroicons:code-bracket"
    title="No recent snippet"
    description="You're all caught up. Recently added snippets will appear here."
    :ui="{ avatar: 'bg-green-300/20' }"
  />
</template>
