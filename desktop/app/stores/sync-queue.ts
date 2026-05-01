import { ref } from "vue";
import { useNetwork } from "@vueuse/core";
import { defineStore } from "pinia";
import { useBookmarkStore } from "~/stores/bookmarks";
import { useNoteStore } from "~/stores/notes";
import { useTodoStore } from "~/stores/todo";
import { useWorkspacesStore } from "~/stores/workspaces";
import { useReminderStore } from "~/stores/reminder";
import { useUserPreferenceStore } from "~/stores/user-preference";
import { useSnippetStore } from "~/stores/snippets";
import { useRecycleBinStore } from "~/stores/recycle-bin";

export const useSyncQueueStore = defineStore("sync_queue_store", () => {
  const { isOnline } = useNetwork();
  const runningSync = ref(false);

  async function preflightCheck(name: string) {
    const query = gql`
      mutation PreflightCheck($name: String!) {
        preflight(name: $name)
      }
    `;

    const variables = { name };

    const { mutate } = useMutation(query, { variables });
    const data = await mutate();
    console.log("Preflight check response:", data);
  }

  async function runSync() {
    if (runningSync.value || !isOnline.value) return;
    runningSync.value = true;
    try {
      await Promise.all([
        useBookmarkStore().syncUpstream(),
        useNoteStore().syncUpstream(),
        useTodoStore().syncUpstream(),
        useWorkspacesStore().syncUpstream(),
        useReminderStore().syncUpstream(),
        useUserPreferenceStore().syncUpstream(),
        useSnippetStore().syncUpstream(),
        useRecycleBinStore().syncUpstream(),
      ]);
    } finally {
      runningSync.value = false;
    }
  }

  return { isOnline, runningSync, preflightCheck, runSync };
});
