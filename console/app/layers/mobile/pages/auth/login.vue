<script setup lang="ts">
import { useAuthStore } from "@shared/stores/auth";
import type { LoginRequest } from "@shared/composables/useAuthApi";

definePageMeta({ layout: "auth" });

const authApi = useAuthApi();
const authStore = useAuthStore();
const { notify } = useAppNotification();
const { rememberEmail } = useRememberedEmail();

const form = reactive<LoginRequest>({ email: "", password: "" });
const errors = reactive({ email: "", password: "" });
const loading = ref(false);
const submitError = ref("");

function validate(): boolean {
  errors.email = emailValidator(form.email) ? "" : "A valid email is required";
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
    rememberEmail(form.email.trim());
    notify({ message: "Logged in successfully", type: "success" });
    await navigateTo("/");
  } catch (error) {
    submitError.value = (error as Error).message;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col flex-1">
    <div class="flex flex-col gap-1">
      <h2 class="text-lg font-semibold">Welcome back</h2>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Sign in to your Lunar account to continue.
      </p>
    </div>

    <form class="flex flex-col mt-4" @submit.prevent="handleSubmit">
      <AppInput
        v-model="form.email"
        type="email"
        name="email"
        label="Email"
        placeholder="you@example.com"
        size="lg"
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
          placeholder="••••••••"
          size="lg"
          :disabled="loading"
        />
        <div class="flex justify-end mt-1">
          <NuxtLink
            to="/auth/reset-password"
            class="text-xs text-primary-500 hover:text-primary-600 font-medium"
          >
            Forgot password?
          </NuxtLink>
        </div>
      </div>
      <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
        {{ errors.password }}
      </p>

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

    <p
      class="text-sm text-left text-gray-500 dark:text-gray-400 w-full left-0 pt-4"
    >
      Don't have an account?
      <NuxtLink
        to="/auth/signup"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign up
      </NuxtLink>
    </p>
  </div>
</template>
