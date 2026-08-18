import type { CreateReminder, Reminder, UpdateReminder } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateReminder, UpdateReminder };

const COLUMNS = [
  "title",
  "description",
  "recurring",
  "recurrence_rule",
  "alarm_sound",
  "remind_at",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class ReminderRepository extends BaseRepository {
  async create(payload: CreateReminder, meta?: RequestMeta): Promise<Reminder> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Reminder>(
      `INSERT INTO reminder
         (identifier, title, description, recurring, recurrence_rule, alarm_sound, remind_at, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title,
        payload.description ?? null,
        payload.recurring ?? false,
        payload.recurrenceRule ?? null,
        payload.alarmSound ?? null,
        payload.remindAt,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Reminder | null> {
    const m = this.requireMeta(meta);
    return this.row<Reminder>(
      `SELECT * FROM reminder WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Reminder[]> {
    const m = this.requireMeta(meta);
    return this.rows<Reminder>(
      `SELECT * FROM reminder WHERE workspace_identifier = $1 ORDER BY created_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateReminder,
    meta?: RequestMeta,
  ): Promise<Reminder> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.description !== undefined) {
      sets.push(`description = $${idx++}`);
      params.push(payload.description);
    }
    if (payload.recurring !== undefined) {
      sets.push(`recurring = $${idx++}`);
      params.push(payload.recurring);
    }
    if (payload.recurrenceRule !== undefined) {
      sets.push(`recurrence_rule = $${idx++}`);
      params.push(payload.recurrenceRule);
    }
    if (payload.alarmSound !== undefined) {
      sets.push(`alarm_sound = $${idx++}`);
      params.push(payload.alarmSound);
    }
    if (payload.remindAt !== undefined) {
      sets.push(`remind_at = $${idx++}`);
      params.push(payload.remindAt);
    }

    const row = await this.row<Reminder>(
      `UPDATE reminder SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("reminder not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const model = await this.find_by_id(identifier, m);
    if (!model) throw new Error("reminder not found");

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, 'reminder', $3, $4, $5)`,
      [
        this.newUuid(),
        identifier,
        JSON.stringify(model),
        this.now(),
        m.workspaceIdentifier,
      ],
    );

    await this.run(
      `DELETE FROM reminder WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "reminder",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
    );
  }

  async duplicate_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.duplicateWorkspaceRecord(
      "reminder",
      record_identifier,
      previous_workspace_identifier,
      target_workspace_identifier,
      COLUMNS,
    );
  }

  async record_exists_in_workspace(
    record_identifier: string,
    workspace_identifier: string,
  ): Promise<boolean> {
    return this.recordExistsInWorkspace(
      "reminder",
      record_identifier,
      workspace_identifier,
    );
  }
}
