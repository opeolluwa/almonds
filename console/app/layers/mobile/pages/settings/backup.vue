<script setup lang="ts">
type BackupProvider = "local" | "cloud" | "self-hosted";

const options: {
  key: BackupProvider;
  label: string;
  desc: string;
  icon: string;
}[] = [
  {
    key: "local",
    label: "Local only",
    desc: "Data stays on this device, no sync.",
    icon: "heroicons:computer-desktop",
  },
  {
    key: "cloud",
    label: "Almond Cloud",
    desc: "Sync across devices via Almond Cloud.",
    icon: "heroicons:cloud",
  },
  {
    key: "self-hosted",
    label: "Self Hosted",
    desc: "Use your own server to sync data.",
    icon: "heroicons:server",
  },
];

const backupStore = useBackupSettingsStore();

onMounted(() => backupStore.init());
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-1">
        Backup & Sync
      </h2>
      <p class="text-xs text-gray-400 mb-4">
        Choose where your data is stored and synced.
      </p>

      <!-- Provider options -->
      <div class="flex flex-col gap-2 mb-5">
        <button
          v-for="opt in options"
          :key="opt.key"
          class="flex items-start gap-3 p-3 rounded-lg border transition-colors text-left"
          :class="
            backupStore.provider === opt.key
              ? 'border-primary-400 bg-primary-50 dark:bg-primary-950 dark:border-primary-600'
              : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'
          "
          @click="backupStore.provider = opt.key"
        >
          <div
            class="mt-0.5 size-8 rounded-md flex items-center justify-center shrink-0"
            :class="
              backupStore.provider === opt.key
                ? 'bg-primary-100 dark:bg-primary-900'
                : 'bg-gray-100 dark:bg-gray-700'
            "
          >
            <UIcon
              :name="opt.icon"
              class="size-4"
              :class="
                backupStore.provider === opt.key
                  ? 'text-primary-600 dark:text-primary-400'
                  : 'text-gray-500 dark:text-gray-400'
              "
            />
          </div>
          <div class="flex-1 min-w-0">
            <p
              class="text-sm font-medium"
              :class="
                backupStore.provider === opt.key
                  ? 'text-primary-700 dark:text-primary-300'
                  : 'text-gray-700 dark:text-gray-200'
              "
            >
              {{ opt.label }}
            </p>
            <p class="text-xs text-gray-400 mt-0.5">{{ opt.desc }}</p>
          </div>
          <UIcon
            v-if="backupStore.provider === opt.key"
            name="heroicons:check-circle"
            class="size-4 text-primary-500 shrink-0 mt-1"
          />
        </button>
      </div>

      <!-- Almond Cloud CTA -->
      <div
        v-if="backupStore.provider === 'cloud'"
        class="rounded-lg bg-primary-50 dark:bg-primary-950 border border-primary-100 dark:border-primary-800 p-4 flex items-center justify-between gap-4"
      >
        <div>
          <p class="text-sm font-medium text-primary-700 dark:text-primary-300">
            Almond Cloud
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
            Secure, encrypted sync. Plans start free.
          </p>
        </div>
        <NuxtLink
          to="/pricing"
          class="shrink-0 px-4 py-2 bg-primary-500 text-white text-sm font-medium rounded-lg hover:bg-primary-600 transition-colors flex items-center gap-1.5"
        >
          View plans
          <UIcon name="heroicons:arrow-top-right-on-square" class="size-3.5" />
        </NuxtLink>
      </div>

      <!-- Self Hosted config -->
      <div
        v-else-if="backupStore.provider === 'self-hosted'"
        class="flex flex-col gap-4"
      >
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
            >API Endpoint</label
          >
          <input
            v-model="backupStore.selfHostedApiUrl"
            type="url"
            placeholder="https://sync.example.com/api"
            class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-primary-300 dark:focus:ring-primary-600 focus:border-transparent font-mono"
          />
          <p class="text-xs text-gray-400 mt-1">
            Base URL of your self-hosted Almond sync server.
          </p>
        </div>
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
            >API Key</label
          >
          <input
            v-model="backupStore.selfHostedApiKey"
            type="password"
            placeholder="sk-••••••••••••"
            class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-primary-300 dark:focus:ring-primary-600 focus:border-transparent font-mono"
          />
        </div>
        <div class="flex items-center justify-between">
          <button
            class="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400 hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
          >
            <UIcon name="heroicons:signal" class="size-5" />
            Test connection
          </button>
          <button
            class="px-4 py-2 bg-primary-500 text-white text-sm font-medium rounded-lg hover:bg-primary-600 transition-colors"
            @click="backupStore.save()"
          >
            {{ !backupStore.savedConfigExists ? "Save" : "Update" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
