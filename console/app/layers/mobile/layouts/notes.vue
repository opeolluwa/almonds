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

function isActive(path: string): boolean {
  if (path === "/") return route.path === "/";
  return route.path.startsWith(path);
}
</script>

<template>
  <main
    id="default_layout_mobile"
    class="flex h-dvh flex-col overflow-hidden bg-gray-50 dark:bg-app-dark-800"
  >
    <AppHeader v-if="!hideHeaderAndNav" />
    <div
      id="viewport_mobile"
      class="min-h-0 flex-1 overscroll-contain overflow-y-auto p-6"
      :class="hideHeaderAndNav ? 'pb-6 pt-12' : 'pb-24 pt-12.5'"
    >
      <slot />
    </div>

    <AppBottonNav v-if="!hideHeaderAndNav" />
    <EditorMenu  />
    <USlideover
      v-model:open="mobileNavOpen"
      side="left"
      :ui="{ content: 'max-w-64' }"
    >
      <template #content>
        <div class="flex flex-col h-full bg-white dark:bg-app-dark-900">
          <div class="shrink-0" style="height: env(safe-area-inset-top)" />

          <div
            class="flex items-center justify-between px-4 py-4 border-b border-gray-200 dark:border-gray-800 shrink-0"
          >
            <UUser
              :name="preferenceStore.fullName"
              :description="preferenceStore.preference?.email"
              :avatar="{ icon: 'i-lucide-user' }"
              class="min-w-0 flex-1 truncate"
            />
            <UButton
              size="sm"
              color="neutral"
              variant="ghost"
              icon="heroicons:x-mark"
              aria-label="Close menu"
              @click="mobileNavOpen = false"
            />
          </div>

          <nav
            class="flex flex-col gap-0.5 px-2 py-2 flex-1 overflow-y-auto scrollbar-config"
          >
            <NuxtLink
              v-for="r in primaryRoutes"
              :key="r.name"
              :to="r.path"
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
              :class="
                isActive(r.path)
                  ? 'bg-primary-50 dark:bg-primary-950 text-primary-700 dark:text-primary-300 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
              "
              @click="mobileNavOpen = false"
            >
              <UIcon
                :name="isActive(r.path) ? r.activeIcon : r.icon"
                class="size-4 shrink-0"
              />
              {{ r.name }}
            </NuxtLink>
          </nav>

          <div class="flex flex-col gap-0.5 px-2 pb-4 shrink-0">
            <USeparator class="mx-1 mb-2" />
            <button
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 w-full"
              @click="isDark = !isDark"
            >
              <UIcon :name="themeIcon" class="size-4 shrink-0" />
              {{ themeLabel }}
            </button>
            <NuxtLink
              v-for="r in secondaryRoutes"
              :key="r.name"
              :to="r.path"
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors"
              :class="
                isActive(r.path)
                  ? 'bg-primary-50 dark:bg-primary-950 text-primary-700 dark:text-primary-300 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
              "
              @click="mobileNavOpen = false"
            >
              <UIcon
                :name="isActive(r.path) ? r.activeIcon : r.icon"
                class="size-4 shrink-0"
              />
              {{ r.name }}
            </NuxtLink>
            <button
              class="flex items-center gap-3 py-2 px-3 text-sm cursor-pointer rounded-lg transition-colors text-red-500 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-950/40 w-full"
              @click="logout"
            >
              <UIcon
                name="heroicons:arrow-right-start-on-rectangle"
                class="size-4 shrink-0"
              />
              Logout
            </button>
          </div>
        </div>
      </template>
    </USlideover>
  </main>
</template>
