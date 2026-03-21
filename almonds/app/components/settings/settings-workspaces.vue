<script setup lang="ts">
const { notify } = useAppNotification();
const workspaceStore = useWorkspacesStore();
async function handleDelete(identifier: string) {
  await workspaceStore.deleteWorkspace(identifier);
}

const workspaces = computed(() => workspaceStore.workspaces);

const loadingWorkspaces = ref(false);
const refreshWorkspace = async () => {
  loadingWorkspaces.value = true;
  try {
    notify({ message: "Fetching workspaces", type: "info" });
  } catch (error) {
    notify({
      message: (error as Error).message || "Failed to fetch workspaces",
    });
  } finally {
    loadingWorkspaces.value = false;
    notify({ message: "Workspaces synced", type: "success" });
  }
};
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <button
      class="w-full text-left cursor-pointer rounded-lg border border-gray-100 dark:border-gray-700 p-5 hover:bg-accent-950 font-medium transition-colors"
      @click="refreshWorkspace"
    >
      <span
        class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400"
      >
        <UIcon
          :loading="loadingWorkspaces"
          name="heroicons:arrow-path"
          class="size-4 hover:text-accent-700"
        />
        Reload workspaces
      </span>
    </button>

    <div
      v-for="(workspace, index) in workspaces"
      :key="index"
      class="cursor-pointer"
    >
      <WorkspaceCard :workspace="workspace" @delete="handleDelete" />
    </div>
  </div>
</template>
