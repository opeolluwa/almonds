import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { getWorkspaceMeta } from "~/composables/getWorkspaceMeta";

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
      try {
        const userPreferences = await invoke<UserPreference[]>(
          "get_unsynced_user_preferences",
        );
        console.log(
          "Unsynced user preferences fetched:",
          JSON.stringify(userPreferences, null, 2),
        );
        return userPreferences;
      } catch (error) {
        console.error("Error fetching unsynced user preferences:", error);
        return [];
      }
    },

    async syncUpstream() {
      const userPreferences = await this.fetchUnsynced();
      if (!userPreferences.length) return;

      const input = userPreferences.map((p) => ({
        identifier: p.identifier,
        first_name: p.firstName,
        last_name: p.lastName,
        email: p.email,
        created_at: p.createdAt,
        updated_at: p.updatedAt,
        workspace_identifier: p.workspaceIdentifier ?? null,
      }));
      const query = gql`
        mutation SyncUserPreferences($input: [SyncUserPreferenceInput!]!) {
          sync_user_preference(input: $input) {
            success
            error_message
            identifier
          }
        }
      `;

      const { mutate } = useMutation(query, { variables: { input } });

      try {
        const data = await mutate();
        console.log("User preferences sync response:", data);
      } catch (error) {
        console.error("Error syncing user preferences:", error);
      }
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_user_preferences", { identifiers });
    },
  },
  persist: true,
});
