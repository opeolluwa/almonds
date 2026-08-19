import type { PGlite } from "@electric-sql/pglite";
import type { RequestMeta } from "lunar";
import { lunarDb } from "./pglite";
import { v4 as uuidv4 } from "uuid";
export type { RequestMeta };

interface DbRow {
  [column: string]: unknown;
}

export function camelize(key: string): string {
  return key.replace(/_([a-z])/g, (_match, letter: string) =>
    letter.toUpperCase(),
  );
}

function normalize(value: unknown): unknown {
  if (value instanceof Date) return value.toISOString();
  return value;
}

function toCamelRow(row: DbRow): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(row)) {
    let normalized = normalize(value);
    if (
      typeof normalized === "string" &&
      (key === "categories" || key === "payload" || key === "description")
    ) {
      try {
        normalized = JSON.parse(normalized);
      } catch {
        // keep the raw string when it is not JSON
      }
    }
    out[camelize(key)] = normalized;
  }
  return out;
}

/**
 * Base class for the PGlite-backed repositories. Exposes a facade that
 * mirrors the wasm-bindgen API in lunar/pkg/lunar.d.ts (snake_case entities
 * are surfaced as camelCase rows, matching the serde rename_all on the Rust
 * models), but executes real SQL against the in-browser Postgres.
 */
export abstract class BaseRepository {
  protected async db(): Promise<PGlite> {
    return lunarDb();
  }

  protected requireMeta(meta?: RequestMeta): RequestMeta {
    if (!meta?.workspaceIdentifier) {
      throw new Error("workspace identifier is required");
    }
    return meta;
  }

  protected async rows<T>(sql: string, params: unknown[] = []): Promise<T[]> {
    const db = await this.db();
    const result = await db.query(sql, params);
    return (result.rows ?? []).map((row) => toCamelRow(row as DbRow)) as T[];
  }

  protected async row<T>(
    sql: string,
    params: unknown[] = [],
  ): Promise<T | null> {
    const rows = await this.rows<T>(sql, params);
    return rows[0] ?? null;
  }

  protected async mustRow<T>(sql: string, params: unknown[] = []): Promise<T> {
    const row = await this.row<T>(sql, params);
    if (!row) throw new Error("expected a row to be returned");
    return row;
  }

  protected async run(sql: string, params: unknown[] = []): Promise<void> {
    const db = await this.db();
    await db.query(sql, params);
  }

  protected async bool(sql: string, params: unknown[] = []): Promise<boolean> {
    const row = await this.row<{ ok: boolean }>(sql, params);
    return row?.ok ?? false;
  }

  protected async workspaceExists(identifier: string): Promise<boolean> {
    return this.bool(
      "SELECT EXISTS (SELECT 1 FROM workspaces WHERE identifier = $1) AS ok",
      [identifier],
    );
  }

  protected async requireWorkspace(identifier: string): Promise<void> {
    if (!(await this.workspaceExists(identifier))) {
      throw new Error(`workspace not found: ${identifier}`);
    }
  }

  protected async recordExistsInWorkspace(
    table: string,
    recordIdentifier: string,
    workspaceIdentifier: string,
  ): Promise<boolean> {
    return this.bool(
      `SELECT EXISTS (SELECT 1 FROM "${table}" WHERE identifier = $1 AND workspace_identifier = $2) AS ok`,
      [recordIdentifier, workspaceIdentifier],
    );
  }

  protected async transferWorkspaceRecord(
    table: string,
    recordIdentifier: string,
    previousWorkspaceIdentifier: string,
    targetWorkspaceIdentifier: string,
  ): Promise<void> {
    await this.requireWorkspace(previousWorkspaceIdentifier);
    await this.requireWorkspace(targetWorkspaceIdentifier);
    if (
      !(await this.recordExistsInWorkspace(
        table,
        recordIdentifier,
        previousWorkspaceIdentifier,
      ))
    ) {
      throw new Error("record not found");
    }
    await this.run(
      `UPDATE "${table}" SET workspace_identifier = $1, updated_at = $2 WHERE identifier = $3`,
      [targetWorkspaceIdentifier, this.now(), recordIdentifier],
    );
  }

  protected async duplicateWorkspaceRecord(
    table: string,
    recordIdentifier: string,
    previousWorkspaceIdentifier: string,
    targetWorkspaceIdentifier: string,
    columns: string[],
  ): Promise<void> {
    await this.requireWorkspace(previousWorkspaceIdentifier);
    await this.requireWorkspace(targetWorkspaceIdentifier);
    const source = await this.row<Record<string, unknown>>(
      `SELECT * FROM "${table}" WHERE identifier = $1 AND workspace_identifier = $2`,
      [recordIdentifier, previousWorkspaceIdentifier],
    );
    if (!source) throw new Error("record not found");

    const values = columns.map((column) => {
      if (column === "workspace_identifier") return targetWorkspaceIdentifier;
      if (column === "created_at" || column === "updated_at") return this.now();
      return source[camelize(column)] ?? null;
    });

    const placeholders = columns.map((_, i) => `$${i + 1}`).join(", ");
    await this.run(
      `INSERT INTO "${table}" (identifier, ${columns.join(", ")})
       VALUES ($${columns.length + 1}, ${placeholders})`,
      [...values, this.newUuid()],
    );
  }

  protected newUuid(): string {
    return uuidv4();
  }

  protected now(): string {
    return new Date().toISOString();
  }
}
