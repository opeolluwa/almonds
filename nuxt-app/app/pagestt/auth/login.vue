<script setup lang="ts">
import type { LoginRequest } from "~/composables/useAuthApi";

definePageMeta({ layout: false });

const authApi = useAuthApi();
const authStore = useAuthStore();
const { notify } = useAppNotification();

const form = reactive<LoginRequest>({ email: "", password: "" });
const errors = reactive({ email: "", password: "" });
const loading = ref(false);
const submitError = ref("");

onMounted(() => {
  if (authStore.isAuthenticated || authStore.isGuest) {
    navigateTo("/");
  }
});

function validate(): boolean {
  errors.email = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email.trim())
    ? ""
    : "A valid email is required";
  errors.password = form.password ? "" : "Password is required";
  return !errors.email && !errors.password;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const response = await authApi.login({
      email: form.email.trim(),
      password: form.password,
    });
    authStore.setSession(
      response.accessToken,
      response.refreshToken,
      response.exp,
    );
    notify({ message: "Logged in successfully", type: "success" });
    await navigateTo("/");
  } catch (error) {
    submitError.value = (error as Error).message;
  } finally {
    loading.value = false;
  }
}

function continueWithoutLogin() {
  authStore.enterGuestMode();
  navigateTo("/");
}
</script>

<template>
  <NuxtLayout name="auth">
    <div class="flex flex-col gap-5">
      <div class="flex flex-col gap-1">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Welcome back
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Sign in to your Lunar account to continue.
        </p>
      </div>

      <form class="flex flex-col gap-4" @submit.prevent="handleSubmit">
        <AppInput
          v-model="form.email"
          type="email"
          name="email"
          label="Email"
          hint="required"
          placeholder="you@example.com"
          :disabled="loading"
        />
        <p v-if="errors.email" class="text-xs text-red-500 -mt-3">
          {{ errors.email }}
        </p>

        <div>
          <AppInput
            v-model="form.password"
            type="password"
            name="password"
            label="Password"
            hint="required"
            placeholder="••••••••"
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
        </div>
        <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
          {{ errors.password }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <AppButton type="submit" :loading="loading" :disabled="loading">
          Sign in
        </AppButton>
      </form>

      <p class="text-sm text-center text-gray-500 dark:text-gray-400">
        Don't have an account?
        <NuxtLink
          to="/auth/signup"
          class="text-accent-500 hover:text-accent-600 font-medium"
        >
          Sign up
        </NuxtLink>
      </p>
    </div>

    <template #below-card>
      <div class="flex flex-col items-center gap-2 mt-4">
        <NuxtLink
          type="link"
          color="neutral"
          class="px-6 py-2.5 rounded-lg text-sm text-gray-600 dark:text-gray-400 dark:hover:bg-gray-700 cursor-pointer"
          :disabled="loading"
          @click="continueWithoutLogin"
        >
          <p class="text-sm text-gray-400 dark:text-gray-500">
            Continue without account
          </p>
        </NuxtLink>
      </div>
    </template>
  </NuxtLayout>
</template>
