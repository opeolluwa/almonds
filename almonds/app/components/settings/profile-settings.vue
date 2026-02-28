<script setup lang="ts">
import { useUserPreferenceStore } from "~/stores/user-preference";

const store = useUserPreferenceStore();

const form = reactive({
  firstName: store.preference?.firstName ?? "",
  lastName: store.preference?.lastName ?? "",
  email: store.preference?.email ?? "",
});

watch(
  () => store.preference,
  (pref) => {
    if (pref) {
      form.firstName = pref.firstName;
      form.lastName = pref.lastName;
      form.email = pref.email;
    }
  },
);

const saving = ref(false);

async function handleSave() {
  saving.value = true;
  try {
    await store.updatePreference({
      firstName: form.firstName.trim(),
      lastName: form.lastName.trim(),
      email: form.email.trim(),
    });
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
        Profile
      </h2>
      <div class="flex items-center gap-4 mb-6">
        <div
          class="size-16 rounded-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center shrink-0"
        >
          <UIcon name="i-lucide-user" class="size-8 text-gray-400" />
        </div>
        <div>
          <p class="text-sm font-medium text-gray-800 dark:text-gray-100">
            {{ store.fullName }}
          </p>
          <p class="text-xs text-gray-400">{{ store.preference?.email }}</p>
        </div>
      </div>
      <div class="flex flex-col gap-4">
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label
              class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
              >First Name</label
            >
            <input
              v-model="form.firstName"
              type="text"
              class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
            >
          </div>
          <div>
            <label
              class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
              >Last Name</label
            >
            <input
              v-model="form.lastName"
              type="text"
              class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
            >
          </div>
        </div>
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
            >Email</label
          >
          <input
            v-model="form.email"
            type="email"
            class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
          >
        </div>
      </div>
      <div class="mt-5 flex justify-end">
        <button
          :disabled="saving"
          class="px-4 py-2 bg-accent-500 text-white text-sm font-medium rounded-lg hover:bg-accent-600 transition-colors disabled:opacity-50"
          @click="handleSave"
        >
          {{ saving ? "Saving…" : "Save changes" }}
        </button>
      </div>
    </div>
  </div>
</template>
