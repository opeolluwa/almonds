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
        this.notes = await invoke<Note[]>("get_all_notes");
      } finally {
        this.loading = false;
      }
    },

    async fetchRecentNotes() {
      this.recentLoading = true;
      try {
        this.recent = await invoke<Note[]>("get_recently_added_notes");
      } finally {
        this.recentLoading = false;
      }
    },

    async createNote(payload: CreateNotePayload): Promise<Note> {
      const created = await invoke<Note>("create_note", { note: payload });
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
      });
      const idx = this.notes.findIndex((n) => n.identifier === identifier);
      if (idx !== -1) this.notes[idx] = updated;
      return updated;
    },

    async deleteNote(identifier: string) {
      await invoke("delete_note", { identifier });
      this.notes = this.notes.filter((n) => n.identifier !== identifier);
      this.recent = this.recent.filter((n) => n.identifier !== identifier);
    },
  },
});
