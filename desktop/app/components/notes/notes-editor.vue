<script setup lang="ts">
import { marked } from "marked";
import { Domternal } from "@domternal/vue";
import DOMPurify from "dompurify";
import { Details } from "@domternal/extension-details";
import { CodeBlockLowlight } from "@domternal/extension-code-block-lowlight";
import { createLowlight, all } from "lowlight";
import {
  StarterKit,
  BubbleMenu,
  BaseKeymap,
  Superscript,
  Subscript,
  Text,
  TextStyle,
  TextAlign,
  Code,
  Heading,
  ListItem,
  BulletList,
  OrderedList,
  Link,
  Placeholder,
} from "@domternal/core";
import { Table } from "@domternal/extension-table";
import { Image } from "@domternal/extension-image";
import {
  Emoji,
  emojis,
  createEmojiSuggestionRenderer,
} from "@domternal/extension-emoji";
import {
  BlockContextMenu,
  BlockHandle,
  FloatingMenu,
  KeyboardReorder,
  SlashCommand,
  SmartPaste,
} from "@domternal/extension-block-menu";

const colorMode = useColorMode();
const isDark = computed(() => colorMode.value === "dark");
const lowlight = createLowlight(all);
const dmVars = computed(() =>
  isDark.value
    ? {
        "--dm-accent": "var(--color-accent-400)",
        "--dm-accent-hover": "var(--color-accent-300)",
        "--dm-accent-surface":
          "color-mix(in srgb, var(--color-accent-400) 15%, transparent)",
        "--dm-bg": "var(--color-surface-900)",
        "--dm-surface": "var(--color-surface-800)",
        "--dm-border-color": "var(--color-surface-700)",
      }
    : {
        "--dm-accent": "var(--color-accent-500)",
        "--dm-accent-hover": "var(--color-accent-600)",
        "--dm-accent-surface":
          "color-mix(in srgb, var(--color-accent-500) 10%, transparent)",
      },
);

const extensions = [
  StarterKit,
  BubbleMenu,
  Table,
  ListItem.configure({
    HTMLAttributes: { class: "notes_list_item" },
  }),
  BulletList.configure({
    HTMLAttributes: { class: "note_list_unordered" },
  }),
  OrderedList.configure({
    HTMLAttributes: { class: "note_list_ordered" },
  }),
  Superscript,
  Subscript,
  Text,
  BaseKeymap,
  Details,
  TextStyle,
  Code,
  TextAlign,
  BlockHandle.configure({ nested: true }),
  BlockContextMenu,
  SlashCommand,
  SmartPaste,
  KeyboardReorder,
  Heading.configure({
    levels: [1, 2, 3, 4, 5, 6],
    HTMLAttributes: { class: "notes_heading" },
  }),
  CodeBlockLowlight.configure({ lowlight }),
  Emoji.configure({
    emojis,
    suggestion: { render: createEmojiSuggestionRenderer() },
  }),
  Link.configure({
    protocols: ["http:", "https:"],
    openOnClick: true,
    autolink: true,
    linkOnPaste: true,
    defaultProtocol: "https",
  }),
  FloatingMenu.configure({
    element: document.getElementById("floating-menu")!,
    requireExplicitTrigger: false,
  }),
  Image.configure({
    //TODO: replace with actual upload handler that uploads to server and returns URL
    uploadHandler: async (file) => {
      const form = new FormData();
      form.append("file", file);
      const res = await fetch("/api/upload", { method: "POST", body: form });
      const { url } = await res.json();
      return url;
    },
  }),

  Placeholder.configure({
    placeholder: ({ node }) => {
      if (node.type.name === "heading") return "Enter a heading...";
      if (node.type.name === "codeBlock") return "// Write code here";
      if (node.type.name === "table") return "";
      return "Type something...";
    },
  }),
];

const model = defineModel<string>();

// Old notes were saved as Markdown (UEditor content-type="markdown").
// Domternal expects HTML, so detect and convert on the way in.
function isHtml(s: string): boolean {
  const t = s.trimStart();
  return t.startsWith("<") && /<[a-z][\s\S]*>/i.test(t);
}

const initialContent = computed(() => {
  const raw = model.value ?? "";
  if (!raw || isHtml(raw)) return raw;
  return marked.parse(raw) as string;
});

function handleUpdate({ editor }: { editor: any }) {
  model.value = DOMPurify.sanitize(editor.getHTML());
}
</script>

<template>
  <div :class="{ 'dm-theme-dark': isDark }" :style="dmVars" class="h-full">
    <Domternal
      :extensions="extensions"
      :content="initialContent"
      :on-update="handleUpdate"
    >
      <Domternal.Toolbar class="mb-12 -mt-5" />
      <Domternal.Content class="bg-transparent" />
      <Domternal.BubbleMenu class="mb-5" />
    </Domternal>
  </div>
</template>

<style></style>
