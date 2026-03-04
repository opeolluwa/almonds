<script setup lang="ts">
import type { DropdownMenuItem } from "@nuxt/ui";

const workspaceStore = useWorkspacesStore();
const openWorkspaceModal = ref(false);

const workspaces = computed(() => {
  const existingWorkspace = workspaceStore.workspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => ({
      label: w.name,
      value: w.identifier,
      icon: "heroicons:check-circle",
      onSelect: () => workspaceStore.setActiveWorkspace(w.identifier),
    }));

  return [
    ...existingWorkspace,

    {
      label: "Manage Workspaces",
      color: "neutral",
      icon: "ri:paint-brush-line",
      onSelect: () => navigateTo("/settings"),
    },

    {
      label: "Add Workspace",
      color: "success",
      icon: "heroicons:plus",
      onSelect: () => (openWorkspaceModal.value = true),
    },
  ] as DropdownMenuItem[];
});

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

const sidebarCollapsed = ref(false);
const asideOpen = ref(false);
const mobileNavOpen = ref(false);

const { searchConfig, searchQuery } = useAppSearch();

function onSearchInput(val: string) {
  searchQuery.value = val;
  searchConfig.value?.searchFn?.(val);
}
</script>

<template>
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
        >
      </div>

      <!-- Right actions -->
      <div class="flex items-center gap-1 ml-auto">
        <UTooltip text="Change workspaces">
          <UDropdownMenu
            :items="workspaces"
            size="sm"
            :content="{
              align: 'start',
            }"
            :ui="{
              content: 'w-48',
            }"
          >
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:briefcase"
              aria-label="Open panel"
            />
          </UDropdownMenu>
        </UTooltip>

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

  <UModal v-model:open="openWorkspaceModal" class="px-12 pb-4 pt-8" title="">
    <template #content>
      <div class="flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Create a New Workspace
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Set up a new workspace to organize your projects and files. You can
          have multiple workspaces and switch between them easily.
        </p>
      </div>

      <AppCreateWorkspaceForm
        class="mt-6"
        @close="openWorkspaceModal = false"
      />
    </template>
  </UModal>
</template>
