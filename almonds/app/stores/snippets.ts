import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface Snippet {
  identifier: string;
  title: string | null;
  language: string | null;
  code: string;
  description: string | null;
  isPinned: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateSnippetPayload {
  title: string | null;
  language: string | null;
  code: string;
  description: string | null;
}

export type UpdateSnippetPayload = Partial<CreateSnippetPayload>;

export const useSnippetStore = defineStore("snippets_store", {
  state: () => ({
    snippets: [] as Snippet[],
    recent: [] as Snippet[],
    loading: false,
    recentLoading: false,
  }),

  getters: {
    languages: (state): string[] => {
      const langs = new Set<string>();
      for (const snippet of state.snippets) {
        if (snippet.language) langs.add(snippet.language);
      }
      return Array.from(langs).sort();
    },
  },

  actions: {
    async fetchSnippets() {
      this.loading = true;
      try {
        this.snippets = await invoke<Snippet[]>("get_all_snippets");
      } finally {
        this.loading = false;
      }
    },

    async fetchRecentSnippets() {
      this.recentLoading = true;
      try {
        this.recent = await invoke<Snippet[]>("get_recently_added_snippet");
      } finally {
        this.recentLoading = false;
      }
    },

    async createSnippet(payload: CreateSnippetPayload): Promise<Snippet> {
      const created = await invoke<Snippet>("create_snippet", {
        snippet: payload,
      });
      this.snippets.unshift(created);
      await this.fetchRecentSnippets();
      return created;
    },

    async updateSnippet(
      identifier: string,
      payload: UpdateSnippetPayload,
    ): Promise<Snippet> {
      const updated = await invoke<Snippet>("update_snippet", {
        identifier,
        snippet: payload,
      });
      const idx = this.snippets.findIndex((s) => s.identifier === identifier);
      if (idx !== -1) this.snippets[idx] = updated;
      return updated;
    },

    async deleteSnippet(identifier: string) {
      await invoke("delete_snippet", { identifier });
      await this.fetchSnippets();
    },
  },
});
