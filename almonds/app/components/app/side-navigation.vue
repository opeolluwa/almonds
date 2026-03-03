<script setup lang="ts">
import _ from "lodash";
import { SNIPPET_LANGUAGES } from "~/data/languages";
import { primaryRoutes, secondaryRoutes } from "~/data/routes";
import { useUserPreferenceStore } from "~/stores/user-preference";

const preferenceStore = useUserPreferenceStore();
const workspaceStore = useWorkspacesStore();
const submitting = ref(false);
const workspaces = computed(() =>
  workspaceStore.workspaces.map((w) => ({
    label: w.name,
    value: w.identifier,
  })),
);
const workspace = ref({
  label: workspaceStore.currentWorkspace?.name ?? "Select workspace",
  value: workspaceStore.currentWorkspace?.identifier ?? "",
});

const route = useRoute();
const colorMode = useColorMode();
const currentWorkspace = ref("");

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

const sidebarCollapsed = ref(false);
</script>

<template>
  <UDashboardSidebar
    v-model:collapsed="sidebarCollapsed"
    class="hidden md:flex"
    :collapsible="false"
    :collapsed-size="4"
    :default-size="18"
    :resizable="true"
    :min-size="18"
    :max-size="42"
    :ui="{
      root: 'bg-white dark:bg-gray-900 transition-[width] duration-300 border-e border-gray-200 dark:border-gray-800',
      header: 'shrink-0 h-auto p-0',
      body: 'flex-1 overflow-y-auto p-0 gap-0',
      footer: 'shrink-0 h-auto p-0',
    }"
  >
    <!-- Sidebar header: user info + collapse button -->
    <template #header="{ collapsed }">
      <div class="flex flex-col pt-4">
        <div
          class="flex items-center px-4 pb-3 gap-2"
          :class="collapsed ? 'justify-center flex-col' : 'justify-between'"
        >
          <div class="flex flex-col gap-0.5 px-2 pb-4">
            <UUser
              :name="preferenceStore.fullName"
              :description="preferenceStore.preference?.email"
              :avatar="{ icon: 'i-lucide-user' }"
              class="min-w-0 flex-1 truncate"
            />
          </div>
        </div>

        <div class="px-3 flex mb-3 max-w-9/12 hidden">
          <UButton
            color="error"
            variant="solid"
            class="flex-1 bg-accent-500 hover:bg-accent-600 justify-center"
          >
            <UIcon name="heroicons:plus" class="size-4 shrink-0" />
            <span v-if="!collapsed">New Workspace</span>
          </UButton>
        </div>

        <USeparator class="mx-3" />
      </div>
    </template>

    <!-- Sidebar body: primary nav -->
    <template #default="{ collapsed }">
      <div class="flex flex-col gap-0.5 px-2 py-2">
        <NuxtLink
          v-for="r in primaryRoutes"
          :key="r.name"
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
      </div>
    </template>

    <!-- Sidebar footer: theme + secondary nav -->
    <template #footer="{ collapsed }">
      <div class="flex flex-col gap-0.5 px-2 pb-4 w-full">
        <USeparator class="mx-1 mb-2" />

        <button
          class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
          :class="collapsed ? 'justify-center' : 'gap-3'"
          @click="toggleTheme"
        >
          <UIcon :name="themeIcon" class="size-4 shrink-0" />
          <span v-if="!collapsed">{{ themeLabel }}</span>
        </button>

        <NuxtLink
          v-for="r in secondaryRoutes"
          :key="r.name"
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
      </div>
    </template>
  </UDashboardSidebar>
</template>
