<script setup lang="ts">
definePageMeta({ layout: false });

const bookmarks = [
  {
    title: "Vue.js Documentation",
    url: "https://vuejs.org",
    tag: "Development",
    date: "Feb 15, 2026",
  },
  {
    title: "Tailwind CSS Cheatsheet",
    url: "https://tailwindcss.com",
    tag: "Design",
    date: "Feb 14, 2026",
  },
  {
    title: "Nuxt 4 Migration Guide",
    url: "https://nuxt.com",
    tag: "Development",
    date: "Feb 12, 2026",
  },
  {
    title: "Figma Community Resources",
    url: "https://figma.com",
    tag: "Design",
    date: "Feb 10, 2026",
  },
];

const tags = ["All", "Development", "Design", "Research", "Inspiration"];
const activeTag = ref("All");
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <button
        class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
      >
        <UIcon name="heroicons:plus" class="size-4" />
        Add Bookmark
      </button>
    </template>

    <template #main_content>
      <div class="flex gap-2 mb-4">
        <button
          v-for="tag in tags"
          :key="tag"
          class="px-3 py-1 rounded-full text-xs font-medium transition-colors"
          :class="
            activeTag === tag
              ? 'bg-accent-500 text-white'
              : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
          "
          @click="activeTag = tag"
        >
          {{ tag }}
        </button>
      </div>

      <div class="flex flex-col gap-3">
        <div
          v-for="bookmark in bookmarks"
          :key="bookmark.title"
          class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow flex items-center gap-4"
        >
          <UIcon
            name="heroicons:bookmark-solid"
            class="size-5 text-accent-500 shrink-0"
          />
          <div class="flex-1 min-w-0">
            <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200">
              {{ bookmark.title }}
            </h3>
            <p class="text-xs text-gray-400 truncate">{{ bookmark.url }}</p>
          </div>
          <span
            class="px-2 py-1 rounded-full bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300 text-xs font-medium"
            >{{ bookmark.tag }}</span
          >
          <p class="text-xs text-gray-400 shrink-0">{{ bookmark.date }}</p>
          <UIcon
            name="heroicons:ellipsis-vertical"
            class="size-4 text-gray-400 cursor-pointer"
          />
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Collections
      </h2>
      <div class="flex flex-col gap-2">
        <div
          class="flex items-center gap-3 py-2 px-3 rounded-lg bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 text-sm font-medium"
        >
          <UIcon name="heroicons:folder-solid" class="size-4" />
          <span>All Bookmarks</span>
          <span class="ml-auto text-xs">{{ bookmarks.length }}</span>
        </div>
        <div
          class="flex items-center gap-3 py-2 px-3 rounded-lg text-gray-600 dark:text-gray-400 text-sm hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
        >
          <UIcon name="heroicons:code-bracket" class="size-4" />
          <span>Development</span>
          <span class="ml-auto text-xs text-gray-400">2</span>
        </div>
        <div
          class="flex items-center gap-3 py-2 px-3 rounded-lg text-gray-600 dark:text-gray-400 text-sm hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
        >
          <UIcon name="heroicons:paint-brush" class="size-4" />
          <span>Design</span>
          <span class="ml-auto text-xs text-gray-400">2</span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
