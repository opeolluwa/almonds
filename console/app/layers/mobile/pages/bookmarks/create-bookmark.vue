<script setup lang="ts">
import { useBookmarkStore, type BookmarkTag } from "@shared/stores/bookmarks";

const router = useRouter();
const bookmarkStore = useBookmarkStore();

const TAGS: { label: string; value: BookmarkTag }[] = [
  { label: "Development", value: "development" },
  { label: "Design", value: "design" },
  { label: "Research", value: "research" },
  { label: "Inspiration", value: "inspiration" },
];

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
    await bookmarkStore.createBookmark({
      title: form.title.trim(),
      url: form.url.trim(),
      tag: form.tag,
    });
    router.push("/bookmarks");
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
    New Bookmark
  </h2>

  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
  >
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

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
          >Tag</label
        >
        <div class="flex gap-1.5 flex-wrap">
          <button
            v-for="tag in TAGS"
            :key="tag.value"
            type="button"
            class="px-3 py-1.5 rounded-lg text-xs font-medium transition-colors"
            :class="
              form.tag === tag.value
                ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 ring-1 ring-accent-200 dark:ring-accent-800'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400'
            "
            @click="form.tag = tag.value"
          >
            {{ tag.label }}
          </button>
        </div>
      </div>

      <div class="flex gap-2 mt-2">
        <UButton
          type="submit"
          size="sm"
          :loading="submitting"
          :disabled="!form.title.trim() || !form.url.trim()"
          class="px-4 py-2"
        >
          Save bookmark
        </UButton>
        <UButton
          type="button"
          variant="ghost"
          size="sm"
          :disabled="submitting"
          @click="router.push('/bookmarks')"
        >
          Discard
        </UButton>
      </div>
    </form>
  </div>
</template>
