<script setup lang="ts">
const { notify } = useAppNotification();
const workspaceStore = useWorkspacesStore();

// ── secure workspace ──────────────────────────────────────────────────────────
const secureTargetId = ref<string | null>(null);
const securePassword = ref("");
const secureConfirm = ref("");
const secureError = ref("");
const secureSubmitting = ref(false);

const secureTargetWorkspace = computed(() =>
  secureTargetId.value
    ? workspaceStore.workspaces.find(
        (w) => w.identifier === secureTargetId.value,
      )
    : null,
);

function handleToggleSecured(identifier: string) {
  const ws = workspaceStore.workspaces.find((w) => w.identifier === identifier);
  if (!ws) return;
  secureTargetId.value = identifier;
  securePassword.value = "";
  secureConfirm.value = "";
  secureError.value = "";
}

function closeSecureModal() {
  secureTargetId.value = null;
  securePassword.value = "";
  secureConfirm.value = "";
  secureError.value = "";
}

async function submitSecure() {
  if (!secureTargetId.value || !secureTargetWorkspace.value) return;

  if (!secureTargetWorkspace.value.isSecured) {
    // Setting a new password
    if (!securePassword.value) {
      secureError.value = "Password is required.";
      return;
    }
    if (securePassword.value !== secureConfirm.value) {
      secureError.value = "Passwords do not match.";
      return;
    }
  }

  secureSubmitting.value = true;
  secureError.value = "";
  try {
    if (secureTargetWorkspace.value.isSecured) {
      // Removing the password — verify current password first
      const ok = await workspaceStore.verifyWorkspacePassword(
        secureTargetId.value,
        securePassword.value,
      );
      if (!ok) {
        secureError.value = "Incorrect password.";
        return;
      }
      await workspaceStore.updateWorkspace(secureTargetId.value, {
        isSecured: false,
        password: "",
      });
      notify({ message: "Workspace password removed", type: "success" });
    } else {
      await workspaceStore.updateWorkspace(secureTargetId.value, {
        isSecured: true,
        password: securePassword.value,
      });
      workspaceStore.unlockWorkspace(secureTargetId.value);
      notify({ message: "Workspace secured with password", type: "success" });
    }
    closeSecureModal();
  } catch (e) {
    notify({
      message: (e as Error).message || "Failed to update workspace security",
      type: "error",
    });
  } finally {
    secureSubmitting.value = false;
  }
}

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
    notify({
      message: (e as Error).message || "Failed to update workspace",
      type: "error",
    });
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
        @toggle-secured="handleToggleSecured"
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
    <!-- Secure workspace modal -->
    <UModal :open="!!secureTargetId" @close="closeSecureModal">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
            {{
              secureTargetWorkspace?.isSecured
                ? "Remove workspace password"
                : "Set workspace password"
            }}
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            {{
              secureTargetWorkspace?.isSecured
                ? "Enter the current password to remove protection from this workspace."
                : "This workspace will require a password before it can be accessed."
            }}
          </p>
          <UFormField
            :label="
              secureTargetWorkspace?.isSecured ? 'Current password' : 'Password'
            "
          >
            <UInput
              v-model="securePassword"
              type="password"
              placeholder="Enter password"
              class="w-full"
              :disabled="secureSubmitting"
            />
          </UFormField>
          <UFormField
            v-if="!secureTargetWorkspace?.isSecured"
            label="Confirm password"
          >
            <UInput
              v-model="secureConfirm"
              type="password"
              placeholder="Confirm password"
              class="w-full"
              :disabled="secureSubmitting"
            />
          </UFormField>
          <p v-if="secureError" class="text-xs text-red-500 dark:text-red-400">
            {{ secureError }}
          </p>
          <div class="flex items-center gap-2 mt-2">
            <UButton
              size="sm"
              :color="secureTargetWorkspace?.isSecured ? 'error' : 'primary'"
              :loading="secureSubmitting"
              :disabled="!securePassword"
              @click="submitSecure"
            >
              {{
                secureTargetWorkspace?.isSecured
                  ? "Remove password"
                  : "Set password"
              }}
            </UButton>
            <UButton
              variant="ghost"
              size="sm"
              :disabled="secureSubmitting"
              @click="closeSecureModal"
            >
              Cancel
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>
