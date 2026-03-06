<script setup lang="ts">
import type { DropdownMenuItem } from "@nuxt/ui";
import { reactive, ref, computed } from "vue";

const workspaceStore = useWorkspacesStore();
const router = useRouter();
const colorMode = useColorMode();
const { searchConfig, searchQuery } = useAppSearch();

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});
const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);
const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));

const sidebarCollapsed = ref(false);
const asideOpen = ref(false);
const mobileNavOpen = ref(false);

function onSearchInput(val: string) {
  searchQuery.value = val;
  searchConfig.value?.searchFn?.(val);
}

const showCreateModal = ref(false);

const activeId = computed(() => workspaceStore.currentWorkspace?.identifier);

const workspaces = computed<DropdownMenuItem[]>(() => [
  ...workspaceStore.workspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => {
      const isActive = w.identifier === activeId.value;
      return {
        label: w.name,
        value: w.identifier,
        icon: isActive
          ? "heroicons:check-circle-solid"
          : "heroicons:check-circle",
        class: isActive ? "font-semibold text-accent-500" : "",
        onSelect: () => workspaceStore.setActiveWorkspace(w.identifier),
      };
    }),
  {
    label: "Manage Workspaces",
    color: "neutral",
    icon: "ri:paint-brush-line",
    onSelect: () => navigateTo("/settings?section=workspaces"),
  },
  {
    label: "Add Workspace",
    color: "success",
    icon: "heroicons:plus",
    onSelect: () => (showCreateModal.value = true),
  },
]);

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
</script>

<template>
  <header
    class="shrink-0 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
    style="padding-top: env(safe-area-inset-top)"
  >
    <div class="flex items-center gap-3 h-14 px-4">
      <!-- Hamburger (mobile only) -->
      <UButton
        class="flex md:hidden shrink-0"
        size="sm"
        color="neutral"
        variant="ghost"
        icon="heroicons:bars-3"
        aria-label="Open navigation"
        @click="mobileNavOpen = true"
      />

      <!-- Expand sidebar (desktop, collapsed state only) -->
      <UDashboardSidebarCollapse
        v-if="sidebarCollapsed"
        size="sm"
        color="neutral"
        variant="ghost"
        class="hidden md:flex shrink-0"
      />

      <!-- Back button -->
      <UButton
        size="sm"
        color="neutral"
        variant="ghost"
        icon="heroicons:chevron-left"
        @click="router.back()"
      />

      <!-- Search -->
      <div class="mx-auto w-6/12">
        <input
          :value="searchQuery"
          :placeholder="searchConfig?.placeholder ?? 'Search...'"
          :disabled="!searchConfig"
          class="w-full bg-transparent outline-none text-sm text-gray-700 dark:text-gray-300 placeholder-gray-400"
          @input="onSearchInput(($event.target as HTMLInputElement).value)"
        >
      </div>

      <!-- Right actions -->
      <div class="flex items-center gap-1 ml-auto">
        <!-- Workspace switcher -->
        <UTooltip text="Change workspaces">
          <UDropdownMenu
            :items="workspaces"
            size="sm"
            :content="{ align: 'start' }"
            :ui="{ content: 'w-48' }"
          >
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:briefcase"
              aria-label="Switch workspace"
            />
          </UDropdownMenu>
        </UTooltip>

        <!-- Theme toggle -->
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          :icon="themeIcon"
          :aria-label="themeLabel"
          @click="isDark = !isDark"
        />

        <!-- Notifications -->
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:bell"
          aria-label="Notifications"
          @click="navigateTo('/notifications')"
        />

        <!-- Right panel toggle (mobile only) -->
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

  <!-- Create Workspace Modal -->
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
