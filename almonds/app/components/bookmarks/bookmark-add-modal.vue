<script setup lang="ts">
import type { BookmarkTag, CreateBookmarkPayload } from "~/stores/bookmarks";

defineProps<{
  tags: { label: string; value: BookmarkTag }[];
}>();

const open = defineModel<boolean>("open", { required: true });

const emit = defineEmits<{ create: [payload: CreateBookmarkPayload] }>();

const form = reactive({
  title: "",
  url: "",
  tag: "development" as BookmarkTag,
});
const submitting = ref(false);

async function handleSubmit() {
  if (!form.title.trim() || !form.url.trim()) return;
  submitting.value = true;
  try {
    emit("create", {
      title: form.title.trim(),
      url: form.url.trim(),
      tag: form.tag,
    });
    form.title = "";
    form.url = "";
    form.tag = "development";
    open.value = false;
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <UModal v-model:open="open" title="Add Bookmark">
    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
        <div class="flex flex-col gap-1">
          <label class="text-xs font-medium text-gray-600 dark:text-gray-400"
            >Title</label
          >
          <UInput
            v-model="form.title"
            placeholder="e.g. Vue.js Docs"
            size="sm"
            :disabled="submitting"
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-xs font-medium text-gray-600 dark:text-gray-400"
            >URL</label
          >
          <UInput
            v-model="form.url"
            placeholder="https://example.com"
            size="sm"
            :disabled="submitting"
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-xs font-medium text-gray-600 dark:text-gray-400"
            >Tag</label
          >
          <USelectMenu
            v-model="form.tag"
            :items="tags.map((t) => ({ label: t.label, value: t.value }))"
            value-key="value"
            size="sm"
            :disabled="submitting"
          />
        </div>
        <div class="flex gap-2">
          <UButton
            type="submit"
            size="sm"
            :loading="submitting"
            :disabled="!form.title.trim() || !form.url.trim()"
          >
            Save
          </UButton>
          <UButton
            type="button"
            variant="ghost"
            size="sm"
            :disabled="submitting"
            @click="open = false"
          >
            Cancel
          </UButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
