<script setup lang="ts">
import { useNoteStore } from "~/stores/notes";

const props = defineProps<{
  identifier: string;
  title: string;
  content: string;
  updatedAt: string;
}>();

const router = useRouter();
const noteStore = useNoteStore();

const deleting = ref(false);
const showDeleteConfirm = ref(false);

const preview = computed(() => {
  const stripped = props.content
    .replace(/#{1,6}\s+/g, "")
    .replace(/\*\*(.+?)\*\*/g, "$1")
    .replace(/\*(.+?)\*/g, "$1")
    .replace(/`{1,3}[^`]*`{1,3}/g, "")
    .replace(/\[(.+?)\]\(.+?\)/g, "$1")
    .replace(/^[-*>]\s+/gm, "")
    .replace(/\n+/g, " ")
    .trim();
  return stripped.length > 120 ? stripped.slice(0, 120) + "…" : stripped;
});

const formattedDate = computed(() =>
  new Date(props.updatedAt).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  }),
);

async function confirmDelete() {
  deleting.value = true;
  await noteStore.deleteNote(props.identifier);
}

function downloadMarkdown() {
  const filename = (props.title || "untitled").replace(/[^a-z0-9_\- ]/gi, "_");
  const blob = new Blob([props.content], { type: "text/markdown;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `${filename}.md`;
  anchor.click();
  URL.revokeObjectURL(url);
}
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow overflow-hidden cursor-pointer"
    @click="router.push(`/notes/edit-notes?id=${identifier}`)"
  >
    <div class="p-4">
      <h3 class="text-sm font-medium text-gray-800 dark:text-gray-200 mb-1.5 truncate">
        {{ title || "Untitled" }}
      </h3>
      <p class="text-xs text-gray-400 dark:text-gray-500 leading-relaxed line-clamp-2">
        {{ preview || "No content" }}
      </p>
    </div>

    <div
      class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
    >
      <p class="text-xs text-gray-400">{{ formattedDate }}</p>

      <div class="flex items-center gap-2" @click.stop>
        <button
          class="text-xs text-accent-600 dark:text-accent-400 hover:text-accent-700 font-medium transition-colors"
          @click="router.push(`/notes/edit-notes?id=${identifier}`)"
        >
          Edit
        </button>

        <button
          class="text-xs text-red-400 hover:text-red-600 dark:hover:text-red-300 transition-colors"
          @click="showDeleteConfirm = true"
        >
          Delete
        </button>

        <button
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          title="Download as .md"
          @click="downloadMarkdown"
        >
          <UIcon name="heroicons:arrow-down-tray" class="size-3.5" />
        </button>
      </div>
    </div>

    <UModal v-model:open="showDeleteConfirm" title="Delete note">
      <template #body>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Are you sure you want to delete
          <span class="font-medium text-gray-800 dark:text-gray-200">{{
            title || "Untitled"
          }}</span>? This action cannot be undone.
        </p>
      </template>
      <template #footer>
        <div class="flex justify-end gap-2">
          <UButton
            size="xs"
            variant="ghost"
            :disabled="deleting"
            @click="showDeleteConfirm = false"
          >
            Cancel
          </UButton>
          <UButton
            size="xs"
            color="error"
            :loading="deleting"
            @click="confirmDelete"
          >
            Delete
          </UButton>
        </div>
      </template>
    </UModal>
  </div>
</template>
