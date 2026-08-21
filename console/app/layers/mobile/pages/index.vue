<script setup lang="ts">
definePageMeta({ layout: "default" });
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-screen p-6">
    <h1 class="text-lg font-semibold">Home</h1>
    <DevOnly>
      <NuxtLink
        to="/auth/signup"
        class="text-accent-500 hover:text-accent-600 font-medium"
      >
        Sign up
      </NuxtLink>

      <NuxtLink
        to="/auth/welcome-back"
        class="text-accent-500 hover:text-accent-600 font-medium"
      >
        Welcome back
      </NuxtLink>
    </DevOnly>
  </div>
</template>
