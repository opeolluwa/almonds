<script setup lang="ts">
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";

const route = useRoute();
const router = useRouter();
const { toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (routeItem) => routeItem.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));
</script>

<template>
  <header
    id="mobile_app_header"
    class="absolute top-0 max-h-20 py-3.5 flex items-center justify-between px-6 z-50 left-0 w-full bg-white dark:bg-app-dark-800"
  >
    <UButton
      v-if="isTopLevel"
      size="md"
      color="neutral"
      variant="ghost"
      icon="heroicons:bars-3"
      aria-label="Open menu"
      @click="toggleMobileNav()"
    />
    <NuxtLink v-else class="inline-flex" @click="router.back()">
      <UIcon name="lucide:arrow-left" class="size-5" />
    </NuxtLink>

    <div class="flex items-center gap-1">
      <UButton
        size="md"
        color="neutral"
        variant="ghost"
        icon="heroicons:bell"
        aria-label="Notifications"
        @click="navigateTo('/notifications')"
      />
    </div>
  </header>
</template>
