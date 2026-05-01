import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

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
        this.snippets = await invoke<Snippet[]>("get_all_snippets", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.loading = false;
      }
    },

    async fetchRecentSnippets() {
      this.recentLoading = true;
      try {
        this.recent = await invoke<Snippet[]>("get_recently_added_snippet", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.recentLoading = false;
      }
    },

    async createSnippet(payload: CreateSnippetPayload): Promise<Snippet> {
      const created = await invoke<Snippet>("create_snippet", {
        snippet: payload,
        meta: await getWorkspaceMeta(),
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
        meta: await getWorkspaceMeta(),
      });

      const idx = this.snippets.findIndex((s) => s.identifier === identifier);
      if (idx !== -1) this.snippets[idx] = updated;

      return updated;
    },

    async deleteSnippet(identifier: string) {
      await invoke("delete_snippet", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      await this.fetchSnippets();
    },

    async duplicateSnippet(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("duplicate_snippet", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      await this.fetchSnippets();
    },

    async transferSnippet(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_snippet", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.snippets = this.snippets.filter(
        (s) => s.identifier !== recordIdentifier,
      );
    },

    async fetchUnsynced() {
      try {
        const snippets = await invoke<Snippet[]>("get_unsynced_snippets");
        console.log(
          "Unsynced snippets fetched:",
          JSON.stringify(snippets, null, 2),
        );
        return snippets;
      } catch (error) {
        console.error("Error fetching unsynced snippets:", error);
        return [];
      }
    },

    async syncUpstream() {
      const snippets = await this.fetchUnsynced();
      if (!snippets.length) return;

      const input = snippets.map((s) => ({
        identifier: s.identifier,
        title: s.title ?? null,
        language: s.language ?? null,
        code: s.code,
        description: s.description ?? null,
        is_pinned: s.isPinned,
        created_at: s.createdAt,
        updated_at: s.updatedAt,
        workspace_identifier: (s as any).workspaceIdentifier ?? null,
      }));
      const query = gql`
        mutation SyncSnippets($input: [SyncSnippetInput!]!) {
          sync_snippet(input: $input) {
            success
            error_message
            identifier
          }
        }
      `;

      const { mutate } = useMutation(query, { variables: { input } });

      try {
        const data = await mutate();
        console.log("Snippets sync response:", data);
      } catch (error) {
        console.error("Error syncing snippets:", error);
      }
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_snippets", { identifiers });
    },
  },
});
