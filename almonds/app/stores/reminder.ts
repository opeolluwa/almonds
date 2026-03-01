import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export interface Reminder {
  identifier: string;
  title: string;
  description: string | null;
  recurring: boolean;
  recurrenceRule: string | null;
  alarmSound: string | null;
  remindAt: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateReminderPayload {
  title: string;
  description?: string;
  recurring?: boolean;
  recurrenceRule?: string;
  alarmSound?: string;
  remindAt: string;
}

export interface UpdateReminderPayload {
  title?: string;
  description?: string;
  recurring?: boolean;
  recurrenceRule?: string;
  alarmSound?: string;
  remindAt?: string;
}

export const useReminderStore = defineStore("reminder_store", {
  state: () => ({
    reminders: [] as Reminder[],
    loading: false,
  }),

  getters: {
    upcomingReminders: (state) => {
      const now = new Date().toISOString();
      return state.reminders.filter((r) => r.remindAt > now);
    },
    recurringReminders: (state) => state.reminders.filter((r) => r.recurring),
    oneTimeReminders: (state) => state.reminders.filter((r) => !r.recurring),
  },

  actions: {
    async fetchReminders() {
      this.loading = true;
      try {
        this.reminders = await invoke<Reminder[]>("get_all_reminders");
      } finally {
        this.loading = false;
      }
    },

    async createReminder(payload: CreateReminderPayload): Promise<Reminder> {
      const created = await invoke<Reminder>("create_reminder", {
        reminder: payload,
      });
      this.reminders.unshift(created);
      return created;
    },

    async updateReminder(
      identifier: string,
      payload: UpdateReminderPayload,
    ): Promise<Reminder> {
      const updated = await invoke<Reminder>("update_reminder", {
        identifier,
        reminder: payload,
      });
      const idx = this.reminders.findIndex((r) => r.identifier === identifier);
      if (idx !== -1) this.reminders[idx] = updated;
      return updated;
    },

    async deleteReminder(identifier: string) {
      await invoke("delete_reminder", { identifier });
      this.reminders = this.reminders.filter(
        (r) => r.identifier !== identifier,
      );
    },
  },
});
