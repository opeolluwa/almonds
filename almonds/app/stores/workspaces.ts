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
  }),

  getters: {
    currentWorkspace: (state) => state.workspaces[0] || null,
  },

  actions: {
    async fetchWorkspaces() {
      this.loading = true;
      try {
        this.workspaces = await invoke<Workspace[]>("list_workspaces");
      } finally {
        this.loading = false;
      }
    },

    async createWorkspace(payload: CreateWorkspacePayload): Promise<Workspace> {
      const created = await invoke<Workspace>("create_workspace", {
        workspace: payload,
      });
      this.workspaces.push(created);
      return created;
    },
  },
});
