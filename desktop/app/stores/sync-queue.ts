import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import type {
  Bookmark,
  Notes,
  RecycleBin,
  Reminder,
  Snippets,
  Todo,
  UserPreference,
  Workspaces,
} from "almond_kernel";

// JsonValue in Notes is a recursive type that exceeds TypeScript's instantiation
// depth limit inside Pinia's state inference — replace it with unknown.
type FlatNotes = Omit<Notes, "categories"> & { categories: unknown };

export const useSyncQueueStore = defineStore("sync_queue_store", {
  state: () => ({
    bookmarks: [] as Bookmark[],
    notes: [] as FlatNotes[],
    todo: [] as Todo[],
    workspaces: [] as Workspaces[],
    reminders: [] as Reminder[],
    userPreferences: [] as UserPreference[],
    snippets: [] as Snippets[],
    recycleBin: [] as RecycleBin[],
    initialized: false,
  }),

  actions: {
    async initialize() {
      await this.SyncAll();
      this.initialized = true;
    },

    async SyncBookmarks() {
      this.bookmarks = await invoke<Bookmark[]>("get_unsynced_bookmarks");
    },

    async SyncNotes() {
      this.notes = (await invoke("get_unsynced_notes")) as FlatNotes[];
    },

    async SyncTodo() {
      this.todo = await invoke<Todo[]>("get_unsynced_todos");
    },

    async SyncWorkspaces() {
      this.workspaces = await invoke<Workspaces[]>("get_unsynced_workspaces");
    },

    async SyncReminders() {
      this.reminders = await invoke<Reminder[]>("get_unsynced_reminders");
    },

    async SyncUserPreferences() {
      this.userPreferences = await invoke<UserPreference[]>(
        "get_unsynced_user_preferences",
      );
    },

    async SyncSnippets() {
      this.snippets = await invoke<Snippets[]>("get_unsynced_snippets");
    },

    async SyncRecycleBin() {
      this.recycleBin = await invoke<RecycleBin[]>("get_unsynced_recycle_bin");
    },

    async SyncAll() {
      await Promise.all([
        this.SyncBookmarks(),
        this.SyncNotes(),
        this.SyncTodo(),
        this.SyncWorkspaces(),
        this.SyncReminders(),
        this.SyncUserPreferences(),
        this.SyncSnippets(),
        this.SyncRecycleBin(),
      ]);
    },

    async ClearBookmarkQueue() {},
    async ClearNotesQueue() {},
    async ClearTodoQueue() {},
    async ClearWorkspacesQueue() {},
    async ClearRemindersQueue() {},
    async ClearUserPreferencesQueue() {},
    async ClearSnippetsQueue() {},
    async ClearRecycleBinQueue() {},

    async ClearAllQueues() {
      await Promise.all([
        this.ClearBookmarkQueue(),
        this.ClearNotesQueue(),
        this.ClearTodoQueue(),
        this.ClearWorkspacesQueue(),
        this.ClearRemindersQueue(),
        this.ClearUserPreferencesQueue(),
        this.ClearSnippetsQueue(),
        this.ClearRecycleBinQueue(),
      ]);
    },
  },
});
