import type { CreateSnippet, Snippets, UpdateSnippet } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateSnippet, UpdateSnippet };

const COLUMNS = [
  "title",
  "language",
  "code",
  "description",
  "is_pinned",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class SnippetRepository extends BaseRepository {
  async create(payload: CreateSnippet, meta?: RequestMeta): Promise<Snippets> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Snippets>(
      `INSERT INTO snippets
         (identifier, title, language, code, description, is_pinned, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title ?? null,
        payload.language ?? null,
        payload.code,
        payload.description ?? null,
        payload.isPinned ?? false,
        payload.createdAt ?? now,
        payload.updatedAt ?? now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Snippets | null> {
    const m = this.requireMeta(meta);
    return this.row<Snippets>(
      `SELECT * FROM snippets WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Snippets[]> {
    const m = this.requireMeta(meta);
    return this.rows<Snippets>(
      `SELECT * FROM snippets WHERE workspace_identifier = $1 ORDER BY created_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async recently_added(meta?: RequestMeta): Promise<Snippets[]> {
    const m = this.requireMeta(meta);
    return this.rows<Snippets>(
      `SELECT * FROM snippets WHERE workspace_identifier = $1
       ORDER BY created_at ASC LIMIT 10`,
      [m.workspaceIdentifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateSnippet,
    meta?: RequestMeta,
  ): Promise<Snippets> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.language !== undefined) {
      sets.push(`language = $${idx++}`);
      params.push(payload.language);
    }
    if (payload.code !== undefined) {
      sets.push(`code = $${idx++}`);
      params.push(payload.code);
    }
    if (payload.description !== undefined) {
      sets.push(`description = $${idx++}`);
      params.push(payload.description);
    }
    if (payload.isPinned !== undefined) {
      sets.push(`is_pinned = $${idx++}`);
      params.push(payload.isPinned);
    }

    const row = await this.row<Snippets>(
      `UPDATE snippets SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("snippet not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const model = await this.find_by_id(identifier, m);
    if (!model) throw new Error("snippet not found");

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, 'snippet', $3, $4, $5)`,
      [
        this.newUuid(),
        identifier,
        JSON.stringify(model),
        this.now(),
        m.workspaceIdentifier,
      ],
    );

    await this.run(
      `DELETE FROM snippets WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "snippets",
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
      "snippets",
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
      "snippets",
      record_identifier,
      workspace_identifier,
    );
  }
}
