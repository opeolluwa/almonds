<script setup lang="ts">
import type { Workspace } from "~/stores/workspaces";

defineProps<{ open: boolean }>();

const emit = defineEmits<{
  "update:open": [value: boolean];
  created: [
    workspace: Workspace,
    initialProfile: { firstName: string; lastName: string; email: string },
  ];
}>();

const workspaceStore = useWorkspacesStore();
const preferenceStore = useUserPreferenceStore();

const form = reactive({ name: "", description: "" });
const errors = reactive({ name: "", description: "" });
const loading = ref(false);
const submitError = ref("");

function resetForm() {
  Object.assign(form, { name: "", description: "" });
  Object.assign(errors, { name: "", description: "" });
  submitError.value = "";
}

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

function handleClose(value: boolean) {
  if (!value) resetForm();
  emit("update:open", value);
}

function handleCancel(close: () => void) {
  resetForm();
  close();
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const prefSnapshot = preferenceStore.preference;
    const created = await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    resetForm();
    emit("update:open", false);
    emit("created", created, {
      firstName: prefSnapshot?.firstName ?? "",
      lastName: prefSnapshot?.lastName ?? "",
      email: prefSnapshot?.email ?? "",
    });
  } catch (e) {
    console.error(e);
    submitError.value = "Failed to create workspace. Please try again.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <UModal
    :open="open"
    title="Create a New Workspace"
    description="Workspaces allow you to organize your projects and files."
    :ui="{ footer: 'justify-end' }"
    @update:open="handleClose"
  >
    <template #body>
      <form class="pb-6 mt-4 flex flex-col" @submit.prevent="handleSubmit">
        <div class="flex flex-col gap-4">
          <FormsInput
            v-model="form.name"
            label="Name"
            hint="required"
            type="text"
            name="workspace-name"
            placeholder="Almonds"
            :error="errors.name"
            :disabled="loading"
          />
          <FormsInput
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
        <p v-if="submitError" class="text-sm text-red-500">
          {{ submitError }}
        </p>
      </form>
    </template>

    <template #footer="{ close }">
      <UButton
        label="Cancel"
        color="neutral"
        variant="outline"
        @click="handleCancel(close)"
      />
      <UButton
        color="primary"
        :loading="loading"
        :disabled="loading"
        @click="handleSubmit"
      >
        Save and continue
      </UButton>
    </template>
  </UModal>
</template>
