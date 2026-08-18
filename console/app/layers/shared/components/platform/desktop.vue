<script setup lang="ts">
import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/plugin-notification";
import { useAlarmScheduler } from "~/composables/useAlarmScheduler";
import { useWorkspaceSetup } from "~/composables/useWorkspaceSetup";
import { IS_WEB } from "~/plugins/lunar.client";
import "@domternal/theme";
const { init: initFontSize } = useFontSize();
const { init: initDarkTheme } = useDarkTheme();
const { checkSetup, initializing } = useUserSetup();
const { checkSetup: checkWorkspaceSetup, initializing: workspaceInitializing } =
  useWorkspaceSetup();

useAlarmScheduler();
const authenticated = ref(true);

const route = useRoute();
const isAuthRoute = computed(() => route.path.startsWith("/auth"));

const showWorkspaceLock = ref(false);

onMounted(async () => {
  try {
    initFontSize();
    initDarkTheme();
    await checkSetup();
    await checkWorkspaceSetup();

    const workspaceStore = useWorkspacesStore();
    await workspaceStore.fetchWorkspaces();

    if (workspaceStore.isCurrentWorkspaceLocked) {
      showWorkspaceLock.value = true;
    }

    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
  } catch (error) {
    console.error("[app] initialization failed", error);
  }
});
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <AppNotification />

    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-300"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <AppSplashScreen v-if="initializing || workspaceInitializing" />
    </Transition>
  </UApp>

  <Body>
    <UApp>
      <AppTitlebar
        v-if="!isAuthRoute || !IS_WEB"
        :authenticated="authenticated"
      />
    </UApp>
  </Body>
</template>

<style>
@reference "@/assets/css/main.css";
.scrollbar-config {
  @apply scrollbar-thumb-rounded-full scrollbar-w-[0.25px] scrollbar-corner-accent-400 scrollbar-h-20 scrollbar-track-rounded-full scrollbar-thin scrollbar-thumb-accent-600 scrollbar-track-transparent;
}

.scrollbar-config-dark {
  @apply scrollbar-thumb-rounded-full scrollbar-w-[0.25px] scrollbar-corner-gray-900 scrollbar-h-20 scrollbar-track-rounded-full scrollbar-thin scrollbar-thumb-gray-900 scrollbar-track-transparent;
}
</style>
