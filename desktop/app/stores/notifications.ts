import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import type { NotificationType } from "almond_kernel";

export interface Notification {
  identifier: string;
  title: string;
  body: string;
  notificationType: NotificationType;
  isRead: boolean;
  createdAt: string;
  updatedAt: string;
  workspaceIdentifier: string | null;
}

export interface CreateNotificationPayload {
  title: string;
  body: string;
  notificationType: NotificationType;
  workspaceIdentifier?: string | null;
  isRead?: boolean;
}

export const useNotificationStore = defineStore("notification_store", {
  state: () => ({
    notifications: [] as Notification[],
    loading: false,
  }),

  getters: {
    unread: (state) => state.notifications.filter((n) => !n.isRead),
    unreadCount: (state) => state.notifications.filter((n) => !n.isRead).length,
    byType: (state) => (type: NotificationType) =>
      state.notifications.filter((n) => n.notificationType === type),
  },

  actions: {
    async fetchNotifications() {
      this.loading = true;
      try {
        this.notifications = await invoke<Notification[]>(
          "get_all_notifications",
          { meta: await getWorkspaceMeta() },
        );
      } finally {
        this.loading = false;
      }
    },

    async fetchByType(notificationType: NotificationType) {
      return invoke<Notification[]>("get_notifications_by_type", {
        notificationType,
        meta: await getWorkspaceMeta(),
      });
    },

    async createNotification(
      payload: CreateNotificationPayload,
    ): Promise<Notification> {
      const created = await invoke<Notification>("create_notification", {
        notification: {
          title: payload.title,
          body: payload.body,
          notificationType: payload.notificationType,
          workspaceIdentifier: payload.workspaceIdentifier ?? null,
          isRead: payload.isRead ?? false,
        },
        meta: await getWorkspaceMeta(),
      });

      this.notifications.unshift(created);

      return created;
    },

    async markAsRead(identifier: string): Promise<Notification> {
      const updated = await invoke<Notification>("mark_notification_as_read", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      const idx = this.notifications.findIndex(
        (n) => n.identifier === identifier,
      );
      if (idx !== -1) this.notifications[idx] = updated;

      return updated;
    },

    async markAllAsRead() {
      const unread = this.notifications.filter((n) => !n.isRead);
      await Promise.all(unread.map((n) => this.markAsRead(n.identifier)));
    },

    async deleteNotification(identifier: string) {
      await invoke("delete_notification", {
        identifier,
        meta: await getWorkspaceMeta(),
      });

      this.notifications = this.notifications.filter(
        (n) => n.identifier !== identifier,
      );
    },
  },
});
