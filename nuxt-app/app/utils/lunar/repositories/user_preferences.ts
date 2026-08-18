import type {
  CreateUserPreferences,
  UpdateUserPreferences,
  UserPreferences,
} from "lunar";
import { BaseRepository } from "../base";

export type { CreateUserPreferences, UpdateUserPreferences };

export class UserPreferencesRepository extends BaseRepository {
  async create(payload: CreateUserPreferences): Promise<UserPreferences> {
    const now = this.now();
    return this.mustRow<UserPreferences>(
      `INSERT INTO user_preferences
         (identifier, master_first_name, master_last_name, master_email, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      [
        this.newUuid(),
        payload.masterFirstName,
        payload.masterLastName,
        payload.masterEmail,
        now,
        now,
      ],
    );
  }

  async get_by_identifier(identifier: string): Promise<UserPreferences> {
    const row = await this.row<UserPreferences>(
      `SELECT * FROM user_preferences WHERE identifier = $1`,
      [identifier],
    );
    if (!row)
      throw new Error(`user preferences with id ${identifier} not found`);
    return row;
  }

  async update(
    identifier: string,
    payload: UpdateUserPreferences,
  ): Promise<UserPreferences> {
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.masterFirstName !== undefined) {
      sets.push(`master_first_name = $${idx++}`);
      params.push(payload.masterFirstName);
    }
    if (payload.masterLastName !== undefined) {
      sets.push(`master_last_name = $${idx++}`);
      params.push(payload.masterLastName);
    }
    if (payload.masterEmail !== undefined) {
      sets.push(`master_email = $${idx++}`);
      params.push(payload.masterEmail);
    }

    const row = await this.row<UserPreferences>(
      `UPDATE user_preferences SET ${sets.join(", ")} WHERE identifier = $1
       RETURNING *`,
      params,
    );
    if (!row)
      throw new Error(`user preferences with id ${identifier} not found`);
    return row;
  }
}
