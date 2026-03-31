<script setup lang="ts">
import { onBeforeRouteLeave } from "vue-router";

definePageMeta({ layout: false, name: "Create note", keepalive: true });

const router = useRouter();
const noteStore = useNoteStore();

const title = ref("");
const content = ref("");
const categories = ref<string[]>([]);
const tagInput = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

onActivated(() => {
  title.value = "";
  content.value = "";
  categories.value = [];
  tagInput.value = "";
  error.value = null;
  submitting.value = false;
  saved.value = false;
});

// ── word count ────────────────────────────────────────────────────────────────
const wordCount = computed(() => {
  const text = content.value.replace(/<[^>]*>/g, " ").trim();
  if (!text) return 0;
  return text.split(/\s+/).filter(Boolean).length;
});

const hasContent = computed(
  () => !!title.value.trim() || !!content.value.trim(),
);

// ── tags ──────────────────────────────────────────────────────────────────────
function addTag() {
  const tag = tagInput.value.trim().toLowerCase();
  if (tag && !categories.value.includes(tag)) {
    categories.value.push(tag);
  }
  tagInput.value = "";
}

function removeTag(tag: string) {
  categories.value = categories.value.filter((t) => t !== tag);
}

function onTagKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" || e.key === ",") {
    e.preventDefault();
    addTag();
  } else if (e.key === "Backspace" && !tagInput.value && categories.value.length) {
    categories.value.pop();
  }
}

// ── save ──────────────────────────────────────────────────────────────────────
async function handleSave() {
  if (!hasContent.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
      categories: categories.value,
    });
    saved.value = true;
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

// Keyboard shortcut: Cmd/Ctrl+S
useEventListener("keydown", (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === "s") {
    e.preventDefault();
    if (hasContent.value && !submitting.value) handleSave();
  }
});

onBeforeRouteLeave(async () => {
  if (submitting.value || saved.value) return;
  if (!hasContent.value) return;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
      categories: categories.value,
    });
  } catch (e) {
    console.error(e);
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-2xl mx-auto">
        <!-- Back link -->
        <button
          class="flex items-center gap-1.5 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 mb-6 transition-colors"
          @click="router.push('/notes')"
        >
          <UIcon name="heroicons:arrow-left" class="size-3.5" />
          All notes
        </button>

        <!-- Title -->
        <textarea
          v-model="title"
          placeholder="Untitled"
          rows="1"
          :disabled="submitting"
          class="w-full resize-none bg-transparent outline-none text-3xl font-bold text-gray-900 dark:text-gray-50 placeholder:text-gray-300 dark:placeholder:text-gray-600 leading-tight mb-4 overflow-hidden"
          @input="($event.target as HTMLTextAreaElement).style.height = 'auto'; ($event.target as HTMLTextAreaElement).style.height = ($event.target as HTMLTextAreaElement).scrollHeight + 'px'"
        />

        <!-- Tags row -->
        <div class="flex flex-wrap items-center gap-1.5 mb-5 min-h-6">
          <span
            v-for="tag in categories"
            :key="tag"
            class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-accent-50 dark:bg-accent-950 text-accent-600 dark:text-accent-300 text-xs font-medium"
          >
            {{ tag }}
            <button
              class="text-accent-400 hover:text-accent-600 dark:hover:text-accent-200 transition-colors leading-none"
              @click="removeTag(tag)"
            >
              <UIcon name="heroicons:x-mark" class="size-3" />
            </button>
          </span>
          <input
            v-model="tagInput"
            placeholder="Add tag…"
            class="bg-transparent outline-none text-xs text-gray-400 placeholder:text-gray-300 dark:placeholder:text-gray-600 w-20 min-w-0"
            @keydown="onTagKeydown"
            @blur="addTag"
          />
        </div>

        <!-- Divider -->
        <div class="border-t border-gray-100 dark:border-gray-800 mb-5" />

        <!-- Editor -->
        <NotesEditor v-model="content" />

        <p v-if="error" class="text-xs text-red-500 mt-4">{{ error }}</p>
      </div>
    </template>

    <template #side_content>
      <!-- Document stats -->
      <div class="mb-6">
        <h2 class="text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wide mb-3">
          Document
        </h2>
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-400">Words</span>
            <span class="font-medium text-gray-700 dark:text-gray-300 tabular-nums">{{ wordCount }}</span>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-400">Categories</span>
            <span class="font-medium text-gray-700 dark:text-gray-300 tabular-nums">{{ categories.length }}</span>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex flex-col gap-2 mb-6">
        <UButton
          block
          size="sm"
          :loading="submitting"
          :disabled="!hasContent"
          @click="handleSave"
        >
          Save note
        </UButton>
        <UButton
          block
          variant="ghost"
          size="sm"
          :disabled="submitting"
          @click="router.push('/notes')"
        >
          Discard
        </UButton>
        <p class="text-center text-[10px] text-gray-300 dark:text-gray-600 mt-1">
          {{ submitting ? 'Saving…' : hasContent ? '⌘S to save' : 'Start writing to save' }}
        </p>
      </div>

      <!-- Tips -->
      <div>
        <h2 class="text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wide mb-3">
          Tips
        </h2>
        <ul class="flex flex-col gap-2.5">
          <li
            v-for="tip in [
              'Type / for formatting commands.',
              'Press Enter after a tag to add it.',
              'Use ⌘S to save anytime.',
              'Navigating away auto-saves your work.',
            ]"
            :key="tip"
            class="flex items-start gap-2 text-xs text-gray-400 dark:text-gray-500"
          >
            <UIcon name="heroicons:light-bulb" class="size-3.5 mt-0.5 shrink-0 text-accent-400" />
            {{ tip }}
          </li>
        </ul>
      </div>
    </template>
  </NuxtLayout>
</template>
