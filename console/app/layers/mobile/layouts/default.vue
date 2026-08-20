<template>
  <main
    id="default_layout"
    class="mobile_app flex flex-col bg-gray-50 dark:bg-surface-950"
  >
    <div class="flex-1 overflow-y-auto p-6 pb-24">
      <slot />
    </div>

    <nav
      class="fixed bottom-0 inset-x-0 z-50 flex items-center justify-around border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900"
    >
      <NuxtLink
        v-for="item in mobile_default_layer"
        :key="item.path"
        :to="item.path"
        class="flex flex-col items-center gap-0.5 py-2 px-3 text-[10px] transition-colors"
        :class="
          isActive(item.path)
            ? 'text-accent-500 dark:text-accent-400'
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
  </main>
</template>

<script lang="ts" setup>
import { mobile_default_layer } from "@shared/data/routes";

const route = useRoute();

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>

<style scoped>
#default_layout {
  min-height: 100dvh;
}
</style>
