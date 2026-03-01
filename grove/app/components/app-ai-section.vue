<template>
  <section
    id="ai"
    class="px-6 py-24 border-t border-slate-100 dark:border-white/[0.04]"
  >
    <div class="mx-auto max-w-7xl">
      <div
        class="rounded-3xl overflow-hidden grid md:grid-cols-2 border border-accent-500/15 bg-slate-50 dark:bg-gradient-to-br dark:from-[#111825] dark:to-[#0f1620]"
      >
        <!-- Text side -->
        <div class="p-12 flex flex-col justify-center">
          <div
            class="inline-block rounded-full px-3 py-1 text-xs font-medium mb-6 w-fit bg-accent-500/10 text-accent-600 dark:text-accent-400 border border-accent-200 dark:border-accent-500/20"
          >
            Local AI · Privacy First
          </div>
          <h2 class="text-3xl md:text-4xl font-bold tracking-tight mb-5 text-slate-900 dark:text-slate-100">
            AI that runs <span class="gradient-text">on your machine.</span>
          </h2>
          <p class="text-sm leading-relaxed mb-8 text-slate-500 dark:text-slate-500">
            Almonds integrates natively with Ollama, letting you chat with powerful language models
            like Llama, Mistral, and Gemma — completely offline. Your conversations never leave your
            device.
          </p>
          <ul class="space-y-3">
            <li
              v-for="point in aiPoints"
              :key="point"
              class="flex items-start gap-3 text-sm text-slate-600 dark:text-slate-400"
            >
              <UIcon
                name="i-heroicons-check"
                class="size-4 mt-0.5 shrink-0 text-accent-500"
              />
              {{ point }}
            </li>
          </ul>
        </div>

        <!-- Visual side: mock chat UI -->
        <div class="relative p-12 flex items-center justify-center">
          <div
            class="absolute inset-0 pointer-events-none"
            style="
              background: radial-gradient(
                ellipse at 60% 50%,
                rgba(208, 39, 82, 0.08) 0%,
                transparent 70%
              );
            "
            aria-hidden="true"
          />
          <div class="relative w-full max-w-sm space-y-4">
            <div
              v-for="msg in aiMessages"
              :key="msg.id"
              class="rounded-xl px-4 py-3 text-sm"
              :class="
                msg.role === 'user'
                  ? 'ml-8 bg-accent-500/15 text-slate-800 dark:text-slate-100 border border-accent-500/20'
                  : 'mr-8 bg-white dark:bg-[#1a2235] text-slate-600 dark:text-slate-400 border border-slate-200 dark:border-white/[0.06]'
              "
            >
              <div
                class="text-xs mb-1.5 font-medium"
                :class="msg.role === 'user' ? 'text-accent-500 dark:text-accent-300' : 'text-slate-400 dark:text-slate-600'"
              >
                {{ msg.role === 'user' ? 'You' : 'Ollama' }}
              </div>
              {{ msg.text }}
            </div>
            <!-- Typing indicator -->
            <div
              class="rounded-xl px-4 py-3 flex items-center gap-1.5 w-fit bg-white dark:bg-[#1a2235] border border-slate-200 dark:border-white/[0.06]"
            >
              <span
                v-for="i in 3"
                :key="i"
                class="h-1.5 w-1.5 rounded-full bg-accent-500 animate-pulse-glow"
                :style="`animation-delay: ${(i - 1) * 0.2}s`"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const aiPoints = [
  'Runs entirely on your device via Ollama',
  'Supports Llama 3, Mistral, Gemma, and more',
  'No API keys, no subscriptions, no data sent to the cloud',
  'Context-aware responses using your workspace content',
]

const aiMessages = [
  { id: 1, role: 'user', text: 'Summarise my notes from this week' },
  {
    id: 2,
    role: 'assistant',
    text: 'You wrote 4 notes this week. Here\'s a quick summary: Project kickoff, side project ideas, and two research notes on Rust...',
  },
  { id: 3, role: 'user', text: 'What are my open todos?' },
]
</script>
