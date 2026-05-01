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
      try {
        const recycleBin = await invoke<RecycleBinEntry[]>(
          "get_unsynced_recycle_bin",
        );
        console.log(
          "Unsynced recycle bin fetched:",
          JSON.stringify(recycleBin, null, 2),
        );
        return recycleBin;
      } catch (error) {
        console.error("Error fetching unsynced recycle bin:", error);
        return [];
      }
    },

    async syncUpstream() {
      const recycleBin = await this.fetchUnsynced();
      if (!recycleBin.length) return;

      const input = recycleBin.map((e) => ({
        identifier: e.identifier,
        item_id: e.itemId,
        item_type: e.itemType,
        payload: e.payload,
        deleted_at: e.deletedAt,
        workspace_identifier: (e as any).workspaceIdentifier ?? null,
      }));
      const query = gql`
        mutation SyncRecycleBin($input: [SyncRecycleBinInput!]!) {
          sync_recycle_bin(input: $input) {
            success
            error_message
            identifier
          }
        }
      `;

      const { mutate } = useMutation(query, { variables: { input } });

      try {
        const data = await mutate();
        console.log("Recycle bin sync response:", data);
      } catch (error) {
        console.error("Error syncing recycle bin:", error);
      }
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_recycle_bin", { identifiers });
    },
  },
});
