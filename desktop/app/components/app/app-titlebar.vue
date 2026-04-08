<script setup lang="ts">
import { ref, computed } from "vue";
import { useOnline, useEventListener } from "@vueuse/core";
import { platform } from "@tauri-apps/plugin-os";
import { getCurrentWindow } from "@tauri-apps/api/window";

const online = useOnline();
const router = useRouter();
const colorMode = useColorMode();
const { searchQuery, isOpen } = useAppSearch();

const appWindow = getCurrentWindow();
const searchInputRef = ref<HTMLInputElement | null>(null);

const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});
const themeIcon = computed(() =>
  isDark.value ? "heroicons:sun" : "heroicons:moon",
);
const themeLabel = computed(() => (isDark.value ? "Light mode" : "Dark mode"));
const internetStatusColor = computed(() =>
  online.value ? "success" : "error",
);

function onSearchInput(val: string) {
  searchQuery.value = val;
  isOpen.value = val.trim().length > 0;
}

const syncing = ref(false);

const currentPlatform = platform();
const isMacOS = computed(() => {
  return currentPlatform.toLowerCase() === "macos";
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
  <div class="titlebar grid grid-cls-12" data-tauri-drag-region>
    <!-- mac os controls-->
    <div v-if="isMacOS" class="traffic-lights col-span-1">
      <UTooltip text="Close">
        <span class="btn close" @click="appWindow.close()" />
      </UTooltip>
      <UTooltip text="Minimize">
        <span class="btn minimize" @click="appWindow.minimize()" />
      </UTooltip>
      <UTooltip text="Maximize">
        <span class="btn maximize" @click="appWindow.toggleMaximize()" />
      </UTooltip>
    </div>

    <!-- Windows controls -->
    <div v-else class="controls ml-12">
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

      <UTooltip text="Maximize">
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="lucide:maximize"
          aria-label="Maximize"
          @click="appWindow.maximize()"
        />
      </UTooltip>

      <UTooltip text="Close">
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:x-mark"
          aria-label="Close"
          @click="appWindow.close()"
        />
      </UTooltip>
    </div>

    <!-- Back & forward button -->
    <div
      class="col-col-end-3 flex items-center justify-center -gap-x-1.25 ml-16"
    >
      <UTooltip text="Syncing data">
        <UButton
          size="sm"
          color="neutral"
          :loading="syncing"
          variant="ghost"
          icon="heroicons:cloud-arrow-up"
          aria-label="Switch workspace"
        />
      </UTooltip>

      <UTooltip :text="`Go back (${isMacOS ? '⌘' : 'Ctrl'}+[)`">
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:chevron-left"
          aria-label="Go back"
          @click="router.back()"
        />
      </UTooltip>

      <UTooltip :text="`Go forward (${isMacOS ? '⌘' : 'Ctrl'}+])`">
        <UButton
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:chevron-right"
          aria-label="Go forward"
          @click="router.forward()"
        />
      </UTooltip>
    </div>

    <!-- Search -->
    <div class="col-span-4 mx-auto w-full max-w-sm relative">
      <div
        class="flex items-center gap-2 px-3 py-1.5 rounded-full border transition-colors bg-gray-100 dark:bg-gray-800 border-gray-200 dark:border-gray-700 focus-within:border-accent-400 dark:focus-within:border-accent-500"
      >
        <UIcon
          name="heroicons:magnifying-glass"
          class="size-3.5 shrink-0 text-gray-400 dark:text-gray-500"
        />
        <input
          ref="searchInputRef"
          :value="searchQuery"
          placeholder="Search..."
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
          class="flex-1 min-w-0 bg-transparent outline-none text-sm text-gray-700 dark:text-gray-300 placeholder-gray-400 dark:placeholder-gray-500"
          @input="onSearchInput(($event.target as HTMLInputElement).value)"
          @keydown.escape="
            isOpen = false;
            searchInputRef?.blur();
          "
        >
        <kbd
          v-if="!searchQuery"
          class="hidden sm:inline-flex items-center gap-0.5 text-[10px] text-gray-400 dark:text-gray-500 font-mono select-none"
        >
          <span>{{ isMacOS ? "⌘" : "Ctrl" }}</span
          ><span>F</span>
        </kbd>
        <button
          v-if="searchQuery"
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          aria-label="Clear search"
          @click="
            onSearchInput('');
            searchInputRef?.focus();
          "
        >
          <UIcon name="heroicons:x-mark" class="size-3.5" />
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
          size="sm"
          color="neutral"
          class="cursor-pointer"
          variant="ghost"
          icon="heroicons:bell"
          aria-label="Notifications"
          @click="navigateTo('/notifications')"
        />
      </UTooltip>

      <UTooltip text="Open panel">
        <UButton
          class="flex md:hidden"
          size="sm"
          color="neutral"
          variant="ghost"
          icon="heroicons:bars-3-bottom-right"
          aria-label="Open panel"
        />
      </UTooltip>

      <UUser
        size="sm"
        class="cursor-pointer"
        :avatar="{
          src: 'https://i.pravatar.cc/150?u=john-doe',
        }"
        :chip="{
          color: internetStatusColor,
          position: 'top-right',
        }"
        @click="navigateTo('/settings?section=profile')"
      />
    </div>
  </div>
</template>

<style scoped>
.titlebar > * {
  cursor: pointer;
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
