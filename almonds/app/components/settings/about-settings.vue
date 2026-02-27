<script setup lang="ts">
import { platform } from "@tauri-apps/plugin-os";
import { getName, getVersion, getTauriVersion } from "@tauri-apps/api/app";

const tauriVersion = await getTauriVersion();
const appVersion = await getVersion();
const currentPlatform = platform();
const appName = await getName();

const info = [
  { label: "Version", value: appVersion, mono: true },
  { label: "Build", value: tauriVersion, mono: true },
  { label: "License", value: "MIT", mono: false },
  { label: "Platform", value: currentPlatform, mono: false },
];
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <div class="flex items-center gap-3 mb-5">
        <div
          class="size-10 bg-accent-100 dark:bg-accent-950 rounded-lg flex items-center justify-center"
        >
          <UIcon
            name="heroicons:sparkles"
            class="size-5 text-accent-600 dark:text-accent-400"
          />
        </div>
        <div>
          <p class="text-sm font-semibold text-gray-800 dark:text-gray-100">
            {{ appName }}
          </p>
          <p class="text-xs text-gray-400">Your personal productivity suite</p>
        </div>
      </div>
      <div
        class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700 mb-5"
      >
        <div
          v-for="item in info"
          :key="item.label"
          class="flex items-center justify-between py-3"
        >
          <span class="text-sm text-gray-500 dark:text-gray-400">{{
            item.label
          }}</span>
          <span
            class="text-sm text-gray-700 dark:text-gray-200"
            :class="item.mono ? 'font-mono' : ''"
            >{{ item.value }}</span
          >
        </div>
      </div>
      <button
        class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 hover:text-accent-600 dark:hover:text-accent-400 transition-colors"
      >
        <UIcon name="heroicons:arrow-path" class="size-4" />
        Check for updates
      </button>
    </div>
  </div>
</template>
