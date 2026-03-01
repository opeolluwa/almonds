const STORAGE_KEY = "almonds:alarm-settings";

export interface AlarmSettings {
  defaultSound: string | null;
  leadTimeMinutes: number;
  snoozeDurationMinutes: number;
  repeatAlarm: boolean;
}

const DEFAULT_SETTINGS: AlarmSettings = {
  defaultSound: null,
  leadTimeMinutes: 5,
  snoozeDurationMinutes: 5,
  repeatAlarm: false,
};

export function useAlarmSettings() {
  const settings = ref<AlarmSettings>({ ...DEFAULT_SETTINGS });

  function load() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      if (raw) settings.value = { ...DEFAULT_SETTINGS, ...JSON.parse(raw) };
    } catch {}
  }

  function update(patch: Partial<AlarmSettings>) {
    settings.value = { ...settings.value, ...patch };
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value));
  }

  load();

  return { settings, update };
}
