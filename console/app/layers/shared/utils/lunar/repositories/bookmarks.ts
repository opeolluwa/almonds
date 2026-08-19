import type { Bookmark, CreateBookmark, Tag, UpdateBookmark } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateBookmark, UpdateBookmark };

const COLUMNS = [
  "title",
  "url",
  "tag",
  "workspace_identifier",
  "created_at",
  "updated_at",
];

export class BookmarkRepository extends BaseRepository {
  async create(payload: CreateBookmark, meta?: RequestMeta): Promise<Bookmark> {
    const m = this.requireMeta(meta);
    const now = this.now();
    return this.mustRow<Bookmark>(
      `INSERT INTO bookmark
         (identifier, title, url, tag, created_at, updated_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [
        this.newUuid(),
        payload.title,
        payload.url,
        payload.tag,
        now,
        now,
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<Bookmark | null> {
    const m = this.requireMeta(meta);
    return this.row<Bookmark>(
      `SELECT * FROM bookmark WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<Bookmark[]> {
    const m = this.requireMeta(meta);
    return this.rows<Bookmark>(
      `SELECT * FROM bookmark WHERE workspace_identifier = $1 ORDER BY created_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async find_by_tag(tag: Tag, meta?: RequestMeta): Promise<Bookmark[]> {
    const m = this.requireMeta(meta);
    return this.rows<Bookmark>(
      `SELECT * FROM bookmark WHERE tag = $1 AND workspace_identifier = $2 ORDER BY created_at DESC`,
      [tag, m.workspaceIdentifier],
    );
  }

  async recently_added(meta?: RequestMeta): Promise<Bookmark[]> {
    const m = this.requireMeta(meta);
    return this.rows<Bookmark>(
      `SELECT * FROM bookmark WHERE workspace_identifier = $1
       ORDER BY created_at ASC LIMIT 10`,
      [m.workspaceIdentifier],
    );
  }

  async exists(identifier: string): Promise<boolean> {
    return this.bool(
      "SELECT EXISTS (SELECT 1 FROM bookmark WHERE identifier = $1) AS ok",
      [identifier],
    );
  }

  async update(
    identifier: string,
    payload: UpdateBookmark,
    meta?: RequestMeta,
  ): Promise<Bookmark> {
    const m = this.requireMeta(meta);
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.title !== undefined) {
      sets.push(`title = $${idx++}`);
      params.push(payload.title);
    }
    if (payload.url !== undefined) {
      sets.push(`url = $${idx++}`);
      params.push(payload.url);
    }
    if (payload.tag !== undefined) {
      sets.push(`tag = $${idx++}`);
      params.push(payload.tag);
    }

    const row = await this.row<Bookmark>(
      `UPDATE bookmark SET ${sets.join(", ")} WHERE identifier = $1 AND workspace_identifier = $${idx}
       RETURNING *`,
      [...params, m.workspaceIdentifier],
    );
    if (!row) throw new Error("bookmark not found");
    return row;
  }

  async delete(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    const model = await this.find_by_id(identifier, m);
    if (!model) throw new Error("bookmark not found");

    await this.run(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, 'bookmark', $3, $4, $5)`,
      [
        this.newUuid(),
        identifier,
        JSON.stringify(model),
        this.now(),
        m.workspaceIdentifier,
      ],
    );

    await this.run(
      `DELETE FROM bookmark WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async transfer_record(
    record_identifier: string,
    previous_workspace_identifier: string,
    target_workspace_identifier: string,
  ): Promise<void> {
    await this.transferWorkspaceRecord(
      "bookmark",
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
      "bookmark",
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
      "bookmark",
      record_identifier,
      workspace_identifier,
    );
  }
}
