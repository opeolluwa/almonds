<script setup lang="ts">
definePageMeta({ layout: false });

const boards: { title: string; items: number; color: string }[] = [];
const images: { src: string; title: string }[] = [];
</script>

<template>
  <NuxtLayout name="default">
    <template #primary_cta>
      <!-- Desktop: full label -->
      <div class="hidden md:flex items-center justify-end">
        <button
          class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          Add Image
        </button>
      </div>
      <!-- Mobile: icon-only round FAB -->
      <button
        class="md:hidden flex items-center justify-center w-14 h-14 bg-accent-500 text-white rounded-full shadow-xl active:scale-95 transition-transform"
        aria-label="Add Image"
      >
        <UIcon name="heroicons:plus" class="size-6" />
      </button>
    </template>

    <template #main_content>
      <div
        v-if="images.length === 0"
        class="flex flex-col items-center justify-center h-full py-24 text-center"
      >
        <div
          class="size-16 rounded-2xl bg-gray-100 dark:bg-gray-800 flex items-center justify-center mb-4"
        >
          <UIcon
            name="heroicons:photo"
            class="size-8 text-gray-400 dark:text-gray-500"
          />
        </div>
        <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          No images yet
        </p>
        <p class="text-xs text-gray-400 dark:text-gray-500">
          Add images to start building your moodboard.
        </p>
      </div>
      <div v-else class="columns-2 gap-3 space-y-3">
        <div
          v-for="image in images"
          :key="image.title"
          class="break-inside-avoid bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 overflow-hidden hover:shadow-sm transition-shadow cursor-pointer group"
        >
          <div class="relative">
            <img
              :src="image.src"
              :alt="image.title"
              class="w-full object-cover"
            >
            <div
              class="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors flex items-end"
            >
              <p
                class="text-white text-sm font-medium p-3 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                {{ image.title }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Boards
      </h2>
      <div
        v-if="boards.length === 0"
        class="flex flex-col items-center justify-center py-6 text-center mb-6"
      >
        <UIcon
          name="heroicons:squares-2x2"
          class="size-6 text-gray-300 dark:text-gray-600 mb-2"
        />
        <p class="text-xs text-gray-400 dark:text-gray-500">No boards yet</p>
      </div>
      <div v-else class="flex flex-col gap-2 mb-6">
        <div
          v-for="board in boards"
          :key="board.title"
          class="flex items-center gap-3 py-2.5 px-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
        >
          <div class="size-8 rounded-lg flex items-center justify-center">
            <UIcon name="heroicons:squares-2x2" class="size-4" />
          </div>
          <div class="flex-1">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ board.title }}
            </p>
            <p class="text-xs text-gray-400">{{ board.items }} items</p>
          </div>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tags
      </h2>
      <p class="text-xs text-gray-400 dark:text-gray-500">No tags yet</p>
    </template>
  </NuxtLayout>
</template>
