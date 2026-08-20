<template>
  <main
    id="auth_layout"
    class="mobile_app p-6 flex flex-col bg-gray-50 dark:bg-surface-950"
  >
    <div class="absolute top-10 left-6">
      <NuxtLink
        v-if="route.path !== '/auth/login'"
        to="/auth/login"
        class="inline-flex"
      >
        <UIcon name="lucide:arrow-left" class="size-5" />
      </NuxtLink>
    </div>
    <slot />
  </main>
</template>

<script lang="ts" setup>
const route = useRoute();
</script>

<style scoped>
#auth_layout form input,
#auth_layout form button {
  padding: 30px 4px;
}
</style>
