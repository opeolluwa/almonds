<script setup lang="ts">
import _ from "lodash";
import { primaryRoutes, secondaryRoutes } from "~/data/routes";

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
    <UDashboardSidebar
      v-model:collapsed="sidebarCollapsed"
      class="hidden md:flex"
      :collapsible="true"
      :collapsed-size="4"
      :default-size="15"
      :resizable="false"
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

          <!--TODO: enable when the project feature is done, the former class is flex before hidden-->
          <!-- <div class="px-3 flex mb-3 max-w-9/12">
            <UButton
              color="error"
              variant="solid"
              class="flex-1 bg-accent-500 hover:bg-accent-600 justify-center"
            >
              <UIcon name="heroicons:plus" class="size-4 shrink-0" />
              <span v-if="!collapsed">New Project</span>
            </UButton>
          </div> -->

          <USeparator class="mx-3 max-w-9/12" />
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

    <!-- Right column: header + main content -->
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <!-- App header -->
      <header
        class="flex items-center gap-3 h-14 px-4 shrink-0 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
      >
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
            class="flex md:hidden"
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
          <div class="flex items-center gap-3 mb-1"/>
          <slot name="page_title">
            <h1 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
              {{ pageTitle }}
            </h1>
          </slot>

          <div class="flex items-center justify-between mt-5 align-center my-6">
            <button
              v-if="route.path != '/'"
              class="flex items-center gap-1.5 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
              @click="router.back()"
            >
              <UIcon name="heroicons:arrow-left" class="size-3.5" />
              Back
            </button>

            <slot name="primary_cta" />
          </div>

          <slot name="main_content" />
        </main>

        <!-- Inline aside: always visible on desktop -->
        <aside
          class="hidden md:flex flex-col w-72 shrink-0 border-l border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 overflow-hidden"
        >
          <div
            class="flex items-center justify-between px-4 py-3 shrink-0 border-b border-gray-200 dark:border-gray-800"
          >
            <span class="font-semibold text-sm text-gray-900 dark:text-white">Panel</span>
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
          <!-- Header -->
          <div class="flex items-center justify-between px-4 py-4 border-b border-gray-200 dark:border-gray-800 shrink-0">
            <UUser
              name="Nick Woods"
              description="nick.woods@gmail.com"
              :avatar="{
                src: 'https://i.pravatar.cc/150?u=nick-woods',
                icon: 'i-lucide-image',
              }"
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
              <UIcon :name="isActive(r.path) ? r.activeIcon : r.icon" class="size-4 shrink-0" />
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
              <UIcon :name="isActive(r.path) ? r.activeIcon : r.icon" class="size-4 shrink-0" />
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

