import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

export type RecycleBinItemType =
  | "note"
  | "todo"
  | "bookmark"
  | "reminder"
  | "snippet";

export interface RecycleBinEntry {
  identifier: string;
  itemId: string;
  itemType: RecycleBinItemType;
  payload: string;
  deletedAt: string;
}

export interface CreateRecycleBinEntryPayload {
  itemId: string;
  itemType: RecycleBinItemType;
  payload: string;
}

export const useRecycleBinStore = defineStore("recycle_bin_store", {
  state: () => ({
    entries: [] as RecycleBinEntry[],
    loading: false,
  }),

  getters: {
    byType: (state) => (type: RecycleBinItemType) =>
      state.entries.filter((e) => e.itemType === type),

    typeCounts: (state) => {
      const counts: Record<string, number> = {};

      for (const e of state.entries) {
        counts[e.itemType] = (counts[e.itemType] ?? 0) + 1;
      }

      return counts;
    },
  },

  actions: {
    async fetchEntries() {
      this.loading = true;

      try {
        this.entries = await invoke<RecycleBinEntry[]>(
          "get_all_recycle_bin_entries",
          {
            meta: await getWorkspaceMeta(),
          },
        );
      } finally {
        this.loading = false;
      }
    },

    async createEntry(
      payload: CreateRecycleBinEntryPayload,
    ): Promise<RecycleBinEntry> {
      const created = await invoke<RecycleBinEntry>(
        "create_recycle_bin_entry",
        {
          entry: payload,
          meta: await getWorkspaceMeta(),
        },
      );

      this.entries.unshift(created);

      return created;
    },

    async purgeEntry(identifier: string) {
      await invoke("purge_recycle_bin_entry", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.entries = this.entries.filter((e) => e.identifier !== identifier);
    },

    async purgeAll() {
      await invoke("purge_all_recycle_bin_entries", {
        meta: await getWorkspaceMeta(),
      });

      this.entries = [];
    },

    async fetchUnsynced() {
      const recycleBin = await invoke<RecycleBinEntry[]>(
        "get_unsynced_recycle_bin",
      );
      return recycleBin;
    },

    async syncUpstream() {
      // const recycleBin = await this.fetchUnsynced();
      // if (!recycleBin.length) return;
      // const { data, execute } = useMutation(`
      //   mutation SyncRecycleBin($input: [SyncRecycleBinInput!]!) {
      //     sync_recycle_bin(input: $input) { success error_message identifier }
      //   }
      // `);
      // await execute({ input: recycleBin });
      // const synced = data.value?.sync_recycle_bin
      //   .filter((r: SyncResult) => r.success)
      //   .map((r: SyncResult) => r.identifier);
      // if (synced?.length)
      //   await invoke("clear_synced_recycle_bin", { identifiers: synced });
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_recycle_bin", { identifiers });
    },
  },
});
