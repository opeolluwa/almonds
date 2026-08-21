<template>
  <main
    id="default_layout_mobile"
    class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-surface-950"
  >
    <header
      class="absolute top-0 py-7 flex items-center justify-between px-6 z-50 left-0 w-full bg-white dark:bg-surface-950"
    >
      <NuxtLink class="inline-flex" @click="router.back()">
        <UIcon name="lucide:arrow-left" class="size-5" />
      </NuxtLink>

      <NuxtLink to="/notifications" class="inline-flex">
        <UIcon name="heroicons:bell" class="size-5" />
      </NuxtLink>
    </header>
    <div
      id="viewport_mobile"
      class="min-h-0 flex-1 overscroll-contain overflow-y-auto p-6"
      :class="hideHeaderAndNav ? 'pb-6 pt-6' : 'pb-24 pt-[50px]'"
    >
      <slot />
    </div>

    <nav
      v-if="!hideHeaderAndNav"
      class="fixed bottom-0 inset-x-0 z-50 flex items-center justify-around border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 pt-3 pb-2.5"
      style="padding-bottom: max(0.625rem, env(safe-area-inset-bottom))"
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
const router = useRouter();

const hideHeaderAndNav = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>
