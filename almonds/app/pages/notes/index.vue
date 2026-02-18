<script setup lang="ts">
import NotesEditor from "~/components/notes/notes-editor.vue";

definePageMeta({
  layout: false,
});

const notes = [
  { title: "Project roadmap", date: "Feb 16, 2026" },
  { title: "Meeting notes", date: "Feb 15, 2026" },
  { title: "Design ideas", date: "Feb 14, 2026" },
];
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex items-center justify-end mb-6">
        <button
          class="flex justify-end items-center gap-2 py-2 px-4 bg-accent-500 text-white rounded-lg text-sm font-medium hover:bg-accent-600 transition-colors"
        >
          <UIcon name="heroicons:plus" class="size-4" />
          New Note
        </button>
      </div>

      <NotesEditor />
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Recently modified
      </h2>
      <div class="flex flex-col gap-1">
        <div
          v-for="note in notes"
          :key="note.title"
          class="flex items-center gap-3 py-2.5 px-3 rounded-lg text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors"
        >
          <UIcon name="heroicons:document-text" class="size-4 text-gray-400" />
          <div class="flex-1">
            <p class="font-medium text-gray-700 dark:text-gray-300">
              {{ note.title }}
            </p>
            <p class="text-xs text-gray-400">{{ note.date }}</p>
          </div>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
