import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import type { NotificationType, Priority } from "lunar";
import type {
  CreateBookmark,
  CreateNote,
  CreateNotification,
  CreateRecycleBinEntry,
  CreateReminder,
  CreateSnippet,
  CreateTodo,
  CreateUserPreference,
  CreateWorkspace,
  LunarConsoleApi,
  RequestMeta,
  UpdateBookmark,
  UpdateNote,
  UpdateReminder,
  UpdateSnippet,
  UpdateTodo,
  UpdateUserPreference,
  UpdateWorkspace,
} from "~/utils/lunar";

type InvokeArgs = Record<string, unknown>;

type CommandHandler = (
  api: LunarConsoleApi,
  args: InvokeArgs,
) => Promise<unknown>;

const asString = (args: InvokeArgs, key: string): string => args[key] as string;
const asMeta = (args: InvokeArgs): RequestMeta | undefined =>
  args.meta as RequestMeta | undefined;

const duplicate = (
  api: LunarConsoleApi,
  a: InvokeArgs,
  kind: "notes" | "todos" | "bookmarks" | "snippets" | "reminders",
) => {
  const repo = api[kind];
  return repo.duplicate_record(
    asString(a, "recordIdentifier"),
    asString(a, "previousWorkspaceIdentifier"),
    asString(a, "targetWorkspaceIdentifier"),
  );
};

const transfer = (
  api: LunarConsoleApi,
  a: InvokeArgs,
  kind: "notes" | "todos" | "bookmarks" | "snippets" | "reminders",
) => {
  const repo = api[kind];
  return repo.transfer_record(
    asString(a, "recordIdentifier"),
    asString(a, "previousWorkspaceIdentifier"),
    asString(a, "targetWorkspaceIdentifier"),
  );
};

const COMMANDS: Record<string, CommandHandler> = {
  list_workspaces: (api) => api.workspaces.list_workspaces(),
  create_workspace: (api, a) =>
    api.workspaces.create_workspace(a.workspace as CreateWorkspace),
  update_workspace: (api, a) =>
    api.workspaces.update_workspace(
      asString(a, "identifier"),
      a.workspace as UpdateWorkspace,
    ),
  delete_workspace: (api, a) =>
    api.workspaces.delete_workspace(asString(a, "identifier"), asMeta(a)),
  verify_workspace_password: (api, a) =>
    api.workspaces.verify_workspace_password(
      asString(a, "identifier"),
      asString(a, "password"),
    ),

  get_workspace_preference: (api, a) => api.workspacePreferences.get(asMeta(a)),
  create_workspace_preference: (api, a) =>
    api.workspacePreferences.create(
      a.preference as CreateUserPreference,
      asMeta(a),
    ),
  update_workspace_preference: (api, a) =>
    api.workspacePreferences.update(
      asString(a, "identifier"),
      a.preference as UpdateUserPreference,
      asMeta(a),
    ),

  get_all_notes: (api, a) => api.notes.find_all(asMeta(a)),
  get_recently_added_notes: (api, a) => api.notes.recently_added(asMeta(a)),
  create_note: (api, a) => api.notes.create(a.note as CreateNote, asMeta(a)),
  update_note: (api, a) =>
    api.notes.update(
      asString(a, "identifier"),
      a.note as UpdateNote,
      asMeta(a),
    ),
  delete_note: (api, a) =>
    api.notes.delete(asString(a, "identifier"), asMeta(a)),
  duplicate_note: (api, a) => duplicate(api, a, "notes"),
  transfer_note: (api, a) => transfer(api, a, "notes"),

  get_all_todos: (api, a) => api.todos.find_all(asMeta(a)),
  create_todo: (api, a) =>
    api.todos.create_todo(a.todo as CreateTodo, asMeta(a)),
  update_todo: (api, a) =>
    api.todos.update(
      asString(a, "identifier"),
      a.todo as UpdateTodo,
      asMeta(a),
    ),
  mark_todo_done: (api, a) =>
    api.todos.mark_done(
      asString(a, "identifier"),
      a.done as boolean,
      asMeta(a),
    ),
  change_todo_priority: (api, a) =>
    api.todos.change_priority(
      asString(a, "identifier"),
      // Stores send lowercase priorities, matching the DB enum string values
      // (`#[sea_orm(string_value)]`); the generated `Priority` binding exposes
      // the Rust variant casing instead, so cast through unknown.
      a.priority as unknown as Priority,
      asMeta(a),
    ),
  update_todo_due_date: (api, a) =>
    api.todos.update_due_date(
      asString(a, "identifier"),
      a.dueDate as string | null,
      asMeta(a),
    ),
  delete_todo: (api, a) =>
    api.todos.delete(asString(a, "identifier"), asMeta(a)),
  duplicate_todo: (api, a) => duplicate(api, a, "todos"),
  transfer_todo: (api, a) => transfer(api, a, "todos"),

  get_all_bookmarks: (api, a) => api.bookmarks.find_all(asMeta(a)),
  create_bookmark: (api, a) =>
    api.bookmarks.create(a.bookmark as CreateBookmark, asMeta(a)),
  update_bookmark: (api, a) =>
    api.bookmarks.update(
      asString(a, "identifier"),
      a.bookmark as UpdateBookmark,
      asMeta(a),
    ),
  delete_bookmark: (api, a) =>
    api.bookmarks.delete(asString(a, "identifier"), asMeta(a)),
  duplicate_bookmark: (api, a) => duplicate(api, a, "bookmarks"),
  transfer_bookmark: (api, a) => transfer(api, a, "bookmarks"),

  get_all_snippets: (api, a) => api.snippets.find_all(asMeta(a)),
  get_recently_added_snippet: (api, a) =>
    api.snippets.recently_added(asMeta(a)),
  create_snippet: (api, a) =>
    api.snippets.create(a.snippet as CreateSnippet, asMeta(a)),
  update_snippet: (api, a) =>
    api.snippets.update(
      asString(a, "identifier"),
      a.snippet as UpdateSnippet,
      asMeta(a),
    ),
  delete_snippet: (api, a) =>
    api.snippets.delete(asString(a, "identifier"), asMeta(a)),
  duplicate_snippet: (api, a) => duplicate(api, a, "snippets"),
  transfer_snippet: (api, a) => transfer(api, a, "snippets"),

  get_all_reminders: (api, a) => api.reminders.find_all(asMeta(a)),
  create_reminder: (api, a) =>
    api.reminders.create(a.reminder as CreateReminder, asMeta(a)),
  update_reminder: (api, a) =>
    api.reminders.update(
      asString(a, "identifier"),
      a.reminder as UpdateReminder,
      asMeta(a),
    ),
  delete_reminder: (api, a) =>
    api.reminders.delete(asString(a, "identifier"), asMeta(a)),
  duplicate_reminder: (api, a) => duplicate(api, a, "reminders"),
  transfer_reminder: (api, a) => transfer(api, a, "reminders"),

  get_all_notifications: (api, a) => api.notifications.find_all(asMeta(a)),
  get_notifications_by_type: (api, a) =>
    api.notifications.find_by_type(
      a.notificationType as NotificationType,
      asMeta(a),
    ),
  create_notification: (api, a) =>
    api.notifications.create(a.notification as CreateNotification, asMeta(a)),
  mark_notification_as_read: (api, a) =>
    api.notifications.mark_as_read(asString(a, "identifier"), asMeta(a)),
  delete_notification: (api, a) =>
    api.notifications.delete(asString(a, "identifier"), asMeta(a)),

  get_all_recycle_bin_entries: (api, a) => api.recycleBin.find_all(asMeta(a)),
  create_recycle_bin_entry: (api, a) =>
    api.recycleBin.store(a.entry as CreateRecycleBinEntry, asMeta(a)),
  purge_recycle_bin_entry: (api, a) =>
    api.recycleBin.purge(asString(a, "identifier"), asMeta(a)),
  purge_all_recycle_bin_entries: (api, a) =>
    api.recycleBin.purge_all(asMeta(a)),
};

const UNSYNCED = new Set([
  "get_unsynced_workspaces",
  "get_unsynced_workspace_preferences",
  "get_unsynced_notes",
  "get_unsynced_todos",
  "get_unsynced_bookmarks",
  "get_unsynced_snippets",
  "get_unsynced_reminders",
  "get_unsynced_recycle_bin",
]);

const CLEAR_SYNCED = new Set([
  "clear_synced_workspaces",
  "clear_synced_workspace_preferences",
  "clear_synced_notes",
  "clear_synced_todos",
  "clear_synced_bookmarks",
  "clear_synced_snippets",
  "clear_synced_reminders",
  "clear_synced_recycle_bin",
]);

const UNSUPPORTED = new Set([
  "export_notes_as_pdf",
  "list_moodboard_images",
  "save_moodboard_image",
  "delete_moodboard_image",
  "set_alarm_settings",
  "list_alarm_sounds",
  "play_alarm_sound",
  "stop_alarm_sound",
  "is_ollama_installed",
  "check_ai_model",
  "pull_ai_model",
  "generate_stream",
]);

async function dispatchBrowser(
  cmd: string,
  args: InvokeArgs,
): Promise<unknown> {
  const api = window.lunar;
  if (!api) {
    throw new Error(
      `[invoke] "${cmd}" — the in-browser lunar data layer is not initialised`,
    );
  }

  const handler = COMMANDS[cmd];
  if (handler) return handler(api, args);

  if (UNSYNCED.has(cmd)) return [];

  if (CLEAR_SYNCED.has(cmd)) return undefined;

  if (UNSUPPORTED.has(cmd)) {
    console.warn(
      `[invoke] "${cmd}" is not available in the browser — returning a no-op`,
    );
    return cmd === "list_moodboard_images" ? [] : undefined;
  }

  throw new Error(`[invoke] unknown command "${cmd}" in the browser`);
}

/**
 * Drop-in replacement for `invoke` from `@tauri-apps/api/core` that talks to
 * the native Rust backend over IPC in the Tauri webview, and to the in-browser
 * PGlite layer (`window.lunar`, installed by plugins/lunar.client.ts) in a
 * plain browser.
 */
export async function invoke<T>(cmd: string, args?: InvokeArgs): Promise<T> {
  if (typeof window !== "undefined" && window.lunar) {
    return (await dispatchBrowser(cmd, args ?? {})) as T;
  }
  return tauriInvoke<T>(cmd, args);
}
