<script lang="ts" setup>
const workspaceStore = useWorkspacesStore();
const showCreateModal = ref(false);

function handleWorkspaceSelect(identifier: string) {
  const ws = workspaceStore.visibleWorkspaces.find(
    (w) => w.identifier === identifier,
  );
  if (!ws) return;
  else {
    workspaceStore.setActiveWorkspace(identifier);
  }
}

const activeWorkspaceName = computed(
  () => workspaceStore.currentWorkspace?.name ?? "Select workspace",
);

const activeId = computed(() => workspaceStore.currentWorkspace?.identifier);

const workspaceItems = computed(() => [
  workspaceStore?.visibleWorkspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => {
      const isActive = w.identifier === activeId.value;
      return {
        label: w.name,
        icon: isActive
          ? "heroicons:check-circle-solid"
          : w.isSecured && !workspaceStore.isWorkspaceUnlocked(w.identifier)
            ? "heroicons:lock-closed"
            : "heroicons:check-circle",
        class: isActive
          ? "font-semibold text-accent-500 dark:text-accent-400 capitalize"
          : "text-gray-700 dark:text-gray-300 capitalize",
        onSelect: () => handleWorkspaceSelect(w.identifier),
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
</script>
<template>
  <UDropdownMenu
    :items="workspaceItems"
    :ui="{
      content:
        'min-w-52 rounded-xl shadow-xl border border-gray-100 dark:border-gray-800 dark:bg-inherit py-1.5',
      item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
      separator: 'my-1 mx-2',
    }"
  >
    <button
      class="inline-flex  min-w-52 items-center gap-2.5 px-3 py-2 mt-2 mb-6 dark:bg-gray-800/60 border-none hover:rounded-2xl focus:rounded hover:bg-gray-100 dark:hover:bg-gray-800 border border-gray-200 dark:border-gray-700 transition-colors group capitalize"
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
</template>
