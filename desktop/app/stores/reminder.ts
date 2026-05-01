import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

type SyncResult = {
  success: boolean;
  error_message: string | null;
  identifier: string;
};

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
        this.reminders = await invoke<Reminder[]>("get_all_reminders", {
          meta: await getWorkspaceMeta(),
        });
      } finally {
        this.loading = false;
      }
    },

    async createReminder(payload: CreateReminderPayload): Promise<Reminder> {
      const created = await invoke<Reminder>("create_reminder", {
        reminder: payload,
        meta: await getWorkspaceMeta(),
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
        meta: await getWorkspaceMeta(),
      });

      const idx = this.reminders.findIndex((r) => r.identifier === identifier);
      if (idx !== -1) this.reminders[idx] = updated;

      return updated;
    },

    async deleteReminder(identifier: string) {
      await invoke("delete_reminder", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.reminders = this.reminders.filter(
        (r) => r.identifier !== identifier,
      );
    },

    async duplicateReminder(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("duplicate_reminder", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      await this.fetchReminders();
    },

    async transferReminder(
      recordIdentifier: string,
      previousWorkspaceIdentifier: string,
      targetWorkspaceIdentifier: string,
    ) {
      await invoke("transfer_reminder", {
        recordIdentifier,
        previousWorkspaceIdentifier,
        targetWorkspaceIdentifier,
        meta: await getWorkspaceMeta(),
      });

      this.reminders = this.reminders.filter(
        (r) => r.identifier !== recordIdentifier,
      );
    },

    async fetchUnsynced() {
      try {
        const reminders = await invoke<Reminder[]>("get_unsynced_reminders");
        console.log(
          "Unsynced reminders fetched:",
          JSON.stringify(reminders, null, 2),
        );
        return reminders;
      } catch (error) {
        console.error("Error fetching unsynced reminders:", error);
        return [];
      }
    },

    async syncUpstream() {
      const reminders = await this.fetchUnsynced();
      if (!reminders.length) return;

      const input = reminders.map((r) => ({
        identifier: r.identifier,
        title: r.title,
        description: r.description ?? null,
        recurring: r.recurring,
        recurrence_rule: r.recurrenceRule ?? null,
        alarm_sound: r.alarmSound ?? null,
        remind_at: r.remindAt,
        created_at: r.createdAt,
        updated_at: r.updatedAt,
        workspace_identifier: (r as any).workspaceIdentifier ?? null,
      }));
      const query = gql`
        mutation SyncReminders($input: [SyncReminderInput!]!) {
          sync_reminder(input: $input) {
            success
            error_message
            identifier
          }
        }
      `;

      const { mutate } = useMutation(query, { variables: { input } });

      try {
        const data = await mutate();
        console.log("Reminders sync response:", data);
      } catch (error) {
        console.error("Error syncing reminders:", error);
      }
    },

    async clearQueue(identifiers: string[]) {
      await invoke("clear_synced_reminders", { identifiers });
    },
  },
});
