<script setup lang="ts">
import NotesEditor from "~/components/notes/notes-editor.vue";
import { useNoteStore } from "~/stores/notes";

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
const error = ref<string | null>(null);

watch(
  original,
  (note) => {
    if (note) {
      title.value = note.title;
      content.value = note.content;
    }
  },
  { immediate: true },
);

async function handleSave() {
  if (!original.value) return;
  submitting.value = true;
  error.value = null;
  try {
    await noteStore.updateNote(original.value.identifier, {
      title: title.value.trim() || "Untitled",
      content: content.value,
    });
    router.push("/notes");
  } catch (e) {
    error.value = String(e);
    submitting.value = false;
  }
}

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

onMounted(async () => {
  if (noteStore.notes.length === 0) {
    await noteStore.fetchNotes();
  }
});
</script>

<template>
  <NuxtLayout name="default">
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

      <template v-else-if="original">
        <!-- Title -->
        <UInput
          v-model="title"
          placeholder="Note title…"
          size="xl"
          variant="none"
          class="mb-2 w-full"
          :ui="{ base: 'text-3xl font-bold placeholder:font-normal placeholder:text-muted' }"
          :disabled="submitting"
        />

        <!-- Editor -->
        <div
          class="border border-gray-100 dark:border-gray-700 rounded-lg overflow-hidden"
        >
          <NotesEditor v-model="content" />
        </div>

        <p v-if="error" class="text-xs text-red-500 mt-3">{{ error }}</p>

        <!-- Actions -->
        <div class="flex items-center gap-2 mt-4">
          <UButton size="sm" :loading="submitting" @click="handleSave">
            Save changes
          </UButton>
          <UButton
            variant="ghost"
            size="sm"
            :disabled="submitting"
            @click="router.push('/notes')"
          >
            Cancel
          </UButton>
          <UButton
            variant="ghost"
            size="sm"
            :disabled="submitting"
            title="Download as .md"
            @click="downloadMarkdown"
          >
            <UIcon name="heroicons:arrow-down-tray" class="size-4" />
          </UButton>
        </div>
      </template>

      <!-- Loading -->
      <div v-else class="flex flex-col gap-3">
        <USkeleton class="h-10 rounded-lg w-64" />
        <USkeleton class="h-96 rounded-lg" />
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Tips
      </h2>
      <ul class="flex flex-col gap-3">
        <li
          v-for="tip in [
            'Changes are saved when you click Save changes.',
            'Select text to access AI-powered improvements.',
            'Use the download button to export this note as Markdown.',
            'Type / for quick formatting commands.',
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
    </template>
  </NuxtLayout>
</template>
