import type { CreateRecycleBinEntry, ItemType, RecycleBin } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateRecycleBinEntry };

export class RecycleBinRepository extends BaseRepository {
  async store(
    payload: CreateRecycleBinEntry,
    meta?: RequestMeta,
  ): Promise<RecycleBin> {
    const m = this.requireMeta(meta);
    return this.mustRow<RecycleBin>(
      `INSERT INTO recycle_bin
         (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      [
        this.newUuid(),
        payload.itemId,
        payload.itemType,
        payload.payload,
        this.now(),
        m.workspaceIdentifier,
      ],
    );
  }

  async find_by_id(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<RecycleBin | null> {
    const m = this.requireMeta(meta);
    return this.row<RecycleBin>(
      `SELECT * FROM recycle_bin WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async find_all(meta?: RequestMeta): Promise<RecycleBin[]> {
    const m = this.requireMeta(meta);
    return this.rows<RecycleBin>(
      `SELECT * FROM recycle_bin WHERE workspace_identifier = $1 ORDER BY deleted_at DESC`,
      [m.workspaceIdentifier],
    );
  }

  async find_by_item_type(
    item_type: ItemType,
    meta?: RequestMeta,
  ): Promise<RecycleBin[]> {
    const m = this.requireMeta(meta);
    return this.rows<RecycleBin>(
      `SELECT * FROM recycle_bin WHERE item_type = $1 AND workspace_identifier = $2 ORDER BY deleted_at DESC`,
      [item_type, m.workspaceIdentifier],
    );
  }

  async purge(identifier: string, meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    await this.run(
      `DELETE FROM recycle_bin WHERE identifier = $1 AND workspace_identifier = $2`,
      [identifier, m.workspaceIdentifier],
    );
  }

  async purge_all(meta?: RequestMeta): Promise<void> {
    const m = this.requireMeta(meta);
    await this.run(`DELETE FROM recycle_bin WHERE workspace_identifier = $1`, [
      m.workspaceIdentifier,
    ]);
  }
}
