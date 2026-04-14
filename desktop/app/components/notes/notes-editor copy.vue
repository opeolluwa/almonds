<script setup lang="ts">
import type { EditorToolbarItem } from "@nuxt/ui";

const value = ref("<h1>Hello World</h1>\n");

const items: EditorToolbarItem[] = [
  // Headings (dropdown)
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

  // Inline formatting
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

  // Alignment
  {
    kind: "textAlign",
    align: "left",
    icon: "i-lucide-align-left",
    tooltip: { text: "Align left" },
  },
  {
    kind: "textAlign",
    align: "center",
    icon: "i-lucide-align-center",
    tooltip: { text: "Align center" },
  },

  // Lists
  {
    kind: "bulletList",
    icon: "i-lucide-list",
    tooltip: { text: "Bullet list" },
  },
  {
    kind: "orderedList",
    icon: "i-lucide-list-ordered",
    tooltip: { text: "Ordered list" },
  },

  // Blocks
  {
    kind: "blockquote",
    icon: "i-lucide-quote",
    tooltip: { text: "Blockquote" },
  },

  // Links
  {
    kind: "link",
    icon: "i-lucide-link",
    tooltip: { text: "Insert link" },
  },
];
</script>

<template>
  <UEditor
    v-slot="{ editor }"
    v-model="value"
    placeholder="type here"
    class="-mt-5"
  >
    <UEditorToolbar :editor="editor" :items="items" floating class="-ml-1" />
  </UEditor>
</template>

<style>
* {
  user-select: none;
}
</style>
