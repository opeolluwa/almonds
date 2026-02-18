<script setup lang="ts">
definePageMeta({ layout: false });

type Section = "profile" | "appearance" | "backup" | "ai" | "notifications" | "about";

const activeSection = ref<Section>("profile");

const navSections: { key: Section; label: string; icon: string }[] = [
  { key: "profile", label: "Profile", icon: "heroicons:user-circle" },
  { key: "appearance", label: "Appearance", icon: "heroicons:paint-brush" },
  { key: "backup", label: "Backup & Sync", icon: "heroicons:cloud-arrow-up" },
  { key: "ai", label: "AI & Ollama", icon: "heroicons:cpu-chip" },
  { key: "notifications", label: "Notifications", icon: "heroicons:bell" },
  { key: "about", label: "About", icon: "heroicons:information-circle" },
];
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <SettingsProfileSettings v-if="activeSection === 'profile'" />
      <SettingsAppearanceSettings v-else-if="activeSection === 'appearance'" />
      <SettingsBackupSettings v-else-if="activeSection === 'backup'" />
      <SettingsAiSettings v-else-if="activeSection === 'ai'" />
      <SettingsNotificationsSettings v-else-if="activeSection === 'notifications'" />
      <SettingsAboutSettings v-else-if="activeSection === 'about'" />
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
