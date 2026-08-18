<script setup lang="ts">
definePageMeta({ layout: "auth" });

const authApi = useAuthApi();
const authStore = useAuthStore();
const { notify } = useAppNotification();

const form = reactive({ email: "", password: "", confirmPassword: "" });
const errors = reactive({ email: "", password: "", confirmPassword: "" });
const loading = ref(false);
const submitError = ref("");

function validate(): boolean {
  errors.email = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email.trim())
    ? ""
    : "A valid email is required";
  errors.password =
    form.password.length >= 6 ? "" : "Password must be at least 6 characters";
  errors.confirmPassword =
    form.confirmPassword === form.password ? "" : "Passwords do not match";
  return !errors.email && !errors.password && !errors.confirmPassword;
}

async function handleSubmit() {
  if (!validate()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const response = await authApi.signup({
      email: form.email.trim(),
      password: form.password,
    });
    authStore.setPendingToken(response.token);
    notify({
      message: "Account created. Check your email for a verification code.",
      type: "success",
      duration: 5000,
    });
    await navigateTo(`/auth/confirm-otp?flow=verify`);
  } catch (error) {
    submitError.value = (error as Error).message;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col gap-5">
    <div class="flex flex-col gap-1">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
        Create your account
      </h2>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Get started in a few seconds.
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

      <AppInput
        v-model="form.password"
        type="password"
        name="password"
        label="Password"
        hint="required"
        placeholder="At least 6 characters"
        :disabled="loading"
        :enable-password-toggle="false"
      />
      <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
        {{ errors.password }}
      </p>

      <AppInput
        v-model="form.confirmPassword"
        type="password"
        name="confirmPassword"
        label="Confirm password"
        hint="required"
        placeholder="Repeat your password"
        :disabled="loading"
        :enable-password-toggle="false"
      />
      <p v-if="errors.confirmPassword" class="text-xs text-red-500 -mt-3">
        {{ errors.confirmPassword }}
      </p>

      <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

      <AppButton type="submit" :loading="loading" :disabled="loading">
        Create account
      </AppButton>
    </form>

    <p class="text-sm text-center text-gray-500 dark:text-gray-400">
      Already have an account?
      <NuxtLink
        to="/auth/login"
        class="text-accent-500 hover:text-accent-600 font-medium"
      >
        Sign in
      </NuxtLink>
    </p>
  </div>
</template>
