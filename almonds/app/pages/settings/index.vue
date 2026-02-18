<script setup lang="ts">
definePageMeta({ layout: false });

type Section = "profile" | "appearance" | "ai" | "notifications" | "about";

const activeSection = ref<Section>("profile");

const navSections = [
  {
    key: "profile" as Section,
    label: "Profile",
    icon: "heroicons:user-circle",
  },
  {
    key: "appearance" as Section,
    label: "Appearance",
    icon: "heroicons:paint-brush",
  },
  {
    key: "ai" as Section,
    label: "AI & Ollama",
    icon: "heroicons:cpu-chip",
  },
  {
    key: "notifications" as Section,
    label: "Notifications",
    icon: "heroicons:bell",
  },
  {
    key: "about" as Section,
    label: "About",
    icon: "heroicons:information-circle",
  },
];

// Profile
const displayName = ref("Nick Woods");
const email = ref("nick.woods@gmail.com");

// Appearance
const colorMode = useColorMode();
const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});
const fontSize = ref<"sm" | "md" | "lg">("md");
const { accent: selectedAccent, setAccent } = useAccentColor();
const accentOptions: { key: AccentKey; label: string; bg: string }[] = [
  { key: "rose", label: "Rose", bg: "bg-rose-600" },
  { key: "emerald", label: "Emerald", bg: "bg-emerald-500" },
  { key: "sky", label: "Sky", bg: "bg-sky-500" },
  { key: "amber", label: "Amber", bg: "bg-amber-500" },
];

// AI & Ollama
const ollamaUrl = ref("http://localhost:11434");
const defaultModel = ref("llama3");
const models = ["llama3", "codellama", "mistral", "gemma"];

// Notifications
const notifTasksDue = ref(true);
const notifReminders = ref(true);
const notifSyncComplete = ref(false);
const notifAppUpdates = ref(true);
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <!-- Profile -->
      <div v-if="activeSection === 'profile'" class="flex flex-col gap-4 mt-4">
        <div
          class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
        >
          <h2
            class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4"
          >
            Profile
          </h2>
          <div class="flex items-center gap-4 mb-6">
            <div class="relative shrink-0">
              <img
                src="https://i.pravatar.cc/150?u=nick-woods"
                class="size-16 rounded-full object-cover"
                alt="Avatar"
              />
              <button
                class="absolute -bottom-1 -right-1 size-6 bg-accent-500 rounded-full flex items-center justify-center hover:bg-accent-600 transition-colors"
              >
                <UIcon name="heroicons:camera" class="size-3 text-white" />
              </button>
            </div>
            <div>
              <p class="text-sm font-medium text-gray-800 dark:text-gray-100">
                {{ displayName }}
              </p>
              <p class="text-xs text-gray-400">{{ email }}</p>
            </div>
          </div>
          <div class="flex flex-col gap-4">
            <div>
              <label
                class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
                >Display Name</label
              >
              <input
                v-model="displayName"
                type="text"
                class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
              />
            </div>
            <div>
              <label
                class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
                >Email</label
              >
              <input
                v-model="email"
                type="email"
                class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
              />
            </div>
          </div>
          <div class="mt-5 flex justify-end">
            <button
              class="px-4 py-2 bg-accent-500 text-white text-sm font-medium rounded-lg hover:bg-accent-600 transition-colors"
            >
              Save changes
            </button>
          </div>
        </div>
      </div>

      <!-- Appearance -->
      <div
        v-else-if="activeSection === 'appearance'"
        class="flex flex-col gap-4 mt-4"
      >
        <div
          class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
        >
          <h2
            class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4"
          >
            Appearance
          </h2>
          <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700">
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  Dark mode
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Switch between light and dark theme
                </p>
              </div>
              <button
                class="relative w-10 h-6 rounded-full transition-colors"
                :class="
                  isDark ? 'bg-accent-500' : 'bg-gray-200 dark:bg-gray-600'
                "
                @click="isDark = !isDark"
              >
                <span
                  class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
                  :class="isDark ? 'translate-x-4' : 'translate-x-0'"
                />
              </button>
            </div>
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  Font size
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Adjust text size across the app
                </p>
              </div>
              <div
                class="flex gap-1 bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5"
              >
                <button
                  v-for="sz in ['sm', 'md', 'lg'] as const"
                  :key="sz"
                  class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors uppercase"
                  :class="
                    fontSize === sz
                      ? 'bg-white dark:bg-gray-600 text-gray-800 dark:text-gray-100 shadow-sm'
                      : 'text-gray-500 dark:text-gray-400'
                  "
                  @click="fontSize = sz"
                >
                  {{ sz }}
                </button>
              </div>
            </div>
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  Accent color
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Primary highlight color
                </p>
              </div>
              <div class="flex gap-2">
                <button
                  v-for="a in accentOptions"
                  :key="a.key"
                  class="size-6 rounded-full transition-transform hover:scale-110"
                  :class="[
                    a.bg,
                    selectedAccent === a.key
                      ? 'ring-2 ring-offset-2 ring-gray-400 scale-110'
                      : '',
                  ]"
                  :title="a.label"
                  @click="setAccent(a.key)"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- AI & Ollama -->
      <div
        v-else-if="activeSection === 'ai'"
        class="flex flex-col gap-4 mt-4"
      >
        <div
          class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
        >
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200">
              AI & Ollama
            </h2>
            <div class="flex items-center gap-1.5">
              <span class="size-2 rounded-full bg-emerald-400" />
              <span class="text-xs text-gray-500 dark:text-gray-400"
                >Connected</span
              >
            </div>
          </div>
          <div class="flex flex-col gap-4">
            <div>
              <label
                class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
                >Server URL</label
              >
              <input
                v-model="ollamaUrl"
                type="text"
                class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent font-mono"
              />
            </div>
            <div>
              <label
                class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-2"
                >Default Model</label
              >
              <div class="flex flex-col gap-1">
                <button
                  v-for="m in models"
                  :key="m"
                  class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm transition-colors text-left"
                  :class="
                    defaultModel === m
                      ? 'bg-primary-50 dark:bg-primary-950 text-primary-700 dark:text-primary-300 font-medium'
                      : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
                  "
                  @click="defaultModel = m"
                >
                  <UIcon name="heroicons:cpu-chip" class="size-4 shrink-0" />
                  {{ m }}
                  <UIcon
                    v-if="defaultModel === m"
                    name="heroicons:check"
                    class="size-4 ml-auto text-primary-500"
                  />
                </button>
              </div>
            </div>
          </div>
          <div class="mt-5 flex justify-end">
            <button
              class="px-4 py-2 bg-accent-500 text-white text-sm font-medium rounded-lg hover:bg-accent-600 transition-colors"
            >
              Save
            </button>
          </div>
        </div>
      </div>

      <!-- Notifications -->
      <div
        v-else-if="activeSection === 'notifications'"
        class="flex flex-col gap-4 mt-4"
      >
        <div
          class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
        >
          <h2
            class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4"
          >
            Notifications
          </h2>
          <div
            class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700"
          >
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  Tasks due
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Remind me when tasks are due
                </p>
              </div>
              <button
                class="relative w-10 h-6 rounded-full transition-colors"
                :class="
                  notifTasksDue
                    ? 'bg-accent-500'
                    : 'bg-gray-200 dark:bg-gray-600'
                "
                @click="notifTasksDue = !notifTasksDue"
              >
                <span
                  class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
                  :class="notifTasksDue ? 'translate-x-4' : 'translate-x-0'"
                />
              </button>
            </div>
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  Reminders
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Calendar event reminders
                </p>
              </div>
              <button
                class="relative w-10 h-6 rounded-full transition-colors"
                :class="
                  notifReminders
                    ? 'bg-accent-500'
                    : 'bg-gray-200 dark:bg-gray-600'
                "
                @click="notifReminders = !notifReminders"
              >
                <span
                  class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
                  :class="notifReminders ? 'translate-x-4' : 'translate-x-0'"
                />
              </button>
            </div>
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  Sync complete
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Notify when sync finishes
                </p>
              </div>
              <button
                class="relative w-10 h-6 rounded-full transition-colors"
                :class="
                  notifSyncComplete
                    ? 'bg-accent-500'
                    : 'bg-gray-200 dark:bg-gray-600'
                "
                @click="notifSyncComplete = !notifSyncComplete"
              >
                <span
                  class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
                  :class="
                    notifSyncComplete ? 'translate-x-4' : 'translate-x-0'
                  "
                />
              </button>
            </div>
            <div class="flex items-center justify-between py-3">
              <div>
                <p class="text-sm text-gray-700 dark:text-gray-200">
                  App updates
                </p>
                <p class="text-xs text-gray-400 mt-0.5">
                  Notify about new versions
                </p>
              </div>
              <button
                class="relative w-10 h-6 rounded-full transition-colors"
                :class="
                  notifAppUpdates
                    ? 'bg-accent-500'
                    : 'bg-gray-200 dark:bg-gray-600'
                "
                @click="notifAppUpdates = !notifAppUpdates"
              >
                <span
                  class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
                  :class="
                    notifAppUpdates ? 'translate-x-4' : 'translate-x-0'
                  "
                />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- About -->
      <div
        v-else-if="activeSection === 'about'"
        class="flex flex-col gap-4 mt-4"
      >
        <div
          class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
        >
          <div class="flex items-center gap-3 mb-5">
            <div
              class="size-10 bg-accent-100 dark:bg-accent-950 rounded-lg flex items-center justify-center"
            >
              <UIcon
                name="heroicons:sparkles"
                class="size-5 text-accent-600 dark:text-accent-400"
              />
            </div>
            <div>
              <p class="text-sm font-semibold text-gray-800 dark:text-gray-100">
                Almonds
              </p>
              <p class="text-xs text-gray-400">
                Your personal productivity suite
              </p>
            </div>
          </div>
          <div
            class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700 mb-5"
          >
            <div class="flex items-center justify-between py-3">
              <span class="text-sm text-gray-500 dark:text-gray-400"
                >Version</span
              >
              <span
                class="text-sm font-mono text-gray-700 dark:text-gray-200"
                >0.1.0</span
              >
            </div>
            <div class="flex items-center justify-between py-3">
              <span class="text-sm text-gray-500 dark:text-gray-400"
                >Build</span
              >
              <span
                class="text-sm font-mono text-gray-700 dark:text-gray-200"
                >2026.02</span
              >
            </div>
            <div class="flex items-center justify-between py-3">
              <span class="text-sm text-gray-500 dark:text-gray-400"
                >License</span
              >
              <span class="text-sm text-gray-700 dark:text-gray-200"
                >MIT</span
              >
            </div>
            <div class="flex items-center justify-between py-3">
              <span class="text-sm text-gray-500 dark:text-gray-400"
                >Platform</span
              >
              <span class="text-sm text-gray-700 dark:text-gray-200"
                >macOS</span
              >
            </div>
          </div>
          <button
            class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 hover:text-accent-600 dark:hover:text-accent-400 transition-colors"
          >
            <UIcon name="heroicons:arrow-path" class="size-4" />
            Check for updates
          </button>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
        Preferences
      </h2>
      <div class="flex flex-col gap-1">
        <button
          v-for="s in navSections"
          :key="s.key"
          class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm transition-colors w-full text-left"
          :class="
            activeSection === s.key
              ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'
          "
          @click="activeSection = s.key"
        >
          <UIcon :name="s.icon" class="size-4 shrink-0" />
          {{ s.label }}
        </button>
      </div>
    </template>
  </NuxtLayout>
</template>
