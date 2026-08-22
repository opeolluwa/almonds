<script setup lang="ts">
import EmptyState from "@shared/components/app/EmptyState.vue";
import type { NoteSort } from "@shared/components/notes";
import { useNoteStore } from "@shared/stores/notes";
import NotesCard from "@shared/components/notes/notes-card.vue";
const sortBy = ref<NoteSort>("date-newest");
const noteStore = useNoteStore();

const sortItems = computed(() => [
  [
    {
      label: "Name A–Z",
      icon:
        sortBy.value === "name-asc"
          ? "heroicons:check"
          : "heroicons:bars-arrow-up",
      onSelect: () => {
        sortBy.value = "name-asc";
      },
    },
    {
      label: "Name Z–A",
      icon:
        sortBy.value === "name-desc"
          ? "heroicons:check"
          : "heroicons:bars-arrow-down",
      onSelect: () => {
        sortBy.value = "name-desc";
      },
    },
  ],
  [
    {
      label: "Last modified (newest)",
      icon:
        sortBy.value === "date-newest" ? "heroicons:check" : "heroicons:clock",
      onSelect: () => {
        sortBy.value = "date-newest";
      },
    },
    {
      label: "Last modified (oldest)",
      icon:
        sortBy.value === "date-oldest" ? "heroicons:check" : "heroicons:clock",
      onSelect: () => {
        sortBy.value = "date-oldest";
      },
    },
  ],
]);

const filteredNotes = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  const list = q
    ? noteStore.notes.filter(
        (n) =>
          n.title.toLowerCase().includes(q) ||
          n.content.toLowerCase().includes(q),
      )
    : noteStore.notes;

  return [...list].sort((a, b) => {
    switch (sortBy.value) {
      case "name-asc":
        return a.title.localeCompare(b.title);
      case "name-desc":
        return b.title.localeCompare(a.title);
      case "date-newest":
        return (
          new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
        );
      case "date-oldest":
        return (
          new Date(a.updatedAt).getTime() - new Date(b.updatedAt).getTime()
        );
    }
  });
});

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}
</script>

<template>
  <!-- Loading -->
  <div v-if="noteStore.loading" class="flex flex-col gap-3">
    <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
  </div>

  <div
    v-if="!noteStore.loading && noteStore.notes.length > 0"
    class="flex justify-end mb-3"
  >
    <UDropdownMenu
      :items="sortItems"
      size="sm"
      :ui="{
        content:
          'min-w-48 rounded-xl shadow-lg border border-gray-100 dark:border-gray-800 py-1',
        item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
        separator: 'my-1 mx-2',
      }"
    >
      <button
        class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-xs font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
      >
        <UIcon name="heroicons:arrows-up-down" class="size-3.5" />
        Sort
      </button>
    </UDropdownMenu>
  </div>

  <!-- Empty state: no notes at all -->
  <div
    v-else-if="noteStore.notes.length === 0"
    class="flex flex-col items-center justify-center py-20 text-center"
  >
    <EmptyState
      title="No notes yet"
      description="Create your first note to get started."
      icon="ri:booklet-line"
      action-label="create note"
      @action="navigateTo('/notes/create-notes')"
    />
  </div>

  <!-- Notes list -->
  <div v-else class="flex flex-col gap-3">
    <NotesCard
      v-for="note in filteredNotes"
      :key="note.identifier"
      :identifier="note.identifier"
      :title="note.title"
      :content="note.content"
      :updated-at="note.updatedAt"
    />
  </div>
</template>
