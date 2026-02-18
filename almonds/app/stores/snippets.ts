import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type { Snippet } from "~/composables/snippets/useGetSnippets";

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

    async deleteSnippet(identifier: string) {
      await invoke("delete_snippet", { identifier });
      await this.fetchSnippets();
    },
  },
});
