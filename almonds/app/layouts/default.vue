<script setup lang="ts">
import _ from "lodash";

interface Route {
  path: string;
  name: string;
  icon: string;
  activeIcon: string;
}

const route = useRoute();
const colorMode = useColorMode();

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

const primaryRoutes: Route[] = [
  {
    path: "/",
    name: "Home",
    icon: "heroicons:home",
    activeIcon: "heroicons:home-solid",
  },
  {
    path: "/notes",
    name: "Notes",
    icon: "heroicons:document-text",
    activeIcon: "heroicons:document-text-solid",
  },
  {
    path: "/bookmarks",
    name: "Bookmarks",
    icon: "heroicons:bookmark",
    activeIcon: "heroicons:bookmark-solid",
  },
  {
    path: "/ollama",
    name: "Ollama",
    icon: "heroicons:cpu-chip",
    activeIcon: "heroicons:cpu-chip-solid",
  },
  {
    path: "/snippets",
    name: "Snippets",
    icon: "heroicons:code-bracket",
    activeIcon: "heroicons:code-bracket-solid",
  },
  {
    path: "/todo",
    name: "Todo",
    icon: "heroicons:check-circle",
    activeIcon: "heroicons:check-circle-solid",
  },
  {
    path: "/moodboard",
    name: "Moodboard",
    icon: "heroicons:squares-2x2",
    activeIcon: "heroicons:squares-2x2-solid",
  },
  {
    path: "/scratch-pad",
    name: "Scratch Pad",
    icon: "heroicons:pencil-square",
    activeIcon: "heroicons:pencil-square-solid",
  },
];

const secondaryRoutes: Route[] = [
  {
    path: "/settings",
    name: "Settings",
    icon: "heroicons:cog-6-tooth",
    activeIcon: "heroicons:cog-6-tooth-solid",
  },
];

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}

const sidebarCollapsed = ref(false);
const asideOpen = ref(true);

watch(sidebarCollapsed, () => {
  asideOpen.value = false;
});

const { searchConfig, searchQuery } = useAppSearch();

function onSearchInput(val: string) {
  searchQuery.value = val;
  searchConfig.value?.searchFn?.(val);
}

const pageTitle = computed(() => {
  const raw = route.name?.toString().replaceAll("-", " ") ?? "";
  return raw
    .split(" ")
    .map((w) => _.capitalize(w))
    .join(" ");
});
</script>

<template>
  <UDashboardGroup id="wild_almonds_app" as="div">
    <!-- Sidebar: icons-only strip when collapsed -->
    <UDashboardSidebar
      v-model:collapsed="sidebarCollapsed"
      :collapsible="true"
      :collapsed-size="4"
      :default-size="15"
      :resizable="true"
      :min-size="12"
      :max-size="32"
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
            <UUser
              v-if="!collapsed"
              name="Nick Woods"
              description="nick.woods@gmail.com"
              :avatar="{
                src: 'https://i.pravatar.cc/150?u=nick-woods',
                icon: 'i-lucide-image',
              }"
              class="min-w-0 flex-1 truncate"
            />

            <UDashboardSidebarCollapse
              v-if="!collapsed"
              size="sm"
              color="neutral"
              variant="ghost"
            />

            <UAvatar
              v-else
              src="https://i.pravatar.cc/150?u=nick-woods"
              size="sm"
              class="shrink-0"
            />
          </div>

          <div class="px-3 mb-3 flex max-w-9/12">
            <UButton
              color="error"
              variant="solid"
              class="flex-1 bg-accent-500 hover:bg-accent-600 justify-center"
            >
              <UIcon name="heroicons:plus" class="size-4 shrink-0" />
              <span v-if="!collapsed">New Project</span>
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
        <div class="flex flex-col gap-0.5 px-2 pb-4">
          <USeparator class="mx-1 mb-2" />

          <button
            @click="toggleTheme"
            class="flex items-center py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
            :class="collapsed ? 'justify-center' : 'gap-3'"
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

    <!-- Right column: header + main content -->
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <!-- App header -->
      <header
        class="flex items-center gap-3 h-14 px-4 shrink-0 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
      >
        <!-- Expand sidebar button (only shown when sidebar is collapsed) -->
        <UDashboardSidebarCollapse
          v-if="sidebarCollapsed"
          size="sm"
          color="neutral"
          variant="ghost"
          class="shrink-0"
        />

        <!-- Search bar -->
        <div class="flex-1 max-w-xl">
          <UInput
            :model-value="searchQuery"
            :placeholder="searchConfig?.placeholder ?? 'Search...'"
            :disabled="!searchConfig"
            icon="heroicons:magnifying-glass"
            size="sm"
            variant="outline"
            class="w-full"
            @update:model-value="onSearchInput"
          />
        </div>

        <!-- Right actions -->
        <div class="flex items-center gap-1 ml-auto">
          <UButton
            size="sm"
            color="neutral"
            variant="ghost"
            :icon="themeIcon"
            :aria-label="themeLabel"
            @click="toggleTheme"
          />
          <UButton
            size="sm"
            color="neutral"
            variant="ghost"
            icon="heroicons:bell"
            @click="navigateTo('/notifications')"
          />
          <UButton
            size="sm"
            color="neutral"
            variant="ghost"
            icon="heroicons:bars-3-bottom-right"
            aria-label="Open panel"
            @click="asideOpen = true"
          />
        </div>
      </header>

      <!-- Page content + inline aside (fullscreen mode) -->
      <div class="flex flex-1 overflow-hidden">
        <main class="flex-1 overflow-y-auto p-6 bg-gray-50 dark:bg-surface-950">
          <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
            {{ pageTitle }}
          </h1>
          <slot name="primary_cta" />
          <slot name="main_content" />
        </main>

        <!-- Inline aside: only when sidebar is expanded -->
        <Transition name="aside-slide">
          <aside
            v-if="!sidebarCollapsed && asideOpen"
            class="w-72 shrink-0 flex flex-col border-l border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 overflow-hidden"
          >
            <div
              class="flex items-center justify-between px-4 py-3 shrink-0 border-b border-gray-200 dark:border-gray-800"
            >
              <span class="font-semibold text-sm text-gray-900 dark:text-white"
                >Panel</span
              >
              <UButton
                size="sm"
                color="neutral"
                variant="ghost"
                icon="heroicons:x-mark"
                @click="asideOpen = false"
              />
            </div>
            <div class="flex-1 overflow-y-auto p-4">
              <slot name="side_content" />
            </div>
          </aside>
        </Transition>
      </div>
    </div>

    <!-- Drawer aside: only when sidebar is collapsed (minimized mode) -->
    <USlideover
      v-if="sidebarCollapsed"
      v-model:open="asideOpen"
      side="right"
      :ui="{ content: 'max-w-sm' }"
    >
      <template #content>
        <div class="flex flex-col h-full p-5 bg-white dark:bg-gray-900">
          <div class="flex items-center justify-between mb-4 shrink-0">
            <span class="font-semibold text-sm text-gray-900 dark:text-white">
              Panel
            </span>
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:x-mark"
              @click="asideOpen = false"
            />
          </div>
          <div class="flex-1 overflow-y-auto">
            <slot name="side_content" />
          </div>
        </div>
      </template>
    </USlideover>
  </UDashboardGroup>
</template>

<style scoped>
.aside-slide-enter-active,
.aside-slide-leave-active {
  transition:
    width 0.25s ease,
    opacity 0.25s ease;
  overflow: hidden;
}
.aside-slide-enter-from,
.aside-slide-leave-to {
  width: 0;
  opacity: 0;
}
.aside-slide-enter-to,
.aside-slide-leave-from {
  width: 18rem; /* matches w-72 */
  opacity: 1;
}
</style>
