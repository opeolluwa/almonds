<template>
  <main id="auth_layout_mobile" class="p-6  dark:bg-app-dark-800 flex flex-col">
    <div class="absolute top-10 left-6">
      <NuxtLink
        v-if="!isLoginPage || (isLoginPage && canGoBack)"
        to="/auth/login"
        class="inline-flex"
      >
        <UIcon name="lucide:arrow-left" class="size-5" />
      </NuxtLink>
    </div>

    <div class="absolute top-10 right-6">
      <NuxtLink
        v-if="route.path === '/auth/welcome-back'"
        to="/auth/login"
        class="inline-flex"
      >
        <span
          class="text-xs text-primary-500 hover:text-primary-600 font-medium whitespace-nowrap"
          >Not Adeoye?</span
        >
      </NuxtLink>
    </div>

    <div class="mt-4">
      <slot />
    </div>
  </main>
</template>

<script lang="ts" setup>
const route = useRoute();
const canGoBack = computed(() => window.history.length > 1);
const isLoginPage = computed(() => route.path === "/auth/login");
</script>
