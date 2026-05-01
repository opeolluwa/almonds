import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";

export type BookmarkTag = "development" | "design" | "research" | "inspiration";

export interface Bookmark {
  identifier: string;
  title: string;
  url: string;
  tag: BookmarkTag;
  createdAt: string;
  updatedAt: string;
}

export interface CreateBookmarkPayload {
  title: string;
  url: string;
  tag: BookmarkTag;
}

export interface UpdateBookmarkPayload {
  title?: string;
  url?: string;
  tag?: BookmarkTag;
}

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

export const useBookmarkStore = defineStore("bookmark_store", {
  state: () => ({
    bookmarks: [] as Bookmark[],
    loading: false,
  }),

  getters: {
    byTag: (state) => (tag: BookmarkTag) =>
      state.bookmarks.filter((b) => b.tag === tag),

    tagCounts: (state) => {
      const counts: Record<BookmarkTag, number> = {
        development: 0,
        design: 0,
        research: 0,
        inspiration: 0,
      };

      for (const b of state.bookmarks) {
        counts[b.tag] = (counts[b.tag] ?? 0) + 1;
      }

      return counts;
    },
  },

  actions: {
    async fetchBookmarks() {
      this.loading = true;
      try {
        this.bookmarks = await invoke<Bookmark[]>("get_all_bookmarks", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.loading = false;
      }
    },

    async createBookmark(payload: CreateBookmarkPayload): Promise<Bookmark> {
      const created = await invoke<Bookmark>("create_bookmark", {
        bookmark: payload,
        meta: await getWorkspaceMeta(),
      });

      this.bookmarks.unshift(created);

      return created;
    },

    async updateBookmark(
      identifier: string,
      payload: UpdateBookmarkPayload,
    ): Promise<Bookmark> {
      const updated = await invoke<Bookmark>("update_bookmark", {
        identifier,
        bookmark: payload,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.bookmarks.findIndex((b) => b.identifier === identifier);

      if (idx !== -1) this.bookmarks[idx] = updated;

      return updated;
    },

    async deleteBookmark(identifier: string) {
      await invoke("delete_bookmark", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.bookmarks = this.bookmarks.filter(
        (b) => b.identifier !== identifier,
      );
    },

    async duplicateBookmark(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      try {
        await invoke("duplicate_bookmark", {
          recordIdentifier,
          previousWorkspaceIdentifier,
          targetWorkspaceIdentifier,
          meta: await getWorkspaceMeta(),
        });
      } catch (e) {
        console.error(e);
      } finally {
        await this.fetchBookmarks();
      }
    },

    async transferBookmark(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_bookmark", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.bookmarks = this.bookmarks.filter(
        (b) => b.identifier !== recordIdentifier,
      );
    },

    async fetchUnsynced() {
      try {
        const bookmarks = await invoke<Bookmark[]>("get_unsynced_bookmarks");
        console.log(
          "Unsynced bookmarks fetched:",
          JSON.stringify(bookmarks, null, 2),
        );
        return bookmarks;
      } catch (error) {
        console.error("Error fetching unsynced bookmarks:", error);
        return [];
      }
    },

    async syncUpstream() {
      const bookmarks = await this.fetchUnsynced();
      if (!bookmarks.length) return;

      const input = bookmarks.map((b) => ({
        identifier: b.identifier,
        title: b.title,
        url: b.url,
        tag: b.tag,
        created_at: b.createdAt,
        updated_at: b.updatedAt,
        workspace_identifier: (b as any).workspaceIdentifier ?? null,
      }));
      const query = gql`
        mutation SyncBookmarks($input: [SyncBookmarkInput!]!) {
          sync_bookmark(input: $input) {
            success
            error_message
            identifier
          }
        }
      `;

      const { mutate } = useMutation(query, { variables: { input } });

      try {
        const data = await mutate();
        console.log("Bookmarks checks response:", data);
      } catch (error) {
        console.error("Error syncing bookmarks:", error);
      }
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_bookmarks", { identifiers });
    },
  },
});
