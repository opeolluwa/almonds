import type { CreateNote, Notes, UpdateNote } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateNote, UpdateNote };

const COLUMNS = [
  "title",
  "content",
  "categories",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class NotesRepository extends BaseRepository {
  async create(payload: CreateNote, meta?: RequestMeta): Promise<Notes> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Notes>(
      `INSERT INTO notes
         (identifier, title, content, categories, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title,
        payload.content,
        JSON.stringify(payload.categories ?? []),
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Notes | null> {
    const m = this.requireMeta(meta);
    return this.row<Notes>(
      `SELECT * FROM notes WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Notes[]> {
    const m = this.requireMeta(meta);
    return this.rows<Notes>(
      `SELECT * FROM notes WHERE workspace_identifier = $1 ORDER BY created_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const model = await this.find_by_id(identifier, m);
    if (!model) throw new Error("note not found");

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, 'note', $3, $4, $5)`,
      [
        this.newUuid(),
        identifier,
        JSON.stringify(model),
        this.now(),
        m.workspaceIdentifier,
      ],
    );

    await this.run(
      `DELETE FROM notes WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async recently_added(meta?: RequestMeta): Promise<Notes[]> {
    const m = this.requireMeta(meta);
    return this.rows<Notes>(
      `SELECT * FROM notes WHERE workspace_identifier = $1
       ORDER BY created_at ASC LIMIT 10`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateNote,
    meta?: RequestMeta,
  ): Promise<Notes> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.content !== undefined) {
      sets.push(`content = $${idx++}`);
      params.push(payload.content);
    }
    if (payload.categories !== undefined) {
      sets.push(`categories = $${idx++}`);
      params.push(JSON.stringify(payload.categories));
    }

    const row = await this.row<Notes>(
      `UPDATE notes SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("note not found");
    return row;
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "notes",
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
      "notes",
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
      "notes",
      record_identifier,
      workspace_identifier,
    );
  }
}
