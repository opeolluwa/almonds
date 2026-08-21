<script setup lang="ts">
import { useAuthStore } from "@shared/stores/auth";

definePageMeta({ layout: "auth" });

const authApi = useAuthApi();
const authStore = useAuthStore();
const { notify } = useAppNotification();
const { rememberedEmail } = useRememberedEmail();

const password = ref("");
const error = ref("");
const loading = ref(false);
const submitError = ref("");

onMounted(() => {
  if (!rememberedEmail.value) navigateTo("/auth/login");
});

function validate(): boolean {
  error.value = password.value ? "" : "Password is required";
  return !error.value;
}

async function handleSubmit() {
  if (!rememberedEmail.value) {
    navigateTo("/auth/login");
    return;
  }
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const response = await authApi.login({
      email: rememberedEmail.value,
      password: password.value,
    });
    authStore.setSession(
      response.accessToken,
      response.refreshToken,
      response.exp,
    );
    notify({ message: "Logged in successfully", type: "success" });
    await navigateTo("/");
  } catch (err) {
    submitError.value = (err as Error).message;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col flex-1">
    <div class="flex items-center justify-between gap-2">
      <h2 class="text-lg font-semibold">Welcome back, Adeoye</h2>
      <NuxtLink
        to="/auth/login"
        class="text-xs text-accent-500 hover:text-accent-600 font-medium whitespace-nowrap"
      >
        Not Adeoye?
      </NuxtLink>
    </div>
    <p class="text-sm text-gray-500 dark:text-gray-400">
      Enter password to continue.
    </p>

    <form class="flex flex-col mt-4" @submit.prevent="handleSubmit">
      <AppInput
        v-model="password"
        type="password"
        name="password"
        label="Password"
        placeholder="••••••••"
        size="lg"
        :disabled="loading"
      />
      <div class="flex justify-end mt-1">
        <NuxtLink
          to="/auth/reset-password"
          class="text-xs text-accent-500 hover:text-accent-600 font-medium"
        >
          Forgot password?
        </NuxtLink>
      </div>
      <p v-if="error" class="text-xs text-red-500 -mt-3">{{ error }}</p>

      <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

      <AppButton
        type="submit"
        class="text-center align-center"
        :loading="loading"
        :disabled="loading"
      >
        Sign in
      </AppButton>
    </form>
  </div>
</template>
