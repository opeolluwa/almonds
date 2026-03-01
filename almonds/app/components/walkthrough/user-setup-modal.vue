<script setup lang="ts">
import { useUserPreferenceStore } from "~/stores/user-preference";

const store = useUserPreferenceStore();

const form = reactive({
  firstName: "",
  lastName: "",
  email: "",
});

const loading = ref(false);
const errors = reactive({
  firstName: "",
  lastName: "",
  email: "",
});

function validate(): boolean {
  errors.firstName = form.firstName.trim() ? "" : "First name is required";
  errors.lastName = form.lastName.trim() ? "" : "Last name is required";
  errors.email = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email.trim())
    ? ""
    : "A valid email is required";
  return !errors.firstName && !errors.lastName && !errors.email;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  try {
    await store.createPreference({
      firstName: form.firstName.trim(),
      lastName: form.lastName.trim(),
      email: form.email.trim(),
    });
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <UModal
    :open="true"
    :close="false"
    :dismissible="false"
    @update:open="() => {}"
  >
    <template #header>
      <div class="flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Welcome to Almonds
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Set up your profile to get started.
        </p>
      </div>
    </template>

    <template #body>
      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
              >First Name</label
            >
            <UInput
              v-model="form.firstName"
              placeholder="Jane"
              :disabled="loading"
              :ui="{ root: errors.firstName ? 'ring-1 ring-red-500' : '' }"
            />
            <p v-if="errors.firstName" class="text-xs text-red-500">
              {{ errors.firstName }}
            </p>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
              >Last Name</label
            >
            <UInput
              v-model="form.lastName"
              placeholder="Doe"
              :disabled="loading"
              :ui="{ root: errors.lastName ? 'ring-1 ring-red-500' : '' }"
            />
            <p v-if="errors.lastName" class="text-xs text-red-500">
              {{ errors.lastName }}
            </p>
          </div>
        </div>

        <div class="flex flex-col gap-1">
          <label class="text-xs font-medium text-gray-500 dark:text-gray-400"
            >Email</label
          >
          <UInput
            v-model="form.email"
            type="email"
            placeholder="jane@example.com"
            :disabled="loading"
            :ui="{ root: errors.email ? 'ring-1 ring-red-500' : '' }"
          />
          <p v-if="errors.email" class="text-xs text-red-500">
            {{ errors.email }}
          </p>
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
  </UModal>
</template>
