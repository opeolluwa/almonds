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
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
      @click="refreshWorkspace"
    >
      <button
        class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 hover:text-accent-600 dark:hover:text-accent-400 transition-colors"
      >
        <UIcon name="heroicons:arrow-path" class="size-4" />
        Reload workspaces
      </button>
    </div>

    <div v-for="(workspace, index) in workspaces">
      <WorkspaceCard
        :key="index"
        :workspace="workspace"
        @delete="handleDelete"
      />
    </div>
  </div>
</template>
