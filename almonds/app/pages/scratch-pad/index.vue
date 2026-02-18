<script setup lang="ts">
definePageMeta({ layout: false });
const { notify } = useAppNotification();
const content = ref("");

const pads = [
  { title: "Quick thoughts", date: "Today", active: true },
  { title: "Meeting notes", date: "Yesterday", active: false },
  { title: "Ideas dump", date: "Feb 15", active: false },
];
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex items-center justify-end mb-6">
        <div class="flex items-center gap-2">
          <button
            class="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          >
            <UIcon name="heroicons:arrow-path" class="size-4" />
          </button>
          <button
            class="flex items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
          >
            <UIcon name="heroicons:plus" class="size-4" />
            New Pad
          </button>
        </div>
      </div>

      <div
        class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700"
        style="height: calc(100vh - 380px)"
      >
        <textarea
          v-model="content"
          class="w-full h-full p-6 text-sm text-gray-700 dark:text-gray-200 bg-transparent resize-none outline-none font-mono leading-relaxed placeholder-gray-400 dark:placeholder-gray-500"
          :placeholder="`# Scratch Pad

Start writing your thoughts here...

This is a free-form space for quick notes, ideas, and brainstorming.`"
        />
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Pads
      </h2>
      <div class="flex flex-col gap-1 mb-6">
        <div
          v-for="pad in pads"
          :key="pad.title"
          class="flex items-center gap-3 py-2.5 px-3 rounded-lg cursor-pointer transition-colors"
          :class="
            pad.active
              ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
          "
        >
          <UIcon name="heroicons:document" class="size-4" />
          <div class="flex-1">
            <p class="text-sm font-medium">{{ pad.title }}</p>
            <p
              class="text-xs"
              :class="pad.active ? 'text-accent-400' : 'text-gray-400'"
            >
              {{ pad.date }}
            </p>
          </div>
        </div>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Quick Actions
      </h2>
      <div class="flex flex-col gap-2">
        <button
          class="flex items-center gap-2 py-2 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors w-full text-left"
        >
          <UIcon name="heroicons:clipboard-document" class="size-4" />
          Copy all
        </button>
        <button
          class="flex items-center gap-2 py-2 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors w-full text-left"
        >
          <UIcon name="heroicons:arrow-down-tray" class="size-4" />
          Export as .md
        </button>
        <button
          class="flex items-center gap-2 py-2 px-3 rounded-lg text-sm text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 transition-colors w-full text-left"
        >
          <UIcon name="heroicons:trash" class="size-4" />
          Clear pad
        </button>
      </div>
    </template>
  </NuxtLayout>
</template>
