import type {
  CreateUserPreference,
  UpdateUserPreference,
  WorkspacePreferences,
} from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateUserPreference, UpdateUserPreference };

const COLUMNS = [
  "first_name",
  "last_name",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class WorkspacePreferenceRepository extends BaseRepository {
  async create(
    payload: CreateUserPreference,
    meta?: RequestMeta,
  ): Promise<WorkspacePreferences> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<WorkspacePreferences>(
      `INSERT INTO workspace_preferences
         (identifier, first_name, last_name, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      [
        this.newUuid(),
        payload.firstName,
        payload.lastName,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async get(meta?: RequestMeta): Promise<WorkspacePreferences | null> {
    const m = this.requireMeta(meta);
    return this.row<WorkspacePreferences>(
      `SELECT * FROM workspace_preferences WHERE workspace_identifier = $1`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateUserPreference,
    meta?: RequestMeta,
  ): Promise<WorkspacePreferences> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.firstName !== undefined) {
      sets.push(`first_name = $${idx++}`);
      params.push(payload.firstName);
    }
    if (payload.lastName !== undefined) {
      sets.push(`last_name = $${idx++}`);
      params.push(payload.lastName);
    }

    const row = await this.row<WorkspacePreferences>(
      `UPDATE workspace_preferences SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("workspace preference not found");
    return row;
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "workspace_preferences",
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
      "workspace_preferences",
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
      "workspace_preferences",
      record_identifier,
      workspace_identifier,
    );
  }
}
