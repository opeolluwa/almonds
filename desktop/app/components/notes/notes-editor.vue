<script setup lang="ts">
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

// const content = defineModel<string>({ required: true });
// const { content } = defineProps({
//   content: {
//     type: String,
//     required: true,
//   },
// });
// const emit = defineEmits(["update:modelValue"]);

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

function handleUpdate({ editor }: { editor: any }) {
  model.value = editor.getHTML();
}
</script>

<template>
  <Domternal
    :extensions="extensions"
    :content="model ?? ''"
    :on-update="handleUpdate"
  >
    <Domternal.Toolbar class="-mt-5" />
    <Domternal.Content class="bg-transparent dark:dm-theme-auto" />
    <Domternal.BubbleMenu class="" />
  </Domternal>
</template>
