<script setup lang="ts">
import { reactive, ref } from "vue";
import { useWorkspacesStore } from "~/stores/workspaces";

const store = useWorkspacesStore();

// Emit to parent
const emit = defineEmits<{
  (e: "close"): void;
}>();

// Form state
const form = reactive({
  name: "",
  description: "",
});

const loading = ref(false);
const errors = reactive({
  name: "",
  description: "",
});

// Validation
function validate(): boolean {
  errors.name = form.name.trim() ? "" : "Name is required";
  errors.description = form.description.trim() ? "" : "Description is required";
  return !errors.name && !errors.description;
}

// Submit handler
async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  try {
    const created = await store.createWorkspace({
      name: form.name.trim() || "default",
      description: form.description.trim() || "default",
    });

    // Emit close event so parent can hide modal
    emit("close");

    // Optionally reset form
    form.name = "";
    form.description = "";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
    <div class="grid grid-cols-2 gap-3">
      <AppInput
        v-model="form.name"
        label="Name"
        hint="required"
        type="text"
        name="workspace"
        placeholder="Almonds"
        :disabled="loading"
      />

      <AppInput
        v-model="form.description"
        label="Description"
        hint="required"
        type="text"
        name="description"
        placeholder="organize files and tasks"
        :disabled="loading"
      />
    </div>

    <div class="flex justify-end pt-1">
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
