<script setup lang="ts">
import { onBeforeRouteLeave } from "vue-router";

definePageMeta({ layout: false, name: "New note", keepalive: true });

const router = useRouter();
const noteStore = useNoteStore();

const title = ref("");
const content = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

onActivated(() => {
  title.value = "";
  content.value = "";
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

const _readTime = computed(() => Math.max(1, Math.ceil(wordCount.value / 200)));

const charCount = computed(() => {
  return content.value.replace(/<[^>]*>/g, "").replace(/\s/g, "").length;
});

const lastSaved = ref<Date | null>(null);

const hasContent = computed(
  () => !!title.value.trim() || !!content.value.trim(),
);

// ── save ──────────────────────────────────────────────────────────────────────
async function handleSave() {
  if (!hasContent.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.createNote({
      title: title.value.trim() || "Untitled",
      content: content.value,
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
    });
  } catch (e) {
    console.error(e);
  }
});
</script>

<template>
  <NuxtLayout name="default">
    <template #page_title>
      <textarea
        v-model="title"
        placeholder="Untitled"
        rows="1"
        :disabled="submitting"
        class="w-full resize-none bg-transparent outline-none text-3xl font-bold text-gray-900 dark:text-gray-200 placeholder:text-gray-400 dark:placeholder:text-gray-500 leading-tight mb-0 overflow-hidden"
        @input="
          ($event.target as HTMLTextAreaElement).style.height = 'auto';
          ($event.target as HTMLTextAreaElement).style.height =
            ($event.target as HTMLTextAreaElement).scrollHeight + 'px';
        "
      />
    </template>
    <template #main_content>
      <NotesEditor v-model="content" />
      <p v-if="error" class="text-xs text-red-500 mt-6">
        {{ error }}
      </p>
    </template>

    <template #side_content>
      <!-- Save -->
      <UButton
        block
        size="sm"
        :loading="submitting"
        :disabled="!hasContent"
        class="mb-2"
        :ui="{
          base: 'bg-accent-500 hover:bg-accent-600 disabled:bg-accent-600 disabled:text-gray-100 disabled:cursor-not-allowed py-2',
        }"
        @click="handleSave"
      >
        Save note
      </UButton>
      <UButton
        block
        variant="ghost"
        size="sm"
        :disabled="submitting"
        :ui="{
          base: 'text-accent-500 hover:text-accent-600 disabled:text-accent-600 disabled:text-gray-100 disabled:cursor-not-allowed py-2',
        }"
        @click="router.push('/notes')"
      >
        Discard
      </UButton>

      <USeparator class="my-5" />

      <!-- Stats -->
      <p
        class="text-[10px] font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-widest mb-3"
      >
        Document
      </p>
      <div class="flex flex-col gap-0.5">
        <div
          v-for="stat in [
            { label: 'Words', value: wordCount },
            { label: 'Characters', value: charCount },
          ]"
          :key="stat.label"
          class="flex items-center justify-between py-2 border-b border-gray-50 dark:border-gray-800/60 text-xs"
        >
          <span class="text-gray-400">{{ stat.label }}</span>
          <span
            class="tabular-nums font-semibold text-gray-700 dark:text-gray-200"
          >
            {{ stat.value }}
          </span>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
