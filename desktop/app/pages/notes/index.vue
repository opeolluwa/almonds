<script setup lang="ts">
definePageMeta({ layout: false });

const noteStore = useNoteStore();
const router = useRouter();
const { searchQuery, setSearch, clearSearch } = useAppSearch();
import NotesCard from "~/components/notes/notes-card.vue";
type NoteSort = "name-asc" | "name-desc" | "date-newest" | "date-oldest";
const sortBy = ref<NoteSort>("date-newest");

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
      <!-- Sort control -->
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
        <button
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
          @click="navigateTo('/notes/create-notes')"
        >
          Create note
        </button>
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
