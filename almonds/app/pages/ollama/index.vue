<script setup lang="ts">
definePageMeta({ layout: false });

interface Message {
  role: "user" | "assistant";
  content: string;
}

const prompt = ref("");
const messages = ref<Message[]>([
  { role: "assistant", content: "Hello! I'm your local Ollama assistant. How can I help you today?" },
]);

const models = ["llama3", "codellama", "mistral", "gemma"];
const selectedModel = ref("llama3");

function sendMessage() {
  if (!prompt.value.trim()) return;
  messages.value.push({ role: "user", content: prompt.value });
  messages.value.push({ role: "assistant", content: "This is a placeholder response. Connect to your local Ollama instance to get real responses." });
  prompt.value = "";
}
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="flex items-center justify-end mb-6">

        <div class="flex items-center gap-2">
          <span class="size-2 rounded-full bg-emerald-400"></span>
          <span class="text-xs text-gray-500 dark:text-gray-400">Connected</span>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 flex flex-col" style="height: calc(100vh - 180px)">
        <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-4">
          <div
            v-for="(msg, i) in messages"
            :key="i"
            class="flex gap-3"
            :class="msg.role === 'user' ? 'flex-row-reverse' : ''"
          >
            <div
              class="size-8 rounded-full flex items-center justify-center shrink-0"
              :class="msg.role === 'user' ? 'bg-violet-100 dark:bg-violet-900' : 'bg-gray-100 dark:bg-gray-700'"
            >
              <UIcon
                :name="msg.role === 'user' ? 'heroicons:user' : 'heroicons:cpu-chip'"
                class="size-4"
                :class="msg.role === 'user' ? 'text-violet-600 dark:text-violet-300' : 'text-gray-600 dark:text-gray-300'"
              />
            </div>
            <div
              class="max-w-[75%] rounded-lg px-4 py-2.5 text-sm"
              :class="msg.role === 'user' ? 'bg-violet-600 text-white' : 'bg-gray-50 dark:bg-gray-700 text-gray-700 dark:text-gray-200'"
            >
              {{ msg.content }}
            </div>
          </div>
        </div>

        <div class="border-t border-gray-100 dark:border-gray-700 p-4">
          <div class="flex items-center gap-3">
            <input
              v-model="prompt"
              type="text"
              placeholder="Ask Ollama something..."
              class="flex-1 bg-gray-50 dark:bg-gray-700 rounded-lg px-4 py-2.5 text-sm text-gray-700 dark:text-gray-200 border-none outline-none focus:ring-2 focus:ring-violet-300 dark:focus:ring-violet-600 placeholder-gray-400 dark:placeholder-gray-500"
              @keydown.enter="sendMessage"
            />
            <button
              class="p-2.5 bg-violet-600 text-white rounded-lg hover:bg-violet-700 transition-colors"
              @click="sendMessage"
            >
              <UIcon name="heroicons:paper-airplane" class="size-4" />
            </button>
          </div>
        </div>
      </div>
    </template>

    <template #side_content>
      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">Model</h2>
      <div class="flex flex-col gap-1 mb-6">
        <button
          v-for="model in models"
          :key="model"
          class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm transition-colors"
          :class="selectedModel === model ? 'bg-violet-50 dark:bg-violet-950 text-violet-700 dark:text-violet-300 font-medium' : 'text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800'"
          @click="selectedModel = model"
        >
          <UIcon name="heroicons:cpu-chip" class="size-4" />
          {{ model }}
        </button>
      </div>

      <USeparator class="my-4" />

      <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">History</h2>
      <div class="flex flex-col gap-2">
        <div class="py-2 px-3 rounded-lg bg-gray-50 dark:bg-gray-800 text-sm text-gray-600 dark:text-gray-400 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700">
          <p class="font-medium text-gray-700 dark:text-gray-300 truncate">Current conversation</p>
          <p class="text-xs text-gray-400">Just now</p>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
