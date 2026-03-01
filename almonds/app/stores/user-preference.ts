import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface UserPreference {
  identifier: string;
  firstName: string;
  lastName: string;
  email: string;
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
      });
      this.preference = updated;
      return updated;
    },
  },
});
