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
import { useMutation } from "villus";

// JsonValue in Notes is a recursive type that exceeds TypeScript's instantiation
// depth limit inside Pinia's state inference — replace it with unknown.
type FlatNotes = Omit<Notes, "categories"> & { categories: unknown };

type QueueState = {
  bookmarks: Bookmark[];
  notes: FlatNotes[];
  todo: Todo[];
  workspaces: Workspaces[];
  reminders: Reminder[];
  userPreferences: UserPreference[];
  snippets: Snippets[];
  recycleBin: RecycleBin[];
};

const SYNC_COMMANDS: Record<keyof QueueState, string> = {
  bookmarks: "get_unsynced_bookmarks",
  notes: "get_unsynced_notes",
  todo: "get_unsynced_todos",
  workspaces: "get_unsynced_workspaces",
  reminders: "get_unsynced_reminders",
  userPreferences: "get_unsynced_user_preferences",
  snippets: "get_unsynced_snippets",
  recycleBin: "get_unsynced_recycle_bin",
};

const QUEUE_KEYS = Object.keys(SYNC_COMMANDS) as (keyof QueueState)[];

export const useSyncQueueStore = defineStore("sync_queue_store", {
  state: (): QueueState & { initialized: boolean } => ({
    bookmarks: [],
    notes: [],
    todo: [],
    workspaces: [],
    reminders: [],
    userPreferences: [],
    snippets: [],
    recycleBin: [],
    initialized: false,
  }),

  actions: {
    async preflightCheck(name: string) {
      const preflightGraphQL = `
mutation Preflight($name: String!) {
  preflight(name: $name)
}
`;

      const { data, execute } = useMutation(preflightGraphQL);

      await execute({ name });
      console.log("Preflight check response:", data.value);
    },

    async initialize() {
      await this.SyncAll();
      this.initialized = true;
    },

    async sync(key: keyof QueueState) {
      this.$patch({ [key]: await invoke(SYNC_COMMANDS[key]) });
    },

    async SyncAll() {
      await Promise.all(QUEUE_KEYS.map((k) => this.sync(k)));
    },

    clearQueue(key: keyof QueueState) {
      this.$patch({ [key]: [] });
    },

    ClearAllQueues() {
      QUEUE_KEYS.forEach((k) => this.clearQueue(k));
    },
  },
});
