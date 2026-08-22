<script lang="ts" setup>
import { useCurrentEditor } from "@domternal/vue";

const { editor } = useCurrentEditor();
</script>

<template>
  <nav
    class="fixed bottom-0 inset-x-0 z-50 flex items-center justify-around border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-app-dark-900 pt-3 pb-2.5"
    style="padding-bottom: max(0.625rem, env(safe-area-inset-bottom))"
  >
    test menu lo
  </nav>
</template>
