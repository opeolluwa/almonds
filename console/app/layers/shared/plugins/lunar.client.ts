// Exposes the in-browser lunar data layer on `window.lunar` when running in a
// plain browser (web deployment). The Tauri build talks to the native SQLite
// backend through IPC instead, so this is skipped when Tauri internals are
// present.
import { IS_WEB } from "../env";
import { createLunarConsoleApi } from "../utils/lunar";

export default defineNuxtPlugin(async () => {
  if (!IS_WEB) return;

  try {
    const api = await createLunarConsoleApi();
    window.lunar = api;

    const workspaces = await api.workspaces.list_workspaces();
    const defaultWorkspace =
      workspaces.find((w) => w.isDefault) ?? workspaces[0] ?? null;

    console.info(
      "[lunar] ready — window.lunar exposes the in-browser data layer (PGlite)",
    );
    if (defaultWorkspace) {
      console.info(
        `[lunar] default workspace: "${defaultWorkspace.name}" (${defaultWorkspace.identifier}) — pass it as meta.workspaceIdentifier`,
      );
    }
  } catch (error) {
    console.error("[lunar] failed to initialize in-browser data layer", error);
  }
});
