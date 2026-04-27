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

const readTime = computed(() => Math.max(1, Math.ceil(wordCount.value / 200)));

const charCount = computed(() => {
  return content.value.replace(/<[^>]*>/g, "").replace(/\s/g, "").length;
});

const lastSaved = ref<Date | null>(null);

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
  } else if (
    e.key === "Backspace" &&
    !tagInput.value &&
    categories.value.length
  ) {
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
    lastSaved.value = new Date();
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

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
    <!-- suppress default page title -->
    <template #page_title>
      <UButton
        variant="ghost"
        @click="router.back()"
        color="neutral"
        icon="heroicons:arrow-long-left"
      ></UButton>
    </template>

    <template #main_content>


      <div class="pb-20">
        <!-- Title -->
        <textarea
          v-model="title"
          placeholder="Untitled"
          rows="1"
          :disabled="submitting"
          class="w-full resize-none bg-transparent outline-none text-4xl font-bold text-gray-900 dark:text-gray-50 placeholder:text-gray-200 dark:placeholder:text-gray-200 leading-snug mb-3 overflow-hidden"
          @input="
            ($event.target as HTMLTextAreaElement).style.height = 'auto';
            ($event.target as HTMLTextAreaElement).style.height =
              ($event.target as HTMLTextAreaElement).scrollHeight + 'px';
          "
        />

        <!-- Tags row -->
        <div class="flex flex-wrap items-center gap-1.5 mb-8 min-h-5">
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
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
            class="bg-transparent outline-none text-xs text-gray-400 dark:text-gray-500 placeholder:text-gray-300 dark:placeholder:text-gray-400 w-20 min-w-0"
            @keydown="onTagKeydown"
            @blur="addTag"
          >
        </div>

        <!-- Editor -->
        <NotesEditor v-model="content" />

        <p v-if="error" class="text-xs text-red-500 mt-6">
          {{ error }}
        </p>
      </div>

 
    </template>

    <template #side_content>
      <!-- Save -->
      <UButton
        block
        size="sm"
        :loading="submitting"
        :disabled="!hasContent"
        class="mb-2"
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

      <USeparator class="my-5" />

      <!-- Stats -->
      <p class="text-[10px] font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-widest mb-3">
        Document
      </p>
      <div class="flex flex-col gap-0.5">
        <div
          v-for="stat in [
            { label: 'Words', value: wordCount },
            { label: 'Characters', value: charCount },
            { label: 'Tags', value: categories.length },
          ]"
          :key="stat.label"
          class="flex items-center justify-between py-2 border-b border-gray-50 dark:border-gray-800/60 text-xs"
        >
          <span class="text-gray-400">{{ stat.label }}</span>
          <span class="tabular-nums font-semibold text-gray-700 dark:text-gray-200">
            {{ stat.value }}
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
