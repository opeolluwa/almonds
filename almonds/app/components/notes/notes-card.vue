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
  await noteStore.deleteNote(props.identifier);
}

function downloadMarkdown() {
  const filename = (props.title || "untitled").replace(/[^a-z0-9_\- ]/gi, "_");
  const blob = new Blob([props.content], {
    type: "text/markdown;charset=utf-8",
  });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `${filename}.md`;
  anchor.click();
  URL.revokeObjectURL(url);
}
const workspaceStore = useWorkspacesStore();
const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);

const handleDuplicate = async (targetWorkspaceId: string) => {
  await noteStore.duplicateNote(
    props.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};

const handleTransfer = async (targetWorkspaceId: string) => {
  await noteStore.transferNote(
    props.identifier,
    currentWorkspaceId.value,
    targetWorkspaceId,
  );
};
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 hover:shadow-sm transition-shadow overflow-hidden cursor-pointer"
    @click="router.push(`/notes/edit-notes?id=${identifier}`)"
  >
    <div class="p-4">
      <h3
        class="text-sm font-medium text-gray-800 dark:text-gray-200 mb-1.5 truncate"
      >
        {{ title || "Untitled" }}
      </h3>
      <p
        class="text-xs text-gray-400 dark:text-gray-500 leading-relaxed line-clamp-2"
      >
        {{ preview || "No content" }}
      </p>
    </div>

    <div
      class="px-4 py-2 border-t border-gray-50 dark:border-gray-700 flex items-center justify-between"
    >
      <p class="text-xs text-gray-400">{{ formattedDate }}</p>

      <div class="flex items-center gap-2" @click.stop>
        <button
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          title="Download as .md"
          @click="downloadMarkdown"
        >
          <UIcon name="heroicons:arrow-down-tray" class="size-3.5" />
        </button>
        <MetaControls
          item-name="note"
          @edit-record="router.push(`/notes/edit-notes?id=${identifier}`)"
          @delete-record="confirmDelete"
          @duplicate-record="
            (targetWorkspaceId) => handleDuplicate(targetWorkspaceId)
          "
          @transfer-record="
            (targetWorkspaceId) => handleTransfer(targetWorkspaceId)
          "
        />
      </div>
    </div>

  </div>
</template>
