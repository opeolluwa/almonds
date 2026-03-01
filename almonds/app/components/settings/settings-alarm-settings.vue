<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useAlarmSettings } from "~/composables/useAlarmSettings";

interface AlarmSound {
  name: string;
  filename: string;
}

const { settings, update } = useAlarmSettings();

const sounds = ref<AlarmSound[]>([]);
const playingFilename = ref<string | null>(null);

onMounted(async () => {
  try {
    sounds.value = await invoke<AlarmSound[]>("list_alarm_sounds");
  } catch (e) {
    console.error("Failed to load alarm sounds:", e);
  }
});

async function togglePreview(filename: string) {
  if (playingFilename.value === filename) {
    await invoke("stop_alarm_sound");
    playingFilename.value = null;
  } else {
    playingFilename.value = filename;
    try {
      await invoke("play_alarm_sound", { filename });
    } catch (e) {
      console.error("Failed to play alarm sound:", e);
    }
    // Clear playing state once the sound naturally ends (rough estimate via timeout)
    // The backend plays once so we just clear after a generous window.
    setTimeout(() => {
      if (playingFilename.value === filename) playingFilename.value = null;
    }, 30_000);
  }
}

onUnmounted(() => {
  if (playingFilename.value) invoke("stop_alarm_sound").catch(() => {});
});

const leadTimeOptions: { value: number; label: string }[] = [
  { value: 0, label: "At time" },
  { value: 5, label: "5 min" },
  { value: 10, label: "10 min" },
  { value: 15, label: "15 min" },
  { value: 30, label: "30 min" },
];

const snoozeOptions: { value: number; label: string }[] = [
  { value: 5, label: "5 min" },
  { value: 10, label: "10 min" },
  { value: 15, label: "15 min" },
];
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
        Alarm
      </h2>
      <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700">
        <!-- Default sound: list -->
        <div class="py-3">
          <p class="text-sm text-gray-700 dark:text-gray-200 mb-0.5">
            Default sound
          </p>
          <p class="text-xs text-gray-400 mb-3">
            Sound played when a reminder fires
          </p>

          <div class="flex flex-col gap-1">
            <!-- None option -->
            <div
              class="flex items-center justify-between px-3 py-2 rounded-lg cursor-pointer transition-colors"
              :class="
                settings.defaultSound === null
                  ? 'bg-accent-50 dark:bg-accent-950'
                  : 'hover:bg-gray-50 dark:hover:bg-gray-700/50'
              "
              @click="update({ defaultSound: null })"
            >
              <div class="flex items-center gap-2.5">
                <span
                  class="size-3.5 rounded-full border-2 shrink-0 transition-colors"
                  :class="
                    settings.defaultSound === null
                      ? 'border-accent-500 bg-accent-500'
                      : 'border-gray-300 dark:border-gray-500'
                  "
                />
                <span
                  class="text-sm"
                  :class="
                    settings.defaultSound === null
                      ? 'text-accent-700 dark:text-accent-300 font-medium'
                      : 'text-gray-600 dark:text-gray-400'
                  "
                  >None</span
                >
              </div>
            </div>

            <!-- Sound options -->
            <div
              v-for="s in sounds"
              :key="s.filename"
              class="flex items-center justify-between px-3 py-2 rounded-lg cursor-pointer transition-colors"
              :class="
                settings.defaultSound === s.filename
                  ? 'bg-accent-50 dark:bg-accent-950'
                  : 'hover:bg-gray-50 dark:hover:bg-gray-700/50'
              "
              @click="update({ defaultSound: s.filename })"
            >
              <div class="flex items-center gap-2.5">
                <span
                  class="size-3.5 rounded-full border-2 shrink-0 transition-colors"
                  :class="
                    settings.defaultSound === s.filename
                      ? 'border-accent-500 bg-accent-500'
                      : 'border-gray-300 dark:border-gray-500'
                  "
                />
                <span
                  class="text-sm capitalize"
                  :class="
                    settings.defaultSound === s.filename
                      ? 'text-accent-700 dark:text-accent-300 font-medium'
                      : 'text-gray-600 dark:text-gray-400'
                  "
                  >{{ s.name }}</span
                >
              </div>

              <button
                class="p-1.5 rounded-md transition-colors shrink-0"
                :class="
                  playingFilename === s.filename
                    ? 'text-accent-500 bg-accent-50 dark:bg-accent-950'
                    : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700'
                "
                @click.stop="togglePreview(s.filename)"
              >
                <UIcon
                  :name="
                    playingFilename === s.filename
                      ? 'heroicons:stop'
                      : 'heroicons:play'
                  "
                  class="size-3.5"
                />
              </button>
            </div>
          </div>
        </div>

        <!-- Lead time -->
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">
              Remind me before
            </p>
            <p class="text-xs text-gray-400 mt-0.5">
              How early to trigger the alarm
            </p>
          </div>
          <div class="flex gap-1 bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5">
            <button
              v-for="opt in leadTimeOptions"
              :key="opt.value"
              class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors"
              :class="
                settings.leadTimeMinutes === opt.value
                  ? 'bg-white dark:bg-gray-600 text-gray-800 dark:text-gray-100 shadow-sm'
                  : 'text-gray-500 dark:text-gray-400'
              "
              @click="update({ leadTimeMinutes: opt.value })"
            >
              {{ opt.label }}
            </button>
          </div>
        </div>

        <!-- Snooze duration -->
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">
              Snooze duration
            </p>
            <p class="text-xs text-gray-400 mt-0.5">
              How long to snooze when dismissed
            </p>
          </div>
          <div class="flex gap-1 bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5">
            <button
              v-for="opt in snoozeOptions"
              :key="opt.value"
              class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors"
              :class="
                settings.snoozeDurationMinutes === opt.value
                  ? 'bg-white dark:bg-gray-600 text-gray-800 dark:text-gray-100 shadow-sm'
                  : 'text-gray-500 dark:text-gray-400'
              "
              @click="update({ snoozeDurationMinutes: opt.value })"
            >
              {{ opt.label }}
            </button>
          </div>
        </div>

        <!-- Repeat alarm -->
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">Repeat alarm</p>
            <p class="text-xs text-gray-400 mt-0.5">
              Keep alerting until manually dismissed
            </p>
          </div>
          <button
            class="relative w-10 h-6 rounded-full transition-colors"
            :class="
              settings.repeatAlarm
                ? 'bg-accent-500'
                : 'bg-gray-200 dark:bg-gray-600'
            "
            @click="update({ repeatAlarm: !settings.repeatAlarm })"
          >
            <span
              class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
              :class="settings.repeatAlarm ? 'translate-x-4' : 'translate-x-0'"
            />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
