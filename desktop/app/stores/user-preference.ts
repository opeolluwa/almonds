import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { getWorkspaceMeta } from "~/composables/getWorkspaceMeta";
import { useMutation } from "villus";

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

export interface UserPreference {
  identifier: string;
  firstName: string;
  lastName: string;
  email: string;
  workspaceIdentifier: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateUserPreferencePayload {
  firstName: string;
  lastName: string;
  email: string;
}

export interface UpdateUserPreferencePayload {
  firstName?: string;
  lastName?: string;
  email?: string;
}

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
          "get_user_preference",
          {
            meta: await getWorkspaceMeta(),
          },
        );
      } finally {
        this.loading = false;
      }
    },

    async createPreference(
      payload: CreateUserPreferencePayload,
    ): Promise<UserPreference> {
      const created = await invoke<UserPreference>("create_user_preference", {
        preference: payload,
        meta: await getWorkspaceMeta(),
      });
      this.preference = created;
      return created;
    },

    async updatePreference(
      payload: UpdateUserPreferencePayload,
    ): Promise<UserPreference> {
      const updated = await invoke<UserPreference>("update_user_preference", {
        identifier: this.preference!.identifier,
        preference: payload,
        meta: await getWorkspaceMeta(),
      });
      this.preference = updated;
      return updated;
    },

    async fetchUnsynced() {
      const userPreferences = await invoke<UserPreference[]>(
        "get_unsynced_user_preferences",
      );
      return userPreferences;
    },

    async syncUpstream() {
      const userPreferences = await this.fetchUnsynced();
      if (!userPreferences.length) return;

      const { data, execute } = useMutation(`
        mutation SyncUserPreferences($input: [SyncUserPreferenceInput!]!) {
          sync_user_preference(input: $input) { success error_message identifier }
        }
      `);
      await execute({ input: userPreferences });

      const synced = data.value?.sync_user_preference
        .filter((r: SyncResult) => r.success)
        .map((r: SyncResult) => r.identifier);
      if (synced?.length)
        await invoke("clear_synced_user_preferences", { identifiers: synced });
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_user_preferences", { identifiers });
    },
  },
  persist: true,
});
