<template>
  <div
    id="auth_layout"
    class="h-screen bg-gray-50 w-full dark:bg-surface-950 flex flex-col items-center justify-center md:p-6 relative overflow-hidden"
    :class="{ 'rounded-xl': !IS_WEB }"
  >
    <div
      class="pointer-events-none absolute -top-24 -left-24 size-96 rounded-full bg-accent-300/20 dark:bg-accent-500/10 blur-3xl hidden md:block"
    />

    <div
      class="pointer-events-none absolute -bottom-24 -right-24 size-96 rounded-full bg-violet-300/20 dark:bg-violet-500/10 blur-3xl hidden md:block"
    />

    <div
      class="relative w-full max-w-md bg-white dark:bg-gray-800 rounded-2xl border border-gray-100 dark:border-gray-700 pb-8 pt-12 px-8"
    >
      <Transition
        enter-active-class="transition-opacity duration-200"
        leave-active-class="transition-opacity duration-300"
        enter-from-class="opacity-0"
        leave-to-class="opacity-0"
      >
        <slot />
      </Transition>
    </div>

    <slot name="below-card" />
  </div>
</template>

<script lang="ts" setup>
import { IS_WEB } from "@shared/env";
</script>

<style lang="css">
/*@reference "@/assets/css/main.css";*/
</style>
