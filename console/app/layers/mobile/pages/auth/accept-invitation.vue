<script setup lang="ts">
definePageMeta({ layout: "auth" });

const route = useRoute();
const authApi = useAuthApi();
const { notify } = useAppNotification();

const token = computed(() => (route.query.token as string | undefined) ?? "");
const email = computed(() => (route.query.email as string | undefined) ?? "");

const loading = ref(false);
const submitError = ref("");

async function handleAccept() {
  loading.value = true;
  submitError.value = "";
  try {
    await authApi.acceptInvitation({ token: token.value });
    notify({
      message: "Invitation accepted. Welcome aboard!",
      type: "success",
    });
    await navigateTo("/");
  } catch (error) {
    submitError.value = (error as Error).message;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col gap-5">
    <div class="flex flex-col gap-1 text-center">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
        You're invited!
      </h2>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        You've been invited to join a workspace on Lunar.
      </p>
    </div>

    <div
      class="flex items-center gap-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 p-4"
    >
      <div
        class="size-10 rounded-lg bg-accent-50 dark:bg-accent-950 flex items-center justify-center shrink-0"
      >
        <UIcon name="heroicons:user-group" class="size-5 text-accent-500" />
      </div>
      <div class="min-w-0 flex-1">
        <p
          v-if="email"
          class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate"
        >
          {{ email }}
        </p>
        <p v-else class="text-sm font-medium text-gray-800 dark:text-gray-100">
          Workspace member
        </p>
        <p class="text-xs text-gray-400 mt-0.5">Shared workspace invitation</p>
      </div>
    </div>

    <p v-if="submitError" class="text-sm text-red-500">{{ submitError }}</p>

    <div v-if="token" class="flex flex-col gap-3">
      <AppButton
        color="primary"
        class="w-full py-3 bg-accent-500 hover:bg-accent-600 rounded-lg text-white font-medium disabled:opacity-50 text-center"
        :loading="loading"
        :disabled="loading"
        @click="handleAccept"
      >
        Accept invitation
      </AppButton>

      <p class="text-sm text-center text-gray-500 dark:text-gray-400">
        Prefer to sign in first?
        <NuxtLink
          to="/auth/login"
          class="text-accent-500 hover:text-accent-600 font-medium"
        >
          Sign in
        </NuxtLink>
      </p>
    </div>

    <div v-else class="flex flex-col gap-3">
      <p class="text-sm text-gray-500 dark:text-gray-400 text-center">
        This invitation link is missing a token or has expired.
      </p>
      <NuxtLink
        to="/auth/login"
        class="w-full py-3 bg-accent-500 hover:bg-accent-600 rounded-lg text-white font-medium text-center"
      >
        Go to sign in
      </NuxtLink>
    </div>
  </div>
</template>
