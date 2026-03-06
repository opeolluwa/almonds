import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface Workspace {
  identifier: string;
  name: string;
  description: string;
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
}

export const useWorkspacesStore = defineStore("workspaces_store", {
  state: () => ({
    workspaces: [] as Workspace[],
    loading: false,
    activeWorkspaceId: "" as string, // <- track active workspace
  }),

  getters: {
    currentWorkspace: (state) =>
      state.workspaces.find((w) => w.identifier === state.activeWorkspaceId) ||
      null,
  },

  actions: {
    async fetchWorkspaces() {
      this.loading = true;
      try {
        this.workspaces = await invoke<Workspace[]>("list_workspaces");
        // If no active workspace yet, set the first one
        if (!this.activeWorkspaceId && this.workspaces.length > 0) {
          this.activeWorkspaceId = this.workspaces[0]!.identifier;
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
      // Automatically set new workspace as active
      this.activeWorkspaceId = created.identifier;
      return created;
    },

    async updateWorkspace(payload: CreateWorkspacePayload): Promise<Workspace> {
      const created = await invoke<Workspace>("create_workspace", {
        workspace: payload,
      });
      this.workspaces.push(created);
      // Automatically set new workspace as active
      this.activeWorkspaceId = created.identifier;
      return created;
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

        return;
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

      const todoStore = useTodoStore();
      const bookmarksStore = useBookmarkStore();
      const recycleBinStore = useRecycleBinStore();
      const reminderStore = useReminderStore();
      const userPreferenceStore = useUserPreferenceStore();
      const snippetsStore = useSnippetStore();

      await todoStore.fetchTodos();
      await bookmarksStore.fetchBookmarks();
      await recycleBinStore.fetchEntries();
      await reminderStore.fetchReminders();
      await userPreferenceStore.fetchPreference();
      await snippetsStore.fetchSnippets();
    },
  },
  persist: true,
});
