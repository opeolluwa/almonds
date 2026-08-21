<script setup lang="ts">
const routes = [
  {
    label: "Account",
    type: "label" as const,
  },
  {
    label: "Profile",
    description: "Name, email & avatar",
    icon: "heroicons:user",
    to: "/settings/profile",
  },
  {
    label: "Appearance",
    description: "Dark mode & font size",
    icon: "heroicons:paint-brush",
    to: "/settings/appearance",
  },
  {
    label: "Locale",
    description: "Language & region",
    icon: "heroicons:language",
    to: "/settings/locale",
  },

  {
    label: "Workspace",
    type: "label" as const,
  },
  {
    label: "Workspaces",
    description: "Create, secure & manage workspaces",
    icon: "heroicons:briefcase",
    to: "/settings/workspaces",
  },
  {
    label: "Backup & Sync",
    description: "Local, cloud or self-hosted backup",
    icon: "heroicons:cloud-arrow-up",
    to: "/settings/backup",
  },

  {
    label: "Intelligence",
    type: "label" as const,
  },
  {
    label: "AI & Ollama",
    description: "AI assistants & local models",
    icon: "heroicons:cpu-chip",
    to: "/settings/ai",
  },

  {
    label: "Notifications",
    type: "label" as const,
  },
  {
    label: "Notifications",
    description: "Alerts & notifications",
    icon: "heroicons:inbox",
    to: "/settings/notifications",
  },
  {
    label: "Alarm",
    description: "Sounds, repeat & vibration",
    icon: "heroicons:bell-alert",
    to: "/settings/alarm",
  },

  {
    label: "System",
    type: "label" as const,
  },
  {
    label: "About",
    description: "Version, build & platform",
    icon: "heroicons:information-circle",
    to: "/settings/about",
  },
];
</script>

<template>
  <div>
    <UNavigationMenu
      #item="{ item }"
      orientation="vertical"
      :items="routes"
      highlight
      highlight-color="neutral"
      class="w-full"
      :ui="{
        link: 'px-2 py-2 text-sm rounded-lg transition-colors hover:bg-white/5 data-active:font-medium',
        linkLeadingIcon: 'size-5  ',
        label:
          'mt-4 pt-3 border-t border-white/10 px-2 pb-1.5 text-[11px] font-medium uppercase tracking-[0.14em]',
      }"
    >
      <template v-if="item.type == 'label'">
        <div
          class="px-2 py-1.5 text-sm font-medium uppercase text-gray-900 dark:text-white tracking-[0.14em]"
        >
          {{ item.label }}
        </div>
      </template>
      <template v-else>
        <NuxtLink :to="item.to" class="flex items-center gap-4">
          <div
            class="size-10 shrink-0 rounded-lg flex items-center justify-center"
          >
            <UIcon
              :name="item.icon"
              class="size-5 text-accent-700 dark:text-accent-300"
            />
          </div>
          <div class="min-w-0 flex-1">
            <h3
              class="text-sm font-medium leading-5 text-gray-900 dark:text-white"
            >
              {{ item.label }}
            </h3>
            <p class="text-sm text-gray-400 dark:text-gray-500 mt-0.5">
              {{ item.description }}
            </p>
          </div>
        </NuxtLink>
      </template>
    </UNavigationMenu>
  </div>
</template>
