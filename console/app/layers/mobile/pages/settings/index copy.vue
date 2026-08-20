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
    Lorem ipsum dolor sit, amet consectetur adipisicing elit. Labore ipsam facilis perferendis ut, fugiat blanditiis dicta eum vero vel quae quo dolorem atque, debitis molestiae rem reiciendis ab velit similique.
    <UNavigationMenu
      orientation="vertical"
      :items="routes"
      highlight
      highlight-color="neutral"
      class="w-full text-white"
      :ui="{
        link: 'px-2 py-2 text-sm rounded-lg text-white/50 transition-colors hover:bg-white/5 hover:text-white data-active:bg-white/10 data-active:text-white data-active:font-medium',
        linkLeadingIcon:
          'size-5 text-white/40 group-hover:text-white group-data-active:text-white',
        label:
          'mt-4 pt-3 border-t border-white/10 px-2 pb-1.5 text-[11px] font-medium uppercase tracking-[0.14em] text-white/30',
      }"
    />
</div>
</template>
