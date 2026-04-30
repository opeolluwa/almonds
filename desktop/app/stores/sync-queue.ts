import { defineStore } from "pinia";
import { useClient, useMutation } from "villus";
import { useBookmarkStore } from "~/stores/bookmarks";
import { useNoteStore } from "~/stores/notes";
import { useTodoStore } from "~/stores/todo";
import { useWorkspacesStore } from "~/stores/workspaces";
import { useReminderStore } from "~/stores/reminder";
import { useUserPreferenceStore } from "~/stores/user-preference";
import { useSnippetStore } from "~/stores/snippets";
import { useRecycleBinStore } from "~/stores/recycle-bin";

export const graphqlClient = useClient({
  url: "http://localhost:8000/orchard",
});
export const useSyncQueueStore = defineStore("sync_queue_store", {
  state: () => ({
    networkAvailable: false,
    runningSync: false,
  }),

  actions: {
    async preflightCheck(name: string) {
      const { data, execute } = useMutation(
        `
        mutation Preflight($name: String!) {
          preflight(name: $name)
        }
      `,
        {context:{}},
      );
      await execute({ name });
      console.log("Preflight check response:", data.value);
    },

    async runSync() {
      if (this.runningSync || !this.networkAvailable) return;
      this.runningSync = true;
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
        this.runningSync = false;
      }
    },
  },
});
