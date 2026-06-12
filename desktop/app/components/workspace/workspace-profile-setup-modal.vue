<script setup lang="ts">
const props = defineProps<{
  open: boolean;
  workspaceName: string;
  initialFirstName?: string;
  initialLastName?: string;
  initialEmail?: string;
}>();

const emit = defineEmits<{
  done: [];
}>();

const preferenceStore = useUserPreferenceStore();
const profileForm = reactive({ firstName: "", lastName: "", email: "" });
const loading = ref(false);

watch(
  () => props.open,
  (val) => {
    if (val) {
      profileForm.firstName = props.initialFirstName ?? "";
      profileForm.lastName = props.initialLastName ?? "";
      profileForm.email = props.initialEmail ?? "";
    }
  },
);

async function submitProfileSetup() {
  loading.value = true;
  try {
    await preferenceStore.createPreference({
      firstName: profileForm.firstName.trim(),
      lastName: profileForm.lastName.trim(),
      email: profileForm.email.trim(),
    });
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
    emit("done");
  }
}

async function useDefaultProfile() {
  try {
    await preferenceStore.createPreference({
      firstName: profileForm.firstName.trim(),
      lastName: profileForm.lastName.trim(),
      email: profileForm.email.trim(),
    });
  } catch (e) {
    console.error(e);
  }
  emit("done");
}
</script>

<template>
  <UModal :open="open" @close="useDefaultProfile">
    <template #content>
      <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Set up your profile
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Add a profile for
          <span class="font-medium text-gray-700 dark:text-gray-200">{{
            workspaceName
          }}</span
          >, or continue with your existing details.
        </p>
      </div>

      <div class="px-6 pb-6 mt-4 flex flex-col gap-4">
        <div class="grid grid-cols-2 gap-3">
          <AppInput
            v-model="profileForm.firstName"
            label="First name"
            type="text"
            name="profile-first-name"
            placeholder="John"
            :disabled="loading"
          />
          <AppInput
            v-model="profileForm.lastName"
            label="Last name"
            type="text"
            name="profile-last-name"
            placeholder="Doe"
            :disabled="loading"
          />
        </div>
        <AppInput
          v-model="profileForm.email"
          label="Email"
          type="email"
          name="profile-email"
          placeholder="john@example.com"
          :disabled="loading"
        />

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            color="neutral"
            variant="ghost"
            :disabled="loading"
            @click="useDefaultProfile"
          >
            Continue with default
          </UButton>
          <UButton
            type="button"
            color="primary"
            :loading="loading"
            :disabled="
              !profileForm.firstName.trim() ||
              !profileForm.email.trim() ||
              loading
            "
            @click="submitProfileSetup"
          >
            Save profile
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>
