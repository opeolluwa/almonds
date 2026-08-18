import type { CreateWorkspace, UpdateWorkspace, Workspaces } from "lunar";
import { BaseRepository, type RequestMeta } from "../base";

export type { CreateWorkspace, UpdateWorkspace };

/**
 * NOTE: the Rust backend hashes workspace passwords with Argon2id. The browser
 * facade stores a SHA-256 digest instead so `verify_workspace_password` can
 * work without shipping a wasm argon2 build — the stored value is not portable
 * to the native database.
 */
export class WorkspaceRepository extends BaseRepository {
  async create_workspace(payload: CreateWorkspace): Promise<Workspaces> {
    const now = this.now();
    return this.mustRow<Workspaces>(
      `INSERT INTO workspaces
         (identifier, name, description, is_default, is_hidden, is_secured, password_hash, created_at, updated_at)
       VALUES ($1, $2, $3, FALSE, FALSE, FALSE, NULL, $4, $5)
       RETURNING *`,
      [this.newUuid(), payload.name, payload.description, now, now],
    );
  }

  async get_workspace_by_id(identifier: string): Promise<Workspaces> {
    const row = await this.row<Workspaces>(
      `SELECT * FROM workspaces WHERE identifier = $1`,
      [identifier],
    );
    if (!row) throw new Error(`Workspace with id ${identifier} not found`);
    return row;
  }

  async list_workspaces(): Promise<Workspaces[]> {
    return this.rows<Workspaces>(
      `SELECT * FROM workspaces ORDER BY is_default DESC, name`,
    );
  }

  async exists(identifier: string): Promise<boolean> {
    return this.bool(
      "SELECT EXISTS (SELECT 1 FROM workspaces WHERE identifier = $1) AS ok",
      [identifier],
    );
  }

  async delete_workspace(
    identifier: string,
    meta?: RequestMeta,
  ): Promise<void> {
    this.requireMeta(meta);
    const model = await this.get_workspace_by_id(identifier);
    if (model.isDefault) {
      throw new Error("Cannot delete the default workspace");
    }
    await this.run(`DELETE FROM workspaces WHERE identifier = $1`, [
      identifier,
    ]);
  }

  async update_workspace(
    identifier: string,
    payload: UpdateWorkspace,
  ): Promise<Workspaces> {
    if (payload.isDefault === true) {
      await this.run(
        `UPDATE workspaces SET is_default = FALSE WHERE identifier != $1`,
        [identifier],
      );
    }

    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.name !== undefined) {
      sets.push(`name = $${idx++}`);
      params.push(payload.name);
    }
    if (payload.description !== undefined) {
      sets.push(`description = $${idx++}`);
      params.push(payload.description);
    }
    if (payload.isDefault !== undefined) {
      sets.push(`is_default = $${idx++}`);
      params.push(payload.isDefault);
    }
    if (payload.isHidden !== undefined) {
      sets.push(`is_hidden = $${idx++}`);
      params.push(payload.isHidden);
    }
    if (payload.isSecured !== undefined) {
      sets.push(`is_secured = $${idx++}`);
      params.push(payload.isSecured);
      if (!payload.isSecured) {
        sets.push(`password_hash = $${idx++}`);
        params.push(null);
      }
    }
    if (payload.password !== undefined) {
      if (payload.password === "") {
        sets.push(`password_hash = $${idx++}`);
        params.push(null);
      } else {
        sets.push(`password_hash = $${idx++}`);
        params.push(await this.sha256(payload.password));
      }
    }

    const row = await this.row<Workspaces>(
      `UPDATE workspaces SET ${sets.join(", ")} WHERE identifier = $1
       RETURNING *`,
      params,
    );
    if (!row) throw new Error(`Workspace with id ${identifier} not found`);
    return row;
  }

  async verify_workspace_password(
    identifier: string,
    password: string,
  ): Promise<boolean> {
    const model = await this.get_workspace_by_id(identifier);
    if (!model.isSecured) return true;
    if (!model.passwordHash) return false;
    return (await this.sha256(password)) === model.passwordHash;
  }

  private async sha256(input: string): Promise<string> {
    const data = new TextEncoder().encode(input);
    const digest = await crypto.subtle.digest("SHA-256", data);
    return Array.from(new Uint8Array(digest))
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  }
}
