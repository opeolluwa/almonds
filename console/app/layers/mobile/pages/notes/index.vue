<script setup lang="ts">
import EmptyState from "@shared/components/app/EmptyState.vue";
import {
  NOTE_SORT_OPTIONS,
  sortNotes,
  type NoteSort,
} from "@shared/utils/sorting";
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

const filteredNotes = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  const list = q
    ? noteStore.notes.filter(
        (n) =>
          n.title.toLowerCase().includes(q) ||
          n.content.toLowerCase().includes(q),
      )
    : noteStore.notes;

  return sortNotes(list, sortBy.value);
});
</script>

<template>
  <div>
    <!-- Create note FAB -->
    <div v-if="!noteStore.loading && filteredNotes.length > 0">
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
        <div class="mb-3">
          <AppInput
            v-model="searchQuery"
            name="search"
            icon="heroicons:magnifying-glass"
            placeholder="Search notes..."
            size="sm"
          />
          <AppSortMenu
            v-if="filteredNotes.length > 0"
            v-model="sortBy"
            :options="NOTE_SORT_OPTIONS"
            class="mt-2"
          />
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
