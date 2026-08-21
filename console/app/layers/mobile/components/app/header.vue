<script setup lang="ts">
import { useAuthStore } from "@shared/stores/auth";
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const { toggleMobileNav } = useMobileNav();

const topLevelPaths = [...primaryRoutes, ...secondaryRoutes].map(
  (routeItem) => routeItem.path,
);
const isTopLevel = computed(() => topLevelPaths.includes(route.path));
</script>

<template>
  <header
    id="mobile_app_header"
    class="absolute top-0 py-7 flex items-center justify-between px-6 z-50 left-0 w-full bg-white dark:bg-surface-950"
  >
    <UButton
      v-if="isTopLevel"
      size="sm"
      color="neutral"
      variant="ghost"
      icon="ri:menu-line"
      aria-label="Open menu"
      @click="toggleMobileNav()"
    />
    <NuxtLink v-else class="inline-flex" @click="router.back()">
      <UIcon name="lucide:arrow-left" class="size-5" />
    </NuxtLink>

    <div class="flex items-center gap-1">
      <UButton
        size="sm"
        color="neutral"
        variant="ghost"
        icon="heroicons:bell"
        aria-label="Notifications"
        @click="navigateTo('/notifications')"
      />

      <UUser
        size="sm"
        class="cursor-pointer"
        :avatar="
          authStore.isGuest || !authStore.isAuthenticated
            ? { icon: 'heroicons:user' }
            : { src: 'https://i.pravatar.cc/150?u=john-doe' }
        "
        @click="navigateTo('/settings/profile')"
      />
    </div>
  </header>
</template>
