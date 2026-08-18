import type { PGlite } from "@electric-sql/pglite";
import { lunarDb } from "./pglite";
import { NotesRepository } from "./repositories/notes";
import { TodoRepository } from "./repositories/todo";
import { BookmarkRepository } from "./repositories/bookmarks";
import { SnippetRepository } from "./repositories/snippets";
import { ReminderRepository } from "./repositories/reminder";
import { NotificationRepository } from "./repositories/notifications";
import { RecycleBinRepository } from "./repositories/recycle_bin";
import { WorkspaceRepository } from "./repositories/workspace";
import { WorkspacePreferenceRepository } from "./repositories/workspace_preferences";
import { UserPreferencesRepository } from "./repositories/user_preferences";
import { SyncQueueRepository } from "./repositories/sync_queue";

export interface LunarConsoleApi {
  db: PGlite;
  notes: NotesRepository;
  todos: TodoRepository;
  bookmarks: BookmarkRepository;
  snippets: SnippetRepository;
  reminders: ReminderRepository;
  notifications: NotificationRepository;
  recycleBin: RecycleBinRepository;
  workspaces: WorkspaceRepository;
  workspacePreferences: WorkspacePreferenceRepository;
  userPreferences: UserPreferencesRepository;
  syncQueue: SyncQueueRepository;
}

/** Builds the in-browser lunar API, applying pending migrations on first use. */
export async function createLunarConsoleApi(): Promise<LunarConsoleApi> {
  const db = await lunarDb();
  return {
    db,
    notes: new NotesRepository(),
    todos: new TodoRepository(),
    bookmarks: new BookmarkRepository(),
    snippets: new SnippetRepository(),
    reminders: new ReminderRepository(),
    notifications: new NotificationRepository(),
    recycleBin: new RecycleBinRepository(),
    workspaces: new WorkspaceRepository(),
    workspacePreferences: new WorkspacePreferenceRepository(),
    userPreferences: new UserPreferencesRepository(),
    syncQueue: new SyncQueueRepository(),
  };
}

export type { RequestMeta } from "./base";
export type { CreateNote, UpdateNote } from "./repositories/notes";
export type { CreateTodo, UpdateTodo } from "./repositories/todo";
export type { CreateBookmark, UpdateBookmark } from "./repositories/bookmarks";
export type { CreateSnippet, UpdateSnippet } from "./repositories/snippets";
export type { CreateReminder, UpdateReminder } from "./repositories/reminder";
export type { CreateNotification } from "./repositories/notifications";
export type { CreateRecycleBinEntry } from "./repositories/recycle_bin";
export type {
  CreateWorkspace,
  UpdateWorkspace,
} from "./repositories/workspace";
export type {
  CreateUserPreference,
  UpdateUserPreference,
} from "./repositories/workspace_preferences";
export type {
  CreateUserPreferences,
  UpdateUserPreferences,
} from "./repositories/user_preferences";
