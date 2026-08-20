<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
        Locale
      </h2>

      <ULocaleSelect
        v-model="locale"
        :locales="Object.values(locales)"
        class="w-48"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import * as locales from "@nuxt/ui/locale";

const locale = ref("en");
</script>
