import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

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
    
      this.notes = this.notes.filter(
        (n) => n.identifier !== recordIdentifier,
      );
    
      this.recent = this.recent.filter(
        (n) => n.identifier !== recordIdentifier,
      );
    },
  },
});
