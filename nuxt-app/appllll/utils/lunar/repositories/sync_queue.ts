import type { SyncQueue } from "lunar";
import { BaseRepository } from "../base";

/** Read access to the sync_queue table (filled by the migration triggers). */
export class SyncQueueRepository extends BaseRepository {
  async list(limit = 50): Promise<SyncQueue[]> {
    return this.rows<SyncQueue>(
      `SELECT * FROM sync_queue ORDER BY created_at DESC, identifier DESC LIMIT $1`,
      [limit],
    );
  }

  async clear(): Promise<void> {
    await this.run(`DELETE FROM sync_queue`);
  }
}
