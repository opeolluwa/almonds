<script setup lang="ts">
import {
  isPermissionGranted,
  requestPermission
} from "@tauri-apps/plugin-notification";
import { useAlarmScheduler } from "~/composables/useAlarmScheduler";
import { useWorkspaceSetup } from "./composables/useWorkspaceSetup";

const { init } = useAccentColor();
const { init: initFontSize } = useFontSize();
const { init: initDarkTheme } = useDarkTheme();
const { setupRequired, checkSetup, initializing } = useUserSetup();
const {
  setupRequired: workspaceSetupRequired,
  checkSetup: checkWorkspaceSetup,
  initializing: workspaceInitializing,
} = useWorkspaceSetup();

useAlarmScheduler();

onMounted(async () => {
  init();
  initFontSize();
  initDarkTheme();
  await checkSetup();
  await checkWorkspaceSetup();

  let permissionGranted = await isPermissionGranted();

  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }
});
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <AppNotification />
    <UserSetupModal v-if="setupRequired" />
    <WorkspaceSetupModal v-if="workspaceSetupRequired" />

    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-300"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <AppSplashScreen v-if="initializing || workspaceInitializing" />
    </Transition>
  </UApp>
</template>
