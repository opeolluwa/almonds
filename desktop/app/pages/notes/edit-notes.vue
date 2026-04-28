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
const categories = ref<string[]>([]);
const tagInput = ref("");
const submitting = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

watch(
  original,
  (note) => {
    if (note && !saved.value) {
      title.value = note.title === "Untitled" ? "" : note.title;
      content.value = note.content;
      categories.value = note.categories ?? [];
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

const readTime = computed(() => Math.max(1, Math.ceil(wordCount.value / 200)));

const lastSaved = ref<Date | null>(null);

const hasChanges = computed(() => {
  if (!original.value) return false;
  const origTitle =
    original.value.title === "Untitled" ? "" : original.value.title;
  return (
    title.value !== origTitle ||
    content.value !== original.value.content ||
    JSON.stringify(categories.value) !==
      JSON.stringify(original.value.categories ?? [])
  );
});

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
  if (!original.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.updateNote(original.value.identifier, {
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
      categories: categories.value,
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

function escapeHtml(str: string) {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function downloadPdf() {
  if (!original.value) return;
  const safeTitle = escapeHtml(title.value || "Untitled");
  const tags = categories.value
    .map((t) => `<span class="tag">${escapeHtml(t)}</span>`)
    .join("");

  const doc = `<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>${safeTitle}</title>
  <style>
    body { font-family: Georgia, serif; max-width: 680px; margin: 48px auto; color: #1a1a1a; line-height: 1.7; font-size: 16px; }
    h1.note-title { font-size: 2rem; font-weight: 700; margin: 0 0 0.75rem; }
    .tags { display: flex; gap: 0.4rem; flex-wrap: wrap; margin-bottom: 1.5rem; }
    .tag { background: #f0f0f0; padding: 2px 10px; border-radius: 9999px; font-size: 0.75rem; font-family: sans-serif; }
    .divider { border: none; border-top: 1px solid #e5e5e5; margin: 1.25rem 0 1.5rem; }
    .content h1 { font-size: 1.5rem; font-weight: 700; margin: 1.5rem 0 0.5rem; }
    .content h2 { font-size: 1.25rem; font-weight: 600; margin: 1.25rem 0 0.4rem; }
    .content h3 { font-size: 1.1rem; font-weight: 600; margin: 1rem 0 0.3rem; }
    .content p { margin: 0.75rem 0; }
    .content ul, .content ol { padding-left: 1.5rem; margin: 0.75rem 0; }
    .content li { margin: 0.25rem 0; }
    .content code { background: #f4f4f4; padding: 2px 5px; border-radius: 3px; font-size: 0.875em; font-family: monospace; }
    .content pre { background: #f4f4f4; padding: 1rem; border-radius: 6px; overflow: auto; }
    .content pre code { background: none; padding: 0; }
    .content blockquote { border-left: 3px solid #ccc; padding-left: 1rem; color: #555; margin: 1rem 0; }
    .content strong { font-weight: 700; }
    .content em { font-style: italic; }
    @media print { body { margin: 0; } }
  </style>
</head>
<body>
  <h1 class="note-title">${safeTitle}</h1>
  ${tags ? `<div class="tags">${tags}</div>` : ""}
  <hr class="divider">
  <div class="content">${content.value}</div>
</body>
</html>`;

  const iframe = document.createElement("iframe");
  iframe.style.cssText =
    "position:fixed;left:-9999px;top:-9999px;width:1px;height:1px;";
  iframe.srcdoc = doc;
  iframe.onload = () => {
    iframe.contentWindow!.print();
    iframe.contentWindow!.onafterprint = () =>
      document.body.removeChild(iframe);
  };
  document.body.appendChild(iframe);
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
      <UButton
        variant="ghost"
        color="neutral"
        icon="heroicons:arrow-long-left"
        @click="router.back()"
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
          <!-- Title -->
          <textarea
            v-model="title"
            placeholder="Untitled"
            rows="1"
            :disabled="submitting"
            class="w-full resize-none bg-transparent outline-none text-3xl font-bold text-gray-900 dark:text-gray-50 placeholder:text-gray-300 dark:placeholder:text-gray-600 leading-tight mb-4 overflow-hidden"
            @input="
              ($event.target as HTMLTextAreaElement).style.height = 'auto';
              ($event.target as HTMLTextAreaElement).style.height =
                ($event.target as HTMLTextAreaElement).scrollHeight + 'px';
            "
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
              autocapitalize="off"
              autocorrect="off"
              spellcheck="false"
              class="bg-transparent outline-none text-xs text-gray-400 dark:text-gray-300 placeholder:text-gray-300 dark:placeholder:text-gray-600 w-20 min-w-0"
              @keydown="onTagKeydown"
              @blur="addTag"
            >
          </div>

          <!-- Divider -->
          <div class="border-t border-gray-100 dark:border-gray-800 mb-5" />

          <!-- Editor -->
          <NotesEditor v-model="content" />

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
              <span class="text-gray-400">Categories</span>
              <span
                class="font-medium text-gray-700 dark:text-gray-300 tabular-nums"
                >{{ categories.length }}</span
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
            @click="handleSave"
          >
            Save changes
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
