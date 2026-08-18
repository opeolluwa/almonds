import type {
  CreateUserPreference,
  UpdateUserPreference,
  WorkspacePreferences,
} from "lunar";
import { defineStore } from "pinia";
import { invoke } from "~/utils/invoke";
import { getWorkspaceMeta } from "~/composables/getWorkspaceMeta";

export type UserPreference = WorkspacePreferences & { email: string };

export type CreateUserPreferencePayload = CreateUserPreference;

export type UpdateUserPreferencePayload = Partial<UpdateUserPreference>;

export const useUserPreferenceStore = defineStore("user_preference_store", {
  state: () => ({
    preference: null as UserPreference | null,
    loading: false,
  }),

  getters: {
    fullName: (state) =>
      state.preference
        ? `${state.preference.firstName} ${state.preference.lastName}`.trim()
        : "",
  },

  actions: {
    async fetchPreference() {
      this.loading = true;
      try {
        this.preference = await invoke<UserPreference | null>(
          "get_workspace_preference",
          {
            meta: await getWorkspaceMeta(),
          },
        );
      } catch (error) {
        console.error("[preferences] failed to fetch", error);
      } finally {
        this.loading = false;
      }
    },

    async createPreference(
      payload: CreateUserPreferencePayload,
    ): Promise<UserPreference> {
      const created = await invoke<UserPreference>(
        "create_workspace_preference",
        {
          preference: payload,
          meta: await getWorkspaceMeta(),
        },
      );
      this.preference = created;
      return created;
    },

    async updatePreference(
      payload: UpdateUserPreferencePayload,
    ): Promise<UserPreference> {
      const updated = await invoke<UserPreference>(
        "update_workspace_preference",
        {
          identifier: this.preference!.identifier,
          preference: payload,
          meta: await getWorkspaceMeta(),
        },
      );
      this.preference = updated;
      return updated;
    },

    async fetchUnsynced() {
      try {
        const userPreferences = await invoke<UserPreference[]>(
          "get_unsynced_workspace_preferences",
        );
        return userPreferences;
      } catch (error) {
        console.error("Error fetching unsynced user preferences:", error);
        return [];
      }
    },

    async syncUpstream() {},

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_workspace_preferences", { identifiers });
    },
  },
  persist: true,
});
