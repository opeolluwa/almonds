<script setup lang="ts">
import { ref } from "vue";

import type { EditorToolbarItem } from "@nuxt/ui";
import NotesEditor from "~/components/notes/notes-editor.vue";
const value = ref("");

definePageMeta({
  layout: false,
});

const items: EditorToolbarItem[][] = [
  [
    {
      icon: "i-lucide-heading",
      tooltip: { text: "Headings" },
      content: { align: "start" },
      items: [
        {
          kind: "heading",
          level: 1,
          icon: "i-lucide-heading-1",
          label: "Heading 1",
        },
        {
          kind: "heading",
          level: 2,
          icon: "i-lucide-heading-2",
          label: "Heading 2",
        },
        {
          kind: "heading",
          level: 3,
          icon: "i-lucide-heading-3",
          label: "Heading 3",
        },
        {
          kind: "heading",
          level: 4,
          icon: "i-lucide-heading-4",
          label: "Heading 4",
        },
      ],
    },
  ],
  [
    {
      kind: "mark",
      mark: "bold",
      icon: "i-lucide-bold",
      tooltip: { text: "Bold" },
    },
    {
      kind: "mark",
      mark: "italic",
      icon: "i-lucide-italic",
      tooltip: { text: "Italic" },
    },
    {
      kind: "mark",
      mark: "underline",
      icon: "i-lucide-underline",
      tooltip: { text: "Underline" },
    },
    {
      kind: "mark",
      mark: "strike",
      icon: "i-lucide-strikethrough",
      tooltip: { text: "Strikethrough" },
    },
    {
      kind: "mark",
      mark: "code",
      icon: "i-lucide-code",
      tooltip: { text: "Code" },
    },
  ],
];

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

      <div
        class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 hidden"
        style="height: calc(100vh - 180px)"
      >
        <UEditor
          v-slot="{ editor }"
          v-model="value"
          content-type="markdown"
          placeholder="Start writing..."
        >
          <UEditorToolbar :editor="editor" :items="items" layout="bubble" />
        </UEditor>
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
