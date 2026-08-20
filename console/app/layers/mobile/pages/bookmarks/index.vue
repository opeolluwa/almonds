<script setup lang="ts">
import { useBookmarkStore } from "@shared/stores/bookmarks";
import EmptyState from "@shared/components/app/EmptyState.vue";
const bookmarkStore = useBookmarkStore();
</script>

<template>
  <!-- Loading -->
  <div v-if="bookmarkStore.loading" class="flex flex-col gap-3">
    <USkeleton v-for="i in 4" :key="i" class="h-24 rounded-lg" />
  </div>

  <!-- Empty state: no bookmarks at all -->
  <div
    v-else-if="bookmarkStore.bookmarks.length === 0"
    class="flex flex-col items-center justify-center py-20 text-center"
  >
    <EmptyState
      title="No bookmarks yet"
      description="Create your first bookmark to get started."
      icon="ri:bookmark-line"
      action-label="create bookmark"
      @action="navigateTo('/bookmarks/create-bookmark')"
    />
  </div>

  <!-- Bookmark list -->
  <!-- <div v-else class="flex flex-col gap-3">
    <BookmarkCard
      v-for="bookmark in filtered"
      :key="bookmark.identifier"
      :bookmark="bookmark"
      @delete="bookmarkStore.deleteBookmark"
      @preview="openUrl(bookmark.url)"
    />
  </div> -->
</template>
