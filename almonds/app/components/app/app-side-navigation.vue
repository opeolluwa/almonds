<script setup lang="ts">
import { primaryRoutes, secondaryRoutes } from "~/data/routes";

const workspaceStore = useWorkspacesStore();
const showCreateModal = ref(false);
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

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}

const sidebarCollapsed = ref(false);

const form = reactive({ name: "", description: "" });
const loading = ref(false);
const errors = reactive({ name: "", description: "" });

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  try {
    await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    showCreateModal.value = false;
    form.name = "";
    form.description = "";
    errors.name = "";
    errors.description = "";
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
}

const workspaceItems = computed(() => [
  workspaceStore.workspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => {
      const isActive = w.identifier === activeId.value;
      return {
        label: w.name,
        icon: isActive
          ? "heroicons:check-circle-solid"
          : "heroicons:check-circle",
        class: isActive
          ? "font-semibold text-accent-500 dark:text-accent-400"
          : "text-gray-700 dark:text-gray-300",
        onSelect: () => workspaceStore.setActiveWorkspace(w.identifier),
      };
    }),
  [
    {
      label: "Manage Workspaces",
      icon: "heroicons:cog-6-tooth",
      class: "text-amber-500 dark:text-amber-400 font-medium",
      onSelect: () => navigateTo("/settings?section=workspaces"),
    },
    {
      label: "Add Workspace",
      icon: "heroicons:plus-circle",
      class: "text-emerald-500 dark:text-emerald-400 font-medium",
      onSelect: () => (showCreateModal.value = true),
    },
  ],
]);

const activeId = computed(() => workspaceStore.currentWorkspace?.identifier);
const activeWorkspaceName = computed(
  () => workspaceStore.currentWorkspace?.name ?? "Select workspace",
);
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
      root: 'bg-white dark:bg-gray-900 overflow-y-scroll transition-[width] duration-300 border-e border-gray-200 dark:border-gray-800',
      header: 'shrink-0 h-auto p-0',
      body: 'flex-1 overflow-y-scroll scrollbar-config p-0 gap-0 ',
      footer: 'shrink-0 h-auto p-0',
    }"
  >
    <!-- Sidebar body: primary nav -->
    <template #default="{ collapsed }">
      <UDropdownMenu
        :items="workspaceItems"
        :ui="{
          content:
            'min-w-52 rounded-xl shadow-xl border border-gray-100 dark:border-gray-800 py-1.5',
          item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
          separator: 'my-1 mx-2',
        }"
      >
        <button
          class="flex items-center gap-2.5 w-full px-3 py-2 mt-2 mb-6 rounded-xl bg-gray-50 dark:bg-gray-800/60 hover:bg-gray-100 dark:hover:bg-gray-800 border border-gray-200 dark:border-gray-700 transition-colors group"
        >
          <span
            class="flex items-center justify-center size-6 rounded-md bg-accent-100 dark:bg-accent-900 shrink-0"
          >
            <UIcon
              name="heroicons:briefcase"
              class="size-3.5 text-accent-600 dark:text-accent-400"
            />
          </span>
          <span
            class="flex-1 text-left text-sm font-medium text-gray-800 dark:text-gray-200 truncate"
          >
            {{ activeWorkspaceName }}
          </span>
          <UIcon
            name="heroicons:chevron-up-down"
            class="size-3.5 text-gray-400 dark:text-gray-500 shrink-0 group-hover:text-gray-600 dark:group-hover:text-gray-300 transition-colors"
          />
        </button>
      </UDropdownMenu>

      <div class="flex flex-col gap-0.5 px-2 py-2 overflow-y-scroll">
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
      <div class="flex flex-col gap-0.5 px-2 pb-4 w-full mb-12">
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
  <UModal v-model:open="showCreateModal">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Create a New Workspace
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Set up a new workspace to organize your projects and files. You can
          have multiple workspaces and switch between them easily.
        </p>
      </div>

      <form
        class="px-6 pb-6 mt-4 flex flex-col gap-4"
        @submit.prevent="handleSubmit"
      >
        <div class="grid grid-cols-2 gap-3">
          <AppInput
            v-model="form.name"
            label="Name"
            hint="required"
            type="text"
            name="workspace-name"
            placeholder="Almonds"
            :error="errors.name"
            :disabled="loading"
          />
          <AppInput
            v-model="form.description"
            label="Description"
            hint="required"
            type="text"
            name="workspace-description"
            placeholder="Organize files and tasks"
            :error="errors.description"
            :disabled="loading"
          />
        </div>

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="loading"
            @click="showCreateModal = false"
          >
            Cancel
          </UButton>
          <UButton
            type="submit"
            color="primary"
            :loading="loading"
            :disabled="loading"
          >
            Save and continue
          </UButton>
        </div>
      </form>
    </template>
  </UModal>
</template>
