<script setup lang="ts">
import { useAlarmScheduler } from "~/composables/useAlarmScheduler";

const { init } = useAccentColor();
const { init: initFontSize } = useFontSize();
const { init: initDarkTheme } = useDarkTheme();
const { setupRequired, checkSetup, initializing } = useUserSetup();

useAlarmScheduler();

onMounted(async () => {
  init();
  initFontSize();
  initDarkTheme();
  await checkSetup();
});
</script>

<template>
  <UApp>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <AppNotification />
    <UserSetupModal v-if="setupRequired" />

    <Transition
      enter-active-class="transition-opacity duration-200"
      leave-active-class="transition-opacity duration-300"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <AppSplashScreen v-if="initializing" />
    </Transition>
  </UApp>
</template>
