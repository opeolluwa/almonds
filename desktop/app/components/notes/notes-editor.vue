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

const props = defineProps<{
  modelValue?: string;
  placeholder?: string;
  disabled?: boolean;
  readonly?: boolean;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();


const value = computed({
  get: () => props.modelValue ?? "",
  set: (v) => emit("update:modelValue", v),
});


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
</script>

<template>
  <Domternal
    :extensions="extensions"
    :content="value"
    :placeholder="placeholder"
    :editable="!disabled && !readonly"
    @update:modelValue="emit('update:modelValue', $event)"
  >
    <Domternal.Toolbar class="-mt-5" />
    <Domternal.Content class="bg-transparent dark:dm-theme-auto" />
    <Domternal.BubbleMenu class="" />
  </Domternal>
</template>
