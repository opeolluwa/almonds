import type {
  CreateNotification,
  NotificationType,
  Notifications,
} from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateNotification };

export class NotificationRepository extends BaseRepository {
  async create(
    payload: CreateNotification,
    meta?: RequestMeta,
  ): Promise<Notifications> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Notifications>(
      `INSERT INTO notifications
         (identifier, title, body, notification_type, is_read, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title,
        payload.body,
        payload.notificationType,
        payload.isRead ?? false,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Notifications | null> {
    const m = this.requireMeta(meta);
    return this.row<Notifications>(
      `SELECT * FROM notifications WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Notifications[]> {
    const m = this.requireMeta(meta);
    return this.rows<Notifications>(
      `SELECT * FROM notifications WHERE workspace_identifier = $1 ORDER BY created_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async find_by_type(
    notification_type: NotificationType,
    meta?: RequestMeta,
  ): Promise<Notifications[]> {
    const m = this.requireMeta(meta);
    return this.rows<Notifications>(
      `SELECT * FROM notifications WHERE notification_type = $1 AND workspace_identifier = $2 ORDER BY created_at DESC`,
      [notification_type, m.workspaceIdentifier],
    );
  }

  async mark_as_read(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Notifications> {
    const m = this.requireMeta(meta);
    const row = await this.row<Notifications>(
      `UPDATE notifications SET is_read = TRUE, updated_at = $2
       WHERE identifier = $1 AND workspace_identifier = $3
       RETURNING *`,
      [identifier, this.now(), m.workspaceIdentifier],
    );
    if (!row) throw new Error("notification not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    await this.run(
      `DELETE FROM notifications WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }
}
