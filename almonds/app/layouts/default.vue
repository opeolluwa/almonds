<script setup lang="ts">
import _ from "lodash";
import SideNavigation from "~/components/app/side-navigation.vue";
import { primaryRoutes, secondaryRoutes } from "~/data/routes";
import { useUserPreferenceStore } from "~/stores/user-preference";

const preferenceStore = useUserPreferenceStore();
const workspaceStore = useWorkspacesStore();

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
const router = useRouter();
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

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}

const sidebarCollapsed = ref(false);
const asideOpen = ref(false);
const mobileNavOpen = ref(false);

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
    <SideNavigation />

    <!-- Right column: header + main content -->
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <!-- App header -->
      <header
        class="shrink-0 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
        style="padding-top: env(safe-area-inset-top)"
      >
        <div class="flex justify-between items-center gap-3 h-14 px-4">
          <!-- Hamburger: mobile only -->
          <UButton
            class="flex md:hidden shrink-0"
            size="sm"
            color="neutral"
            variant="ghost"
            icon="heroicons:bars-3"
            aria-label="Open navigation"
            @click="mobileNavOpen = true"
          />

          <!-- Expand sidebar button (only shown when sidebar is collapsed) -->
          <UDashboardSidebarCollapse
            v-if="sidebarCollapsed"
            size="sm"
            color="neutral"
            variant="ghost"
            class="hidden md:flex shrink-0"
          />

          <div class="flex items-center gap-1">
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:chevron-left"
              @click="router.back()"
            />
          </div>

          <!-- Search bar -->
          <div class="mx-auto w-6/12">
            <input
              :model-value="searchQuery"
              :placeholder="searchConfig?.placeholder ?? 'Search...'"
              :disabled="!searchConfig"
              icon="heroicons:magnifying-glass"
              variant="outline"
              :ui="{ root: 'bg-transparent' }"
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
              class="flex md:hidden"
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:bars-3-bottom-right"
              aria-label="Open panel"
              @click="asideOpen = true"
            />
          </div>
        </div>
      </header>

      <!-- Page content + inline aside (fullscreen mode) -->
      <div class="flex flex-1 overflow-hidden">
        <main class="flex-1 overflow-y-auto p-6 bg-gray-50 dark:bg-surface-950">
          <div class="flex items-center gap-3 mb-1" />
          <slot name="page_title">
            <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
              {{ pageTitle }}
            </h1>
          </slot>

          <div class="hidden md:flex items-center justify-end mt-5 my-6">
            <slot name="primary_cta" />
          </div>

          <div class="mt-5">
            <slot name="main_content" />
          </div>
        </main>

        <!-- Mobile FAB for primary_cta -->
        <div
          class="md:hidden fixed bottom-6 right-6 z-40"
          style="padding-bottom: env(safe-area-inset-bottom)"
        >
          <slot name="primary_cta" />
        </div>

        <!-- Inline aside: always visible on desktop -->
        <aside
          class="hidden md:flex flex-col w-72 shrink-0 border-l border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 overflow-hidden"
        >
          <div
            class="flex items-center justify-between px-4 py-3 shrink-0 border-b border-gray-200 dark:border-gray-800"
          >
            <span class="font-semibold text-sm text-gray-900 dark:text-white"
              >Panel</span
            >
          </div>
          <div class="flex-1 overflow-y-auto p-4">
            <slot name="side_content" />
          </div>
        </aside>
      </div>
    </div>

    <!-- Mobile nav drawer -->
    <USlideover
      v-model:open="mobileNavOpen"
      side="left"
      :ui="{ content: 'max-w-64' }"
    >
      <template #content>
        <div class="flex flex-col h-full bg-white dark:bg-gray-900">
          <!-- Safe-area spacer -->
          <div class="shrink-0" style="height: env(safe-area-inset-top)" />
          <!-- Header -->
          <div
            class="flex items-center justify-between px-4 py-4 border-b border-gray-200 dark:border-gray-800 shrink-0"
          >
            <UUser
              :name="preferenceStore.fullName"
              :description="preferenceStore.preference?.email"
              :avatar="{ icon: 'i-lucide-user' }"
              class="min-w-0 flex-1 truncate"
            />
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:x-mark"
              @click="mobileNavOpen = false"
            />
          </div>

          <!-- Primary nav -->
          <nav class="flex flex-col gap-0.5 px-2 py-2 flex-1 overflow-y-auto">
            <NuxtLink
              v-for="r in primaryRoutes"
              :key="r.name"
              :to="r.path"
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
              :class="
                isActive(r.path)
                  ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
              "
              @click="mobileNavOpen = false"
            >
              <UIcon
                :name="isActive(r.path) ? r.activeIcon : r.icon"
                class="size-4 shrink-0"
              />
              {{ r.name }}
            </NuxtLink>
          </nav>

          <!-- Footer -->
          <div class="flex flex-col gap-0.5 px-2 pb-4 shrink-0">
            <USeparator class="mx-1 mb-2" />
            <button
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
              @click="toggleTheme"
            >
              <UIcon :name="themeIcon" class="size-4 shrink-0" />
              {{ themeLabel }}
            </button>
            <NuxtLink
              v-for="r in secondaryRoutes"
              :key="r.name"
              :to="r.path"
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
              :class="
                isActive(r.path)
                  ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
              "
              @click="mobileNavOpen = false"
            >
              <UIcon
                :name="isActive(r.path) ? r.activeIcon : r.icon"
                class="size-4 shrink-0"
              />
              {{ r.name }}
            </NuxtLink>
          </div>
        </div>
      </template>
    </USlideover>

    <!-- Right panel drawer: mobile only -->
    <USlideover
      v-model:open="asideOpen"
      side="right"
      :ui="{ content: 'max-w-sm' }"
    >
      <template #content>
        <div class="flex flex-col h-full bg-white dark:bg-gray-900">
          <!-- Safe-area spacer -->
          <div class="shrink-0" style="height: env(safe-area-inset-top)" />
          <div class="flex flex-col flex-1 overflow-hidden p-5">
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
        </div>
      </template>
    </USlideover>
  </UDashboardGroup>
</template>
