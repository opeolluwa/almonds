<script setup lang="ts">
import { useAlarmScheduler } from "~/composables/useAlarmScheduler";
import { useWorkspaceSetup } from "./composables/useWorkspaceSetup";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

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

  // when using `"withGlobalTauri": true`, you may use
  // const { isPermissionGranted, requestPermission, sendNotification, } = window.__TAURI__.notification;

  // Do you have permission to send a notification?
  let permissionGranted = await isPermissionGranted();

  // If not we need to request it
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }

  // Once permission has been granted we can send the notification
  if (permissionGranted) {
    sendNotification({ title: "Tauri", body: "Tauri is awesome!" });
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
