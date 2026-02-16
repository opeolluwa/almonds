<script setup lang="ts">
definePageMeta({ layout: false });

const boards = [
  { title: "UI Inspiration", items: 8, color: "violet" },
  { title: "Color Palettes", items: 12, color: "rose" },
  { title: "Typography", items: 5, color: "amber" },
  { title: "Illustrations", items: 3, color: "emerald" },
];

const images = [
  { src: "https://picsum.photos/seed/a/400/300", title: "Dashboard layout" },
  { src: "https://picsum.photos/seed/b/400/400", title: "Card design" },
  { src: "https://picsum.photos/seed/c/400/250", title: "Navigation pattern" },
  { src: "https://picsum.photos/seed/d/400/350", title: "Color palette" },
  { src: "https://picsum.photos/seed/e/400/300", title: "Typography scale" },
  { src: "https://picsum.photos/seed/f/400/450", title: "Icon set" },
];

const colorMap: Record<string, string> = {
  violet: "bg-violet-100 dark:bg-violet-900 text-violet-700 dark:text-violet-300",
  rose: "bg-rose-100 dark:bg-rose-900 text-rose-700 dark:text-rose-300",
  amber: "bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-300",
  emerald: "bg-emerald-100 dark:bg-emerald-900 text-emerald-700 dark:text-emerald-300",
};
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">Moodboard</h1>
        <button class="flex items-center gap-2 py-2 px-4 bg-violet-600 text-white rounded-lg text-sm font-medium hover:bg-violet-700 transition-colors">
          <UIcon name="heroicons:plus" class="size-4" />
          Add Image
        </button>
      </div>

      <div class="columns-2 gap-3 space-y-3">
        <div
          v-for="image in images"
          :key="image.title"
          class="break-inside-avoid bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 overflow-hidden hover:shadow-sm transition-shadow cursor-pointer group"
        >
          <div class="relative">
            <img :src="image.src" :alt="image.title" class="w-full object-cover" />
            <div class="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors flex items-end">
              <p class="text-white text-sm font-medium p-3 opacity-0 group-hover:opacity-100 transition-opacity">{{ image.title }}</p>
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">Boards</h2>
      <div class="flex flex-col gap-2 mb-6">
        <div
          v-for="board in boards"
          :key="board.title"
          class="flex items-center gap-3 py-2.5 px-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
        >
          <div class="size-8 rounded-lg flex items-center justify-center" :class="colorMap[board.color]">
            <UIcon name="heroicons:squares-2x2" class="size-4" />
          </div>
          <div class="flex-1">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300">{{ board.title }}</p>
            <p class="text-xs text-gray-400">{{ board.items }} items</p>
          </div>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">Tags</h2>
      <div class="flex flex-wrap gap-2">
        <span class="px-2.5 py-1 rounded-full bg-violet-50 dark:bg-violet-950 text-violet-600 dark:text-violet-300 text-xs">UI Design</span>
        <span class="px-2.5 py-1 rounded-full bg-rose-50 dark:bg-rose-950 text-rose-600 dark:text-rose-300 text-xs">Colors</span>
        <span class="px-2.5 py-1 rounded-full bg-amber-50 dark:bg-amber-950 text-amber-600 dark:text-amber-300 text-xs">Typography</span>
        <span class="px-2.5 py-1 rounded-full bg-emerald-50 dark:bg-emerald-950 text-emerald-600 dark:text-emerald-300 text-xs">Layout</span>
        <span class="px-2.5 py-1 rounded-full bg-sky-50 dark:bg-sky-950 text-sky-600 dark:text-sky-300 text-xs">Icons</span>
      </div>
    </template>
  </NuxtLayout>
</template>
