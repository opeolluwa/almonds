<script setup lang="ts">
import { mobileBottomNavRoutes } from "@shared/data/routes";

const route = useRoute();

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>

<template>
  <nav
    class="fixed bottom-0 inset-x-0 z-50 flex items-center justify-around border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-app-dark-900 pt-3 pb-2.5"
    style="padding-bottom: max(0.625rem, env(safe-area-inset-bottom))"
  >
    <NuxtLink
      v-for="item in mobileBottomNavRoutes"
      :key="item.path"
      :to="item.path"
      class="flex flex-col items-center gap-0.5 py-2 px-3 text-[10px] transition-colors"
      :class="
        isActive(item.path)
          ? 'text-primary-500 dark:text-primary-400'
          : 'text-gray-400 dark:text-gray-500'
      "
    >
      <UIcon
        :name="isActive(item.path) ? item.activeIcon : item.icon"
        class="size-5"
      />
      {{ item.name }}
    </NuxtLink>
  </nav>
</template>
