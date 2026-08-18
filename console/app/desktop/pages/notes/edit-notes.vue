<script setup lang="ts">
import { onBeforeRouteLeave } from "vue-router";

definePageMeta({ layout: false, keepalive: true, name: "Edit notes" });

const route = useRoute();
const router = useRouter();
const noteStore = useNoteStore();

const id = computed(() => route.query.id as string | undefined);
const original = computed(
  () => noteStore.notes.find((n) => n.identifier === id.value) ?? null,
);

const title = ref("");
const content = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

watch(
  original,
  (note) => {
    if (note && !saved.value) {
      title.value = note.title === "Untitled" ? "" : note.title;
      content.value = note.content;
    }
  },
  { immediate: true },
);

// ── word count ────────────────────────────────────────────────────────────────
const wordCount = computed(() => {
  const text = content.value.replace(/<[^>]*>/g, " ").trim();
  if (!text) return 0;
  return text.split(/\s+/).filter(Boolean).length;
});

const lastSaved = ref<Date | null>(null);
const notesEditor = ref<InstanceType<typeof NotesEditor> | null>(null);

const hasChanges = computed(() => {
  if (!original.value) return false;
  const origTitle =
    original.value.title === "Untitled" ? "" : original.value.title;
  return (
    title.value !== origTitle ||
    content.value !== original.value.content
  );
});

// ── save ──────────────────────────────────────────────────────────────────────
async function handleSave() {
  if (!original.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.updateNote(original.value.identifier, {
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

// Keyboard shortcut: Cmd/Ctrl+S
useEventListener("keydown", (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === "s") {
    e.preventDefault();
    if (hasChanges.value && !submitting.value) handleSave();
  }
});

onBeforeRouteLeave(async () => {
  if (submitting.value || saved.value) return;
  if (!hasChanges.value) return;
  try {
    await noteStore.updateNote(original.value!.identifier, {
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
  } catch (e) {
    console.error(e);
  }
});

// ── downloads ─────────────────────────────────────────────────────────────────
function downloadMarkdown() {
  if (!original.value) return;
  const filename = (title.value || "untitled").replace(/[^a-z0-9_\- ]/gi, "_");
  const blob = new Blob([content.value], {
    type: "text/markdown;charset=utf-8",
  });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `${filename}.md`;
  anchor.click();
  URL.revokeObjectURL(url);
}

function downloadPdf() {
  notesEditor.value?.editor?.commands.printDocument();
}

onMounted(async () => {
  if (noteStore.notes.length === 0) {
    await noteStore.fetchNotes();
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
      <!-- Not found -->
      <div
        v-if="!original && !noteStore.loading"
        class="flex flex-col items-center justify-center py-20 text-center"
      >
        <div class="mb-4 p-3 rounded-full bg-gray-100 dark:bg-gray-800">
          <UIcon name="heroicons:document-text" class="size-7 text-gray-400" />
        </div>
        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Note not found
        </h3>
        <button
          class="text-xs text-accent-500 hover:text-accent-600 font-medium mt-2"
          @click="router.push('/notes')"
        >
          Back to notes
        </button>
      </div>

      <!-- Loading -->
      <div
        v-else-if="noteStore.loading && !original"
        class="max-w-2xl mx-auto flex flex-col gap-4"
      >
        <USkeleton class="h-10 rounded-lg w-64" />
        <USkeleton class="h-4 rounded-lg w-32" />
        <USkeleton class="h-96 rounded-lg" />
      </div>

      <div v-else-if="original">
        <div class="mx-auto pb-20">
          <!-- Editor -->
          <NotesEditor ref="notesEditor" v-model="content" />

          <p v-if="error" class="text-xs text-red-500 mt-4">{{ error }}</p>
        </div>

        <!-- Sticky bottom bar -->
      </div>
    </template>

    <template #side_content>
      <template v-if="original">
        <!-- Document stats -->
        <div class="mb-6">
          <h2
            class="text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wide mb-3"
          >
            Document
          </h2>
          <div class="flex flex-col gap-2">
            <div class="flex items-center justify-between text-xs">
              <span class="text-gray-400">Words</span>
              <span
                class="font-medium text-gray-700 dark:text-gray-300 tabular-nums"
                >{{ wordCount }}</span
              >
            </div>
            <div class="flex items-center justify-between text-xs">
              <span class="text-gray-400">Modified</span>
              <span class="font-medium text-gray-700 dark:text-gray-300">
                {{
                  new Date(original.updatedAt).toLocaleDateString("en-US", {
                    month: "short",
                    day: "numeric",
                    year: "numeric",
                  })
                }}
              </span>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex flex-col gap-2 mb-6">
          <UButton
            block
            size="sm"
            :loading="submitting"
            :disabled="!hasChanges"
            :ui="{
              base: 'bg-accent-500 hover:bg-accent-600 disabled:bg-accent-600 disabled:text-gray-100 disabled:cursor-not-allowed py-2',
            }"
            @click="handleSave"
          >
            Save changes
          </UButton>
          <UButton
            block
            variant="ghost"
            size="sm"
            :disabled="submitting"
            :ui="{ base: 'text-accent-500' }"
            @click="router.push('/notes')"
          >
            Discard
          </UButton>
          <p
            class="text-center text-[10px] text-gray-300 dark:text-gray-600 mt-1"
          >
            {{
              submitting
                ? "Saving…"
                : hasChanges
                  ? "⌘S to save"
                  : "No unsaved changes"
            }}
          </p>
        </div>

        <!-- Export -->
        <div class="mb-6">
          <h2
            class="text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wide mb-3"
          >
            Export
          </h2>
          <div class="flex flex-col gap-2">
            <button
              class="flex items-center gap-2 px-3 py-2 rounded-lg text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors text-left"
              @click="downloadMarkdown"
            >
              <UIcon
                name="heroicons:document-text"
                class="size-3.5 shrink-0 text-gray-400"
              />
              Download as Markdown
            </button>
            <button
              class="flex items-center gap-2 px-3 py-2 rounded-lg text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors text-left"
              @click="downloadPdf"
            >
              <UIcon
                name="heroicons:arrow-down-tray"
                class="size-3.5 shrink-0 text-gray-400"
              />
              Download as PDF
            </button>
          </div>
        </div>

        <!-- Tips -->
        <div>
          <h2
            class="text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wide mb-3"
          >
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
              <UIcon
                name="heroicons:light-bulb"
                class="size-3.5 mt-0.5 shrink-0 text-accent-400"
              />
              {{ tip }}
            </li>
          </ul>
        </div>
      </template>
    </template>
  </NuxtLayout>
</template>
