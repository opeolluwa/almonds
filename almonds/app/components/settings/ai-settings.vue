<script setup lang="ts">
const ollamaUrl = ref("http://localhost:11434");
const defaultModel = ref("llama3");
const models = ["llama3", "codellama", "mistral", "gemma"];
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200">
          AI & Ollama
        </h2>
        <div class="flex items-center gap-1.5">
          <span class="size-2 rounded-full bg-emerald-400" />
          <span class="text-xs text-gray-500 dark:text-gray-400"
            >Connected</span
          >
        </div>
      </div>
      <div class="flex flex-col gap-4">
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
            >Server URL</label
          >
          <input
            v-model="ollamaUrl"
            type="text"
            class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent font-mono"
          >
        </div>
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-2"
            >Default Model</label
          >
          <div class="flex flex-col gap-1">
            <button
              v-for="m in models"
              :key="m"
              class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm transition-colors text-left"
              :class="
                defaultModel === m
                  ? 'bg-accent-50 dark:bg-accent-950 text-accent-700 dark:text-accent-300 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
              "
              @click="defaultModel = m"
            >
              <UIcon name="heroicons:cpu-chip" class="size-4 shrink-0" />
              {{ m }}
              <UIcon
                v-if="defaultModel === m"
                name="heroicons:check"
                class="size-4 ml-auto text-accent-500"
              />
            </button>
          </div>
        </div>
      </div>
      <div class="mt-5 flex justify-end">
        <button
          class="px-4 py-2 bg-accent-500 text-white text-sm font-medium rounded-lg hover:bg-accent-600 transition-colors"
        >
          Save
        </button>
      </div>
    </div>
  </div>
</template>
