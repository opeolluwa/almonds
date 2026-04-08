<script setup lang="ts">
import type { BookmarkTag, CreateBookmarkPayload } from "~/stores/bookmarks";
import AppInput from "../forms/app-input.vue";
import AppSelect from "../forms/app-select.vue";
import _ from "lodash";

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
        <AppInput
          v-model="form.title"
          placeholder="Bookmark title"
          label="Title"
          name="bookmark title"
          :disabled="submitting"
        />

        <AppInput
          v-model="form.url"
          label="URL"
          name="bookmark url"
          placeholder="https://example.com"
          :disabled="submitting"
        />

        <AppSelect
          v-model="form.tag"
          :items="
            tags.map((t) => ({ label: _.capitalize(t.label), value: t.value }))
          "
          value-key="value"
          label="Tag"
          name="bookmark tag"
          :ui="{ base: 'almond_input_box' }"
          :disabled="submitting"
        />

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
