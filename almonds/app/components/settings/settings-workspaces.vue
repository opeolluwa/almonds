<script setup lang="ts">
const { notify } = useAppNotification();
const workspaceStore = useWorkspacesStore();

// ── delete ────────────────────────────────────────────────────────────────────
async function handleDelete(identifier: string) {
  await workspaceStore.deleteWorkspace(identifier);
}

// ── set default ───────────────────────────────────────────────────────────────
async function handleSetDefault(identifier: string) {
  try {
    await workspaceStore.updateWorkspace(identifier, { isDefault: true });
    notify({ message: "Default workspace updated", type: "success" });
  } catch (e) {
    notify({ message: (e as Error).message || "Failed to update workspace", type: "error" });
  }
}

// ── toggle hidden ─────────────────────────────────────────────────────────────
async function handleToggleHidden(identifier: string) {
  const ws = workspaceStore.workspaces.find((w) => w.identifier === identifier);
  if (!ws) return;
  try {
    await workspaceStore.updateWorkspace(identifier, {
      isHidden: !ws.isHidden,
    });
    notify({
      message: ws.isHidden ? "Workspace is now visible" : "Workspace hidden",
      type: "success",
    });
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  }
}

// ── edit ──────────────────────────────────────────────────────────────────────
const editingId = ref<string | null>(null);
const editName = ref("");
const editDescription = ref("");
const editSubmitting = ref(false);

function handleEdit(identifier: string) {
  const ws = workspaceStore.workspaces.find((w) => w.identifier === identifier);
  if (!ws) return;
  editingId.value = identifier;
  editName.value = ws.name;
  editDescription.value = ws.description;
}

function closeEdit() {
  editingId.value = null;
  editName.value = "";
  editDescription.value = "";
}

async function submitEdit() {
  if (!editingId.value) return;
  editSubmitting.value = true;
  try {
    await workspaceStore.updateWorkspace(editingId.value, {
      name: editName.value.trim() || undefined,
      description: editDescription.value.trim(),
    });
    notify({ message: "Workspace updated", type: "success" });
    closeEdit();
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
  } finally {
    editSubmitting.value = false;
  }
}

const workspaces = computed(() => workspaceStore.workspaces);
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <!-- Workspace list -->
    <div
      v-for="workspace in workspaces"
      :key="workspace.identifier"
      class="cursor-pointer"
    >
      <WorkspaceCard
        :workspace="workspace"
        @delete="handleDelete"
        @edit="handleEdit"
        @toggle-hidden="handleToggleHidden"
        @set-default="handleSetDefault"
      />
    </div>

    <!-- Edit modal -->
    <UModal :open="!!editingId" @close="closeEdit">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            Edit workspace
          </h3>
          <UFormField label="Name">
            <UInput
              v-model="editName"
              placeholder="Workspace name"
              class="w-full"
              :disabled="editSubmitting"
            />
          </UFormField>
          <UFormField label="Description">
            <UInput
              v-model="editDescription"
              placeholder="Short description"
              class="w-full"
              :disabled="editSubmitting"
            />
          </UFormField>
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              :loading="editSubmitting"
              :disabled="!editName.trim()"
              @click="submitEdit"
            >
              Save
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="editSubmitting"
              @click="closeEdit"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>
