import { invoke } from "@tauri-apps/api/core";
import { useAlarmSettings } from "~/composables/useAlarmSettings";

/**
 * Syncs alarm settings to the Rust backend scheduler.
 * The actual cron loop, DB queries, notifications, and sound playback all run
 * in Rust — this composable just keeps the backend in sync with the user's
 * preferred lead time and default sound.
 */
export function useAlarmScheduler() {
  const { settings } = useAlarmSettings();

  async function syncSettings() {
    try {
      await invoke("set_alarm_settings", {
        leadTimeMinutes: settings.value.leadTimeMinutes,
        defaultSound: settings.value.defaultSound,
      });
    } catch (e) {
      console.error("[AlarmScheduler] Failed to sync settings:", e);
    }
  }

  onMounted(syncSettings);
  watch(settings, syncSettings, { deep: true });
}
