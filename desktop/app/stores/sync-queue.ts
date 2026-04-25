export const useSyncQueueStore = defineStore("sync_queue_store", {
  state: () => ({
    bookmarks: [],
    notes: [],
    todo: [],
    workspaces: [],
    reminders: [],
    userPreferences: [],
    snippets: [],
    initialized: false,
  }),

  actions: {
    async initialize() {},
    async SyncBookmarks() {},
    async SyncNotes() {},
    async SyncTodo() {},
    async SyncWorkspaces() {},
    async SyncReminders() {},
    async SyncUserPreferences() {},
    async SyncSnippets() {},

    async SyncAll() {
      await Promise.all([
        this.SyncBookmarks(),
        this.SyncNotes(),
        this.SyncTodo(),
        this.SyncWorkspaces(),
        this.SyncReminders(),
        this.SyncUserPreferences(),
        this.SyncSnippets(),
      ]);
    },

    async ClearBookmarkQueue() {},
    async ClearNotesQueue() {},
    async ClearTodoQueue() {},
    async ClearWorkspacesQueue() {},
    async ClearRemindersQueue() {},
    async ClearUserPreferencesQueue() {},
    async ClearSnippetsQueue() {},

    async ClearAllQueues() {
      await Promise.all([
        this.ClearBookmarkQueue(),
        this.ClearNotesQueue(),
        this.ClearTodoQueue(),
        this.ClearWorkspacesQueue(),
        this.ClearRemindersQueue(),
        this.ClearUserPreferencesQueue(),
        this.ClearSnippetsQueue(),
      ]);
    }

    
  },
});
