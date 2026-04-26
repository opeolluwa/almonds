<script setup lang="ts">
import { marked } from "marked";
import { Domternal } from "@domternal/vue";
import {
  StarterKit,
  BubbleMenu,
  Superscript,
  Subscript,
  Text,
  TextStyle,
  TextAlign,
  Code,
} from "@domternal/core";
import { Table } from "@domternal/extension-table";
import { Image } from "@domternal/extension-image";
import {
  Emoji,
  emojis,
  createEmojiSuggestionRenderer,
} from "@domternal/extension-emoji";

const colorMode = useColorMode();
const isDark = computed(() => colorMode.value === "dark");

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
  Superscript,
  Subscript,
  Text,
  TextStyle,
  Code,
  TextAlign,
  Emoji.configure({
    emojis,
    suggestion: { render: createEmojiSuggestionRenderer() },
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
  model.value = editor.getHTML();
}
</script>

<template>
  <div :class="{ 'dm-theme-dark': isDark}" :style="dmVars" class="h-full">
    <Domternal
      :extensions="extensions"
      :content="initialContent"
      :on-update="handleUpdate"
    >
      <Domternal.Toolbar class="-mt-5" />
      <Domternal.Content class="bg-transparent" />
      <Domternal.BubbleMenu class="" />
    </Domternal>
  </div>
</template>
