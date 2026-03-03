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
          <AppInput
            v-model="form.firstName"
            label="First name"
            hint="required"
            type="text"
            name="email"
            placeholder="Jane"
            :disabled="loading"
          />

          <AppInput
            v-model="form.lastName"
            label="Last name"
            hint="required"
            type="text"
            name="lastName"
            placeholder="Doe"
            :disabled="loading"
          />
        </div>

        <AppInput
          v-model="form.email"
          label="Email"
          hint="required"
          type="email"
          name="email"
          placeholder="jane@example.com"
          :disabled="loading"
        />

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
