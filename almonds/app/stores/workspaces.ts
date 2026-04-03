import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface Workspace {
  identifier: string;
  name: string;
  description: string;
  isDefault: boolean;
  isHidden: boolean;
  isSecured: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateWorkspacePayload {
  name: string;
  description: string;
}

export interface UpdateWorkspacePayload {
  name?: string;
  description?: string;
  isDefault?: boolean;
  isHidden?: boolean;
  isSecured?: boolean;
  /** Plain-text password; pass empty string to remove the password. */
  password?: string;
}

export const useWorkspacesStore = defineStore("workspaces_store", {
  state: () => ({
    workspaces: [] as Workspace[],
    loading: false,
    activeWorkspaceId: "" as string,
    /** Identifiers of secured workspaces the user has unlocked this session. */
    unlockedWorkspaceIds: [] as string[],
  }),

  getters: {
    currentWorkspace: (state) =>
      state.workspaces.find((w) => w.identifier === state.activeWorkspaceId) ||
      null,

    visibleWorkspaces: (state) => state.workspaces.filter((w) => !w.isHidden),

    isWorkspaceUnlocked: (state) => (identifier: string) =>
      !state.workspaces.find((w) => w.identifier === identifier)?.isSecured ||
      state.unlockedWorkspaceIds.includes(identifier),

    isCurrentWorkspaceLocked: (state) => {
      const current = state.workspaces.find(
        (w) => w.identifier === state.activeWorkspaceId,
      );
      return !!current?.isSecured && !state.unlockedWorkspaceIds.includes(state.activeWorkspaceId);
    },
  },

  actions: {
    async fetchWorkspaces() {
      this.loading = true;
      try {
        this.workspaces = await invoke<Workspace[]>("list_workspaces");
        if (!this.activeWorkspaceId && this.workspaces.length > 0) {
          // Prefer the default workspace on first load
          const defaultWs = this.workspaces.find((w) => w.isDefault);
          this.activeWorkspaceId = (
            defaultWs ?? this.workspaces[0]!
          ).identifier;
        }
      } finally {
        this.loading = false;
      }
    },

    async createWorkspace(payload: CreateWorkspacePayload): Promise<Workspace> {
      const created = await invoke<Workspace>("create_workspace", {
        workspace: payload,
      });
      this.workspaces.push(created);
      this.activeWorkspaceId = created.identifier;
      return created;
    },

    async updateWorkspace(
      identifier: string,
      payload: UpdateWorkspacePayload,
    ): Promise<Workspace> {
      const updated = await invoke<Workspace>("update_workspace", {
        identifier,
        workspace: payload,
      });
      const idx = this.workspaces.findIndex((w) => w.identifier === identifier);
      if (idx !== -1) this.workspaces[idx] = updated;
      return updated;
    },

    async deleteWorkspace(identifier: string): Promise<void> {
      const { notify } = useAppNotification();
      try {
        await invoke<Workspace>("delete_workspace", {
          identifier,
          meta: await getWorkspaceMeta(),
        });

        await this.fetchWorkspaces();
        notify({
          message: "Workspace deleted",
          type: "success",
        });
      } catch (error) {
        notify({
          message:
            (error as unknown as Error).message || "Failed to delete workspace",
          type: "error",
        });
      }
    },

    async setActiveWorkspace(identifier: string) {
      if (this.workspaces.find((w) => w.identifier === identifier)) {
        this.activeWorkspaceId = identifier;
      } else {
        console.warn("Workspace not found:", identifier);
      }

      const noteStore = useNoteStore();
      const todoStore = useTodoStore();
      const bookmarksStore = useBookmarkStore();
      const recycleBinStore = useRecycleBinStore();
      const reminderStore = useReminderStore();
      const userPreferenceStore = useUserPreferenceStore();
      const snippetsStore = useSnippetStore();

      await noteStore.fetchNotes();
      await noteStore.fetchRecentNotes();
      await todoStore.fetchTodos();
      await bookmarksStore.fetchBookmarks();
      await recycleBinStore.fetchEntries();
      await reminderStore.fetchReminders();
      await userPreferenceStore.fetchPreference();
      await snippetsStore.fetchSnippets();
    },

    async verifyWorkspacePassword(
      identifier: string,
      password: string,
    ): Promise<boolean> {
      return invoke<boolean>("verify_workspace_password", {
        identifier,
        password,
      });
    },

    unlockWorkspace(identifier: string) {
      if (!this.unlockedWorkspaceIds.includes(identifier)) {
        this.unlockedWorkspaceIds.push(identifier);
      }
    },
  },
  persist: {
    omit: ["unlockedWorkspaceIds"],
  },
});
