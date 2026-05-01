import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

export interface Note {
  identifier: string;
  title: string;
  content: string;
  categories: string[];
  createdAt: string;
  updatedAt: string;
}

export interface CreateNotePayload {
  title: string;
  content: string;
  categories?: string[];
}

export type UpdateNotePayload = Partial<CreateNotePayload>;

export const useNoteStore = defineStore("notes_store", {
  state: () => ({
    notes: [] as Note[],
    recent: [] as Note[],
    loading: false,
    recentLoading: false,
  }),

  actions: {
    async fetchNotes() {
      this.loading = true;

      try {
        this.notes = await invoke<Note[]>("get_all_notes", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.loading = false;
      }
    },

    async fetchRecentNotes() {
      this.recentLoading = true;

      try {
        this.recent = await invoke<Note[]>("get_recently_added_notes", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.recentLoading = false;
      }
    },

    async createNote(payload: CreateNotePayload): Promise<Note> {
      console.log("Creating note with payload:", payload);
      const created = await invoke<Note>("create_note", {
        note: payload,
        meta: await getWorkspaceMeta(),
      });

      this.notes.unshift(created);

      await this.fetchRecentNotes();

      return created;
    },

    async updateNote(
      identifier: string,
      payload: UpdateNotePayload,
    ): Promise<Note> {
      const updated = await invoke<Note>("update_note", {
        identifier,
        note: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.notes.findIndex((n) => n.identifier === identifier);

      if (idx !== -1) {
        this.notes[idx] = updated;
      }

      return updated;
    },

    async deleteNote(identifier: string) {
      await invoke("delete_note", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.notes = this.notes.filter((n) => n.identifier !== identifier);
      this.recent = this.recent.filter((n) => n.identifier !== identifier);
    },

    async duplicateNote(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("duplicate_note", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      await this.fetchNotes();
    },

    async transferNote(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_note", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.notes = this.notes.filter((n) => n.identifier !== recordIdentifier);

      this.recent = this.recent.filter(
        (n) => n.identifier !== recordIdentifier,
      );
    },

    async exportAsPdf(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
    ) {
      await invoke("export_notes_as_pdf", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });
    },

    async fetchUnsynced() {
      try {
        const notes = await invoke<Note[]>("get_unsynced_notes");
        console.log("Unsynced notes fetched:", JSON.stringify(notes, null, 2));
        return notes;
      } catch (error) {
        console.error("Error fetching unsynced notes:", error);
        return [];
      }
    },

    async syncUpstream() {
      const notes = await this.fetchUnsynced();
      if (!notes.length) return;

      const input = notes.map((n) => ({
        identifier: n.identifier,
        title: n.title,
        content: n.content,
        categories: n.categories,
        created_at: n.createdAt,
        updated_at: n.updatedAt,
        workspace_identifier: (n as any).workspaceIdentifier ?? null,
      }));
      const query = gql`
        mutation SyncNotes($input: [SyncNoteInput!]!) {
          sync_note(input: $input) {
            success
            error_message
            identifier
          }
        }
      `;

      const { mutate } = useMutation(query, { variables: { input } });

      try {
        const data = await mutate();
        console.log("Notes checks response:", data);
      } catch (error) {
        console.error("Error syncing notes:", error);
      }
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_notes", { identifiers });
    },
  },
});
