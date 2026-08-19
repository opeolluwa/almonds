<script setup lang="ts">
import { primaryRoutes, secondaryRoutes } from "~/data/routes";
import { useSidebarStore } from "~/stores/sidebar";

const route = useRoute();
const colorMode = useColorMode();
const sidebarStore = useSidebarStore();

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});

function toggleTheme() {
  isDark.value = !isDark.value;
}

const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);

const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>

<template>
  <UDashboardSidebar
    id="app"
    v-model:collapsed="sidebarStore.collapsed"
    class="hidden md:flex"
    :collapsible="true"
    :collapsed-size="4"
    :default-size="18"
    resizable
    :min-size="4"
    :max-size="42"
    :ui="{
      root: 'bg-white dark:bg-gray-900 overflow-y-auto transition-[width] duration-300 border-e border-gray-200 dark:border-gray-800',
      header: 'shrink-0 h-auto p-0',
      body: 'flex-1 overflow-y-auto scrollbar-config p-0 gap-0',
      footer: 'shrink-0 h-auto p-0',
      handle: 'cursor-ew-resize',
    }"
  >
    <template #header>
      <UDashboardSidebarCollapse
        class="pl-6"
        :icon="
          sidebarStore.collapsed
            ? 'i-lucide-panel-left-open'
            : 'i-lucide-panel-left-close'
        "
    /></template>

    <template #resize-handle="{ onMouseDown, onTouchStart, onDoubleClick }">
      <UDashboardResizeHandle
        class="after:absolute after:inset-y-0 after:right-0 after:w-px hover:after:bg-(--ui-border-accented) after:transition"
        @mousedown="onMouseDown"
        @touchstart="onTouchStart"
        @dblclick="onDoubleClick"
      />
    </template>

    <template #default="{ collapsed }">
      <div class="flex flex-col gap-0.5 px-2 py-2 overflow-y-scroll">
        <UTooltip
          v-for="r in primaryRoutes"
          :key="r.name"
          :text="r.name"
          :disabled="!collapsed"
        >
          <NuxtLink
            :to="r.path"
            class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
            :class="[
              collapsed ? 'justify-center' : 'gap-3',
              isActive(r.path)
                ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800',
            ]"
          >
            <UIcon
              :name="isActive(r.path) ? r.activeIcon : r.icon"
              class="size-4 shrink-0"
            />
            <span v-if="!collapsed">{{ r.name }}</span>
          </NuxtLink>
        </UTooltip>
      </div>
    </template>

    <template #footer="{ collapsed }">
      <div class="flex flex-col gap-0.5 px-2 pb-4 w-full mb-12">
        <USeparator class="mx-1 mb-2" />

        <UTooltip :text="themeLabel" :disabled="!collapsed">
          <button
            class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
            :class="collapsed ? 'justify-center' : 'gap-3'"
            @click="toggleTheme"
          >
            <UIcon :name="themeIcon" class="size-4 shrink-0" />
            <span v-if="!collapsed">{{ themeLabel }}</span>
          </button>
        </UTooltip>

        <UTooltip v-for="r in secondaryRoutes" :key="r.name" :text="r.name">
          <NuxtLink
            :to="r.path"
            class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
            :class="[
              collapsed ? 'justify-center' : 'gap-3',
              isActive(r.path)
                ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800',
            ]"
          >
            <UIcon
              :name="isActive(r.path) ? r.activeIcon : r.icon"
              class="size-4 shrink-0"
            />
            <span v-if="!collapsed">{{ r.name }}</span>
          </NuxtLink>
        </UTooltip>
      </div>
    </template>
  </UDashboardSidebar>
</template>
