<script setup lang="ts">
import { useRoute } from "#imports";

definePageMeta({ layout: false });

type Section =
  | "profile"
  | "appearance"
  | "locale"
  | "backup"
  | "ai"
  | "notifications"
  | "alarm"
  | "about"
  | "workspaces";

const route = useRoute();
const router = useRouter();

const navSections: {
  key: Section;
  label: string;
  description: string;
  icon: string;
}[] = [
  {
    key: "profile",
    label: "Profile",
    description: "Name, email & avatar",
    icon: "heroicons:user",
  },
  {
    key: "appearance",
    label: "Appearance",
    description: "Dark mode & font size",
    icon: "heroicons:paint-brush",
  },
  {
    key: "locale",
    label: "Locale",
    description: "Language & region",
    icon: "heroicons:language",
  },
  {
    key: "workspaces",
    label: "Workspaces",
    description: "Create, secure & manage workspaces",
    icon: "heroicons:briefcase",
  },
  {
    key: "backup",
    label: "Backup & Sync",
    description: "Local, cloud or self-hosted backup",
    icon: "heroicons:cloud-arrow-up",
  },
  {
    key: "ai",
    label: "AI & Ollama",
    description: "AI assistants & local models",
    icon: "heroicons:cpu-chip",
  },
  {
    key: "notifications",
    label: "Notifications",
    description: "Alerts & notifications",
    icon: "heroicons:inbox",
  },
  {
    key: "alarm",
    label: "Alarm",
    description: "Sounds, repeat & vibration",
    icon: "heroicons:bell-alert",
  },
  {
    key: "about",
    label: "About",
    description: "Version, build & platform",
    icon: "heroicons:information-circle",
  },
];

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const isValidSection = (s: any): s is Section =>
  navSections.some((n) => n.key === s);

const activeSection = computed<Section | null>(() =>
  isValidSection(route.query.section) ? (route.query.section as Section) : null,
);

const goToSection = (key: Section) => router.push({ query: { section: key } });

const goToMenu = () => router.push({ query: {} });
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div v-if="!activeSection">
        <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
          Preferences
        </h2>
        <div class="flex flex-col gap-2">
          <button
            v-for="s in navSections"
            :key="s.key"
            class="flex items-center gap-4 rounded-xl border border-gray-100 dark:border-gray-800 bg-white dark:bg-gray-900 p-4 text-left transition-colors cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800 w-full"
            @click="goToSection(s.key)"
          >
            <div
              class="size-10 shrink-0 rounded-lg flex items-center justify-center"
            >
              <UIcon
                :name="s.icon"
                class="size-5 text-accent-700 dark:text-accent-300"
              />
            </div>
            <div class="min-w-0 flex-1">
              <h3
                class="text-sm font-medium leading-5 text-gray-900 dark:text-white"
              >
                {{ s.label }}
              </h3>
              <p class="text-sm text-gray-400 dark:text-gray-500 mt-0.5">
                {{ s.description }}
              </p>
            </div>
            <UIcon
              name="heroicons:chevron-right"
              class="size-5 shrink-0 text-gray-400 dark:text-gray-500"
            />
          </button>
        </div>
      </div>

      <template v-else>
        <button
          class="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100 transition-colors cursor-pointer mb-4"
          @click="goToMenu"
        >
          <UIcon name="heroicons:arrow-left" class="size-4" />
          Back to settings
        </button>
        <SettingsProfileSettings v-if="activeSection === 'profile'" />
        <SettingsAppearanceSettings
          v-else-if="activeSection === 'appearance'"
        />
        <SettingsBackupSettings v-else-if="activeSection === 'backup'" />
        <SettingsAiSettings v-else-if="activeSection === 'ai'" />
        <SettingsNotificationsSettings
          v-else-if="activeSection === 'notifications'"
        />
        <SettingsAlarmSettings v-else-if="activeSection === 'alarm'" />
        <SettingsAboutSettings v-else-if="activeSection === 'about'" />
        <SettingsWorkspaces v-else-if="activeSection === 'workspaces'" />
        <SettingsLocaleSettings v-else-if="activeSection === 'locale'" />
      </template>
    </template>
  </NuxtLayout>
</template>
