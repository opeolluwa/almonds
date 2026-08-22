<script lang="ts" setup>
import { primaryRoutes, secondaryRoutes } from "@shared/data/routes";
import { useAuthStore } from "@shared/stores/auth";
import { useUserPreferenceStore } from "@shared/stores/workspace-preferences";

const route = useRoute();
const authStore = useAuthStore();
const preferenceStore = useUserPreferenceStore();
const { mobileNavOpen } = useMobileNav();

const colorMode = useColorMode();
const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});
const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);
const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));

function logout() {
  mobileNavOpen.value = false;
  authStore.clearSession();
  authStore.exitGuestMode();
  navigateTo("/auth/login");
}

const hideHeaderAndNav = computed(() => {
  return (
    route.path.includes("/create-notes") || route.path.includes("/edit-notes")
  );
});


</script>

<template>
  <main
    id="default_layout_mobile"
    class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
  >
    <AppHeader v-if="!hideHeaderAndNav" />
    <!-- <Viewport :hide-header-and-nav="hideHeaderAndNav">
      <slot />
    </Viewport> -->

    <div
      id="viewport_mobile"
      class="min-h-0 flex-1 overscroll-contain overflow-y-auto p-6"
      :class="hideHeaderAndNav ? 'pb-6 pt-12' : 'pb-24 pt-20'"
    >
      <slot />
    </div>
    
    <AppBottonNav v-if="!hideHeaderAndNav" />

    <AppSideNav :mobile-nav-open="mobileNavOpen" />
  </main>
</template>
