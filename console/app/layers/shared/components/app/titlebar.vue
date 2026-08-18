<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";
import { useEventListener } from "@vueuse/core";
import { computed, onMounted, watch } from "vue";
import { IS_WEB } from "~/plugins/lunar.client";

const props = defineProps<{ authenticated?: boolean }>();
const authStore = useAuthStore();
const syncQueueStore = useSyncQueueStore();
const isOnline = computed(() => syncQueueStore.isOnline);
const runningSync = computed(() => syncQueueStore.runningSync);
const router = useRouter();
const hideAuthGated = computed(
  () => props.authenticated && !authStore.isAuthenticated && !authStore.isGuest,
);
const colorMode = useColorMode();
const { searchQuery, isOpen } = useAppSearch();
const appWindow = getCurrentWindow();
const searchInputRef = ref<HTMLInputElement | null>(null);
const currentPlatform = platform();
const { toggleMobileNav } = useMobileNav();

const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));
const isMacOS = computed(() => {
  return currentPlatform.toLowerCase() === "macos";
});
const syncIcon = computed(() =>
  runningSync.value ? "heroicons:arrow-path" : "heroicons:cloud-arrow-up",
);
const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});
const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);

const userMenuItems = computed(() => [
  {
    label: authStore.isGuest
      ? "Guest mode"
      : useUserPreferenceStore().fullName || "Account",
    icon: authStore.isGuest ? "heroicons:user" : "heroicons:user-circle",
    class: "text-gray-500 dark:text-gray-400 pointer-events-none",
    onSelect: () => {},
  },
  {
    label: authStore.isGuest ? "Exit" : "Logout",
    icon: "heroicons:arrow-right-start-on-rectangle",
    class: "text-red-500 dark:text-red-400",
    onSelect: () => {
      authStore.clearSession();
      authStore.exitGuestMode();
      navigateTo("/auth/login");
    },
  },
]);

function onSearchInput(val: string) {
  searchQuery.value = val;
  isOpen.value = val.trim().length > 0;
}

onMounted(() => {
  if (isOnline.value) syncQueueStore.runSync();
});

watch(isOnline, (online) => {
  if (online) syncQueueStore.runSync();
});

useEventListener("keydown", (e: KeyboardEvent) => {
  const mod = isMacOS.value ? e.metaKey : e.ctrlKey;
  if (!mod) return;

  if (e.key === "f") {
    e.preventDefault();
    searchInputRef.value?.focus();
    searchInputRef.value?.select();
  } else if (e.key === "<" || e.key === "[") {
    e.preventDefault();
    router.back();
  } else if (e.key === ">" || e.key === "]") {
    e.preventDefault();
    router.forward();
  }
});
</script>

<template>
  <div
    class="titlebar flex items-center gap-2 px-2 h-12"
    data-tauri-drag-region
    :class="{ 'rounded-t-xl': !IS_WEB }"
  >
    <!-- mobile nav toggle -->
    <div v-if="authenticated">
      <UButton
        size="lg"
        color="neutral"
        variant="ghost"
        class="md:hidden"
        icon="heroicons:bars-3"
        aria-label="Open menu"
        @click="toggleMobileNav"
      />
    </div>

    <!-- mac os controls-->
    <div v-if="isMacOS && !IS_WEB" class="traffic-lights shrink-0">
      <span class="btn close" @click="appWindow.close()" />
      <span class="btn minimize" @click="appWindow.minimize()" />
      <span class="btn maximize" @click="appWindow.toggleMaximize()" />
    </div>

    <!-- Windows controls -->
    <div v-else-if="!isMacOS && IS_WEB" class="flex items-center shrink-0">
      <UTooltip text="Minimize">
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:minus"
          aria-label="Minimize"
          @click="appWindow.minimize()"
        />
      </UTooltip>

      <!-- web control/mobile -->

      <AppButton
        size="xl"
        color="neutral"
        variant="ghost"
        icon="lucide:maximize"
        aria-label="Maximize"
        @click="appWindow.maximize()"
      />

      <UTooltip text="Close">
        <UButton
          size="xl"
          color="neutral"
          variant="ghost"
          icon="heroicons:x-mark"
          aria-label="Close"
          @click="appWindow.close()"
        />
      </UTooltip>
    </div>

    <div class="inline-flex items-center shrink-0">
      <UButton
        size="sm"
        color="neutral"
        variant="ghost"
        class="cursor-pointer hidden md:block"
        icon="heroicons:chevron-left"
        aria-label="Go back"
        @click="router.back()"
      />
      <UButton
        size="sm"
        color="neutral"
        variant="ghost"
        class="cursor-pointer hidden md:block"
        icon="heroicons:chevron-right"
        aria-label="Go forward"
        @click="router.forward()"
      />
    </div>

    <WorkspaceSelect
      v-if="!hideAuthGated"
      class="cursor-pointer hidden md:block"
    />

    <UButton
      size="sm"
      variant="ghost"
      :disabled="!isOnline"
      aria-label="Sync data"
      class="hidden"
      @click="syncQueueStore.runSync()"
    >
      <template #leading>
        <UIcon
          :name="syncIcon"
          :class="['size-4', runningSync && 'animate-spin']"
        />
      </template>
    </UButton>

    <!-- Search -->
    <div
      class="flex-1 min-w-0 flex items-center md:max-w-sm md:mx-auto md:relative"
    >
      <div
        v-if="!hideAuthGated"
        class="flex items-center gap-2 h-9 px-3 w-full rounded-md transition-colors bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 focus-within:border-accent-400 dark:focus-within:border-accent-500"
      >
        <UIcon
          name="heroicons:magnifying-glass"
          class="size-5 shrink-0 text-gray-400 dark:text-gray-500"
        />
        <input
          ref="searchInputRef"
          :value="searchQuery"
          placeholder="Search..."
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
          class="flex-1 w-full md:min-w-0 outline-none text-sm text-gray-700 dark:text-gray-300 placeholder-gray-400 dark:placeholder-gray-500"
          @input="onSearchInput(($event.target as HTMLInputElement).value)"
          @keydown.escape="
            isOpen = false;
            searchInputRef?.blur();
          "
        />
        <kbd
          v-if="!searchQuery"
          class="hidden sm:inline-flex items-center gap-0.5 text-[10px] text-gray-400 dark:text-gray-500 font-mono select-none"
        />
        <button
          v-if="searchQuery"
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          aria-label="Clear search"
          @click="
            onSearchInput('');
            searchInputRef?.focus();
          "
        >
          <UIcon name="heroicons:x-mark" class="size-5" />
        </button>
      </div>

      <AppGlobalSearch
        v-if="isOpen"
        @close="
          isOpen = false;
          searchQuery = '';
        "
      />
    </div>

    <!-- Right actions -->
    <div class="flex items-center gap-1 ml-auto">
      <UTooltip :text="themeLabel">
        <UButton
          size="sm"
          color="neutral"
          class="cursor-pointer"
          variant="ghost"
          :icon="themeIcon"
          :aria-label="themeLabel"
          @click="isDark = !isDark"
        />
      </UTooltip>

      <UTooltip text="Notifications">
        <UButton
          v-if="!hideAuthGated"
          size="sm"
          color="neutral"
          class="cursor-pointer"
          variant="ghost"
          icon="heroicons:bell"
          aria-label="Notifications"
          @click="() => navigateTo('/notifications')"
        />
      </UTooltip>

      <div v-if="!hideAuthGated" class="items-center gap-1.5 flex">
        <UDropdownMenu
          :items="userMenuItems"
          :ui="{
            content:
              'min-w-48 rounded-xl shadow-xl border border-gray-100 dark:border-gray-800 py-1.5',
            item: 'rounded-lg mx-1 px-3 py-2 gap-2.5 text-sm transition-colors duration-150',
            separator: 'my-1 mx-2',
          }"
        >
          <UUser
            size="sm"
            class="cursor-pointer"
            :avatar="
              authStore.isGuest || !authStore.isAuthenticated
                ? { icon: 'heroicons:user' }
                : { src: 'https://i.pravatar.cc/150?u=john-doe' }
            "
          />
        </UDropdownMenu>
      </div>
    </div>
  </div>
</template>

<style scoped>
.titlebar > * {
  cursor: pointer;
  border: none;
  outline: none;
  box-shadow: none;
}

.traffic-lights {
  display: flex;
  gap: 8px;
  padding: 10px;
}

.btn {
  width: 11px;
  height: 12px;
  border-radius: 50%;
  display: inline-block;
  cursor: pointer;
}

/* Colors */
.close {
  background: #ff5f57;
}

.minimize {
  background: #ffbd2e;
}

.maximize {
  background: #28c840;
}

/* Optional hover icons */
.traffic-lights:hover .btn::after {
  content: "";
  display: block;
  width: 100%;
  height: 100%;
  background-size: 8px;
  background-repeat: no-repeat;
  background-position: center;
}

.close:hover::after {
  content: "✕";
  font-size: 8px;
  color: #4d0000;
  text-align: center;
}

.minimize:hover::after {
  content: "–";
  font-size: 10px;
  color: #664d00;
  text-align: center;
}

.maximize:hover::after {
  content: "+";
  font-size: 10px;
  color: #003300;
  text-align: center;
}
</style>
