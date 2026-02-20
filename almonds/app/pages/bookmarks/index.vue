<script setup lang="ts">
import { useBookmarkStore, type BookmarkTag } from "~/stores/bookmarks";

definePageMeta({ layout: false });

const bookmarkStore = useBookmarkStore();
const { searchQuery, setSearch, clearSearch } = useAppSearch();

const TAGS: { label: string; value: BookmarkTag | "all" }[] = [
  { label: "All", value: "all" },
  { label: "Development", value: "development" },
  { label: "Design", value: "design" },
  { label: "Research", value: "research" },
  { label: "Inspiration", value: "inspiration" },
];

const TAG_ICONS: Record<BookmarkTag, string> = {
  development: "heroicons:code-bracket",
  design: "heroicons:paint-brush",
  research: "heroicons:magnifying-glass",
  inspiration: "heroicons:light-bulb",
};

const activeTag = ref<BookmarkTag | "all">("all");
const showAddModal = ref(false);

const previewBookmark = ref<import("~/stores/bookmarks").Bookmark | null>(null);
const showPreview = ref(false);

function openPreview(bookmark: import("~/stores/bookmarks").Bookmark) {
  previewBookmark.value = bookmark;
  showPreview.value = true;
}

const filtered = computed(() => {
  let list =
    activeTag.value === "all"
      ? bookmarkStore.bookmarks
      : bookmarkStore.byTag(activeTag.value);

  const q = searchQuery.value.trim().toLowerCase();
  if (q) {
    list = list.filter(
      (b) =>
        b.title.toLowerCase().includes(q) ||
        b.url.toLowerCase().includes(q) ||
        b.tag.toLowerCase().includes(q),
    );
  }

  return list;
});

onMounted(async () => {
  setSearch({ placeholder: "Search bookmarks..." });
  await bookmarkStore.fetchBookmarks();
});

onUnmounted(() => clearSearch());

async function handleCreate(payload: {
  title: string;
  url: string;
  tag: BookmarkTag;
}) {
  await bookmarkStore.createBookmark(payload);
}
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <!-- Desktop: full label -->
      <div class="hidden md:flex items-center justify-end">
        <button
          class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
          @click="showAddModal = true"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          Add Bookmark
        </button>
      </div>
      <!-- Mobile: icon-only round FAB -->
      <button
        class="md:hidden flex items-center justify-center w-14 h-14 bg-accent-500 text-white rounded-full shadow-xl active:scale-95 transition-transform"
        aria-label="Add Bookmark"
        @click="showAddModal = true"
      >
        <UIcon name="heroicons:plus" class="size-6" />
      </button>
    </template>

    <template #main_content>
      <BookmarksBookmarkTagFilters v-model="activeTag" :tags="TAGS" />

      <!-- Loading -->
      <div
        v-if="bookmarkStore.loading"
        class="flex items-center justify-center py-16 text-gray-400"
      >
        <UIcon name="heroicons:arrow-path" class="size-5 animate-spin mr-2" />
        <span class="text-sm">Loading bookmarks…</span>
      </div>

      <!-- Empty state: no bookmarks at all -->
      <div
        v-else-if="bookmarkStore.bookmarks.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center gap-3"
      >
        <div
          class="size-14 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center"
        >
          <UIcon
            name="heroicons:bookmark"
            class="size-7 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="text-sm font-medium text-gray-600 dark:text-gray-400">
          No bookmarks yet
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500 max-w-xs">
          Save links you want to revisit. Click "Add Bookmark" to get started.
        </p>
      </div>

      <!-- Empty state: search / tag filter has no results -->
      <div
        v-else-if="filtered.length === 0"
        class="flex flex-col items-center justify-center py-20 text-center gap-3"
      >
        <div
          class="size-14 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center"
        >
          <UIcon
            name="heroicons:magnifying-glass"
            class="size-7 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="text-sm font-medium text-gray-600 dark:text-gray-400">
          No results found
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500 max-w-xs">
          Try a different search term or tag.
        </p>
        <div class="flex gap-3">
          <button
            v-if="searchQuery"
            class="text-xs text-accent-500 hover:text-accent-600 font-medium"
            @click="searchQuery = ''"
          >
            Clear search
          </button>
          <button
            v-if="activeTag !== 'all'"
            class="text-xs text-gray-400 hover:text-gray-600 font-medium"
            @click="activeTag = 'all'"
          >
            Clear filter
          </button>
        </div>
      </div>

      <!-- Bookmark list -->
      <div v-else class="flex flex-col gap-3">
        <BookmarksBookmarkCard
          v-for="bookmark in filtered"
          :key="bookmark.identifier"
          :bookmark="bookmark"
          @delete="bookmarkStore.deleteBookmark"
          @preview="openPreview"
        />
      </div>
    </template>

    <template #side_content>
      <BookmarksBookmarkCollections
        v-model="activeTag"
        :tags="TAGS"
        :tag-icons="TAG_ICONS"
        :total-count="bookmarkStore.bookmarks.length"
        :tag-counts="bookmarkStore.tagCounts"
      />
    </template>
  </NuxtLayout>

  <BookmarksBookmarkAddModal
    v-model:open="showAddModal"
    :tags="TAGS.slice(1) as { label: string; value: BookmarkTag }[]"
    @create="handleCreate"
  />

  <BookmarksBookmarkPreview
    v-model:open="showPreview"
    :bookmark="previewBookmark"
  />
</template>
