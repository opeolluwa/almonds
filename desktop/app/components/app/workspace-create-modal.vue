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

function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  try {
    const existingPref = preferenceStore.preference;
    const created = await workspaceStore.createWorkspace({
      name: form.name.trim(),
      description: form.description.trim(),
    });
    form.name = "";
    form.description = "";
    errors.name = "";
    errors.description = "";
    emit("update:open", false);
    emit("created", created, {
      firstName: existingPref?.firstName ?? "",
      lastName: existingPref?.lastName ?? "",
      email: existingPref?.email ?? "",
    });
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <UModal :open="open" @update:open="$emit('update:open', $event)">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 mb-4 dark:text-white">
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
            @click="$emit('update:open', false)"
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
