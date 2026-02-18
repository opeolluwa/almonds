<script setup lang="ts">
import { useBookmarkStore, type BookmarkTag } from "~/stores/bookmarks";

definePageMeta({ layout: false });

const bookmarkStore = useBookmarkStore();

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

const filtered = computed(() =>
  activeTag.value === "all"
    ? bookmarkStore.bookmarks
    : bookmarkStore.byTag(activeTag.value),
);

onMounted(() => bookmarkStore.fetchBookmarks());

async function handleCreate(payload: { title: string; url: string; tag: BookmarkTag }) {
  await bookmarkStore.createBookmark(payload);
}
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <button
        class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
        @click="showAddModal = true"
      >
        <UIcon name="heroicons:plus" class="size-4" />
        Add Bookmark
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

      <!-- Empty state -->
      <div
        v-else-if="filtered.length === 0"
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
          {{
            activeTag === "all"
              ? "No bookmarks yet"
              : `No ${activeTag} bookmarks`
          }}
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500 max-w-xs">
          {{
            activeTag === "all"
              ? 'Save links you want to revisit. Click "Add Bookmark" to get started.'
              : "Try a different tag or add a new bookmark."
          }}
        </p>
      </div>

      <!-- Bookmark list -->
      <div v-else class="flex flex-col gap-3">
        <BookmarksBookmarkCard
          v-for="bookmark in filtered"
          :key="bookmark.identifier"
          :bookmark="bookmark"
          @delete="bookmarkStore.deleteBookmark"
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
</template>
