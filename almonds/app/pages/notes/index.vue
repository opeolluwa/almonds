<script setup lang="ts">
import NotesCard from "~/components/notes/notes-card.vue";
import { useNoteStore } from "~/stores/notes";

definePageMeta({ layout: false });

const noteStore = useNoteStore();
const router = useRouter();
const { searchQuery, setSearch, clearSearch } = useAppSearch();

const filteredNotes = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return noteStore.notes;
  return noteStore.notes.filter(
    (n) =>
      n.title.toLowerCase().includes(q) || n.content.toLowerCase().includes(q),
  );
});

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

onMounted(async () => {
  setSearch({ placeholder: "Search notes..." });
  await Promise.all([noteStore.fetchNotes(), noteStore.fetchRecentNotes()]);
});

onUnmounted(() => clearSearch());
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <PrimaryCta
        v-if="noteStore.notes.length !== 0"
        label="New Note"
        icon="heroicons:plus"
        to="/notes/create-notes"
      />
    </template>

    <template #main_content>
      <!-- Loading -->
      <div v-if="noteStore.loading" class="flex flex-col gap-3">
        <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
      </div>

      <!-- Empty state: no notes at all -->
      <div
        v-else-if="noteStore.notes.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div
          class="mb-4 p-2 flex justify-center items-center rounded-full bg-gray-100 dark:bg-gray-800"
        >
          <UIcon
            name="heroicons:document-text"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No notes yet
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Create your first note to get started.
        </p>
        <NuxtLink
          to="/notes/create-notes"
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
        >
          Create note
        </NuxtLink>
      </div>

      <!-- Empty state: search has no results -->
      <div
        v-else-if="filteredNotes.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-4 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon
            name="heroicons:magnifying-glass"
            class="w-8 h-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No results found
        </h3>
        <p class="text-xs text-gray-400 dark:text-gray-500 mb-4">
          Try a different search term.
        </p>
        <button
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
          @click="searchQuery = ''"
        >
          Clear search
        </button>
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

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Recently modified
      </h2>

      <div v-if="noteStore.recentLoading" class="flex flex-col gap-1">
        <USkeleton v-for="i in 3" :key="i" class="h-10 rounded-lg" />
      </div>

      <div
        v-else-if="noteStore.recent.length === 0"
        class="flex flex-col items-center py-6 text-center"
      >
        <UIcon
          name="heroicons:clock"
          class="w-5 h-5 text-gray-300 dark:text-gray-600 mb-2"
        />
        <p class="text-xs text-gray-400 dark:text-gray-500">No recent notes</p>
      </div>

      <div v-else class="flex flex-col gap-1">
        <div
          v-for="note in noteStore.recent"
          :key="note.identifier"
          class="flex items-center gap-3 py-2.5 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
          @click="router.push(`/notes/edit-notes?id=${note.identifier}`)"
        >
          <UIcon
            name="heroicons:document-text"
            class="size-4 text-gray-400 shrink-0"
          />
          <div class="flex-1 min-w-0">
            <p
              class="font-medium text-gray-700 dark:text-gray-300 truncate text-xs"
            >
              {{ note.title || "Untitled" }}
            </p>
            <p class="text-xs text-gray-400">
              {{ formatDate(note.updatedAt) }}
            </p>
          </div>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
