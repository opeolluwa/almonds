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
      await invoke("duplicate_bookmark", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });
    
      await this.fetchBookmarks();
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
  },
  
});
