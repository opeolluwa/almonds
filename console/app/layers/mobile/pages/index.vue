<script setup lang="ts">
definePageMeta({ layout: "default" });
</script>

<template>
  <div class="flex h-full flex-col items-center justify-center">
    <h1 class="text-lg font-semibold">Home</h1>
    <DevOnly>
      <NuxtLink
        to="/auth/signup"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Sign up
      </NuxtLink>

      <NuxtLink
        to="/auth/welcome-back"
        class="text-primary-500 hover:text-primary-600 font-medium"
      >
        Welcome back
      </NuxtLink>
    </DevOnly>
  </div>
</template>
