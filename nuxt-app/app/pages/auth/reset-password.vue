<script setup lang="ts">
definePageMeta({ layout: "auth" });

const route = useRoute();
const authApi = useAuthApi();
const authStore = useAuthStore();
const { notify } = useAppNotification();

const step = computed(() => (route.query.step === "set" ? "set" : "email"));

const form = reactive({ email: "", password: "", confirmPassword: "" });
const errors = reactive({ email: "", password: "", confirmPassword: "" });
const loading = ref(false);
const submitError = ref("");

function validateEmail(): boolean {
  errors.email = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email.trim())
    ? ""
    : "A valid email is required";
  return !errors.email;
}

function validatePassword(): boolean {
  errors.password =
    form.password.length >= 6 ? "" : "Password must be at least 6 characters";
  errors.confirmPassword =
    form.confirmPassword === form.password ? "" : "Passwords do not match";
  return !errors.password && !errors.confirmPassword;
}

async function handleRequestCode() {
  if (!validateEmail()) return;
  loading.value = true;
  submitError.value = "";
  try {
    const response = await authApi.forgottenPassword({
      email: form.email.trim(),
    });
    authStore.setPendingToken(response.token);
    notify({
      message: "Reset code sent to your email.",
      type: "success",
      duration: 5000,
    });
    await navigateTo("/auth/confirm-otp?flow=reset");
  } catch (error) {
    submitError.value = (error as Error).message;
  } finally {
    loading.value = false;
  }
}

async function handleSetPassword() {
  if (!validatePassword()) return;
  if (!authStore.hasPendingToken) {
    submitError.value = "Your reset session has expired. Please start over.";
    return;
  }

  loading.value = true;
  submitError.value = "";
  try {
    await authApi.setNewPassword(
      {
        password: form.password,
        confirmPassword: form.confirmPassword,
      },
      authStore.pendingToken,
    );
    authStore.clearPendingToken();
    notify({ message: "Password updated successfully", type: "success" });
    await navigateTo("/auth/login");
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
      <template v-if="step === 'email'">
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Reset your password
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Enter your account email and we'll send you a reset code.
        </p>
      </template>
      <template v-else>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
          Set a new password
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Choose a new password for your account.
        </p>
      </template>
    </div>

    <form
      class="flex flex-col gap-4"
      @submit.prevent="
        step === 'email' ? handleRequestCode() : handleSetPassword()
      "
    >
      <template v-if="step === 'email'">
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

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <AppButton type="submit" :loading="loading" :disabled="loading">
          Send reset code
        </AppButton>
      </template>

      <template v-else>
        <AppInput
          v-model="form.password"
          type="password"
          name="password"
          label="New password"
          hint="required"
          placeholder="At least 6 characters"
          :disabled="loading"
        />
        <p v-if="errors.password" class="text-xs text-red-500 -mt-3">
          {{ errors.password }}
        </p>

        <AppInput
          v-model="form.confirmPassword"
          type="password"
          name="confirmPassword"
          label="Confirm new password"
          hint="required"
          placeholder="Repeat your new password"
          :disabled="loading"
        />
        <p v-if="errors.confirmPassword" class="text-xs text-red-500 -mt-3">
          {{ errors.confirmPassword }}
        </p>

        <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

        <AppButton
          type="submit"
          color="primary"
          class="w-full py-3 bg-accent-500 hover:bg-accent-600 rounded-lg text-white font-medium disabled:opacity-50 text-center"
          :loading="loading"
          :disabled="loading"
        >
          Update password
        </AppButton>
      </template>
    </form>

    <p class="text-sm text-center text-gray-500 dark:text-gray-400">
      Remembered your password?
      <NuxtLink
        to="/auth/login"
        class="text-accent-500 hover:text-accent-600 font-medium"
      >
        Sign in
      </NuxtLink>
    </p>
  </div>
</template>
