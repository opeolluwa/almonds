<script setup lang="ts">
import EmptyState from "@shared/components/app/EmptyState.vue";
import type { NoteSort } from "@shared/components/notes";
import { useNoteStore } from "@shared/stores/notes";
import NotesCard from "@shared/components/notes/notes-card.vue";
const noteStore = useNoteStore();
const { searchQuery, clearSearch } = useAppSearch();
const sortBy = ref<NoteSort>("date-newest");

definePageMeta({
  layout: "notes",
});

onMounted(async () => {
  await noteStore.fetchNotes();
});

onUnmounted(() => clearSearch());

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
</script>

<template>
  <div>
    <!-- Create note FAB -->
    <div v-if="!noteStore.loading && noteStore.notes.length > 0">
      <AppFab
        aria-label="Add note"
        @click="navigateTo('/notes/create-notes')"
      />
    </div>

    <!-- Loading -->
    <div v-if="noteStore.loading" class="flex flex-col gap-3">
      <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
    </div>

    <template v-else>
      <!-- Empty state: no notes at all -->
      <div
        v-if="noteStore.notes.length === 0"
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

      <template v-else>
        <!-- Search + sort controls -->
        <div class="flex items-center gap-2 mb-3">
          <AppInput
            v-model="searchQuery"
            name="search"
            icon="heroicons:magnifying-glass"
            placeholder="Search notes..."
            size="sm"
          />
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
              class="flex shrink-0 items-center gap-1.5 px-2.5 py-1.5 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-xs font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
            >
              <UIcon name="heroicons:arrows-up-down" class="size-3.5" />
              Sort
            </button>
          </UDropdownMenu>
        </div>

        <!-- Empty state: search has no results -->
        <EmptyState
          v-if="filteredNotes.length === 0"
          title="No results found"
          description="Try a different search term."
          icon="heroicons:magnifying-glass"
          action-label="clear search"
          @action="searchQuery = ''"
        />

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
    </template>
  </div>
</template>
