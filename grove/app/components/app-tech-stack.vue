<template>
  <section
    id="developers"
    class="px-6 py-24 border-t border-slate-100 dark:border-white/[0.04]"
  >
    <div class="mx-auto max-w-7xl">
      <div class="text-center mb-12">
        <p class="text-xs font-semibold uppercase tracking-widest text-accent-500 dark:text-accent-400 mb-3">
          Open Source
        </p>
        <h2 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white mb-4">
          Built for developers, by developers
        </h2>
        <p class="text-slate-500 dark:text-slate-400 max-w-xl mx-auto text-sm leading-relaxed">
          Almonds is fully open source and MIT licensed. Contribute, fork, or self-host — the entire stack is built on best-in-class open source technology.
        </p>
      </div>

      <!-- Tech stack pills -->
      <div class="flex flex-wrap items-center justify-center gap-3 mb-12">
        <div
          v-for="tech in techStack"
          :key="tech.name"
          class="flex items-center gap-2.5 rounded-xl px-5 py-3 text-sm font-medium transition-all duration-200 card-border bg-slate-50 dark:bg-[#111825] text-slate-600 dark:text-slate-400"
        >
          <UIcon :name="tech.icon" class="size-4 text-accent-500 dark:text-accent-400" />
          {{ tech.name }}
        </div>
      </div>

      <!-- Dev links -->
      <div class="flex flex-wrap items-center justify-center gap-4">
        <a
          href="https://github.com/opeolluwa/almonds"
          target="_blank"
          rel="noopener"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-full text-sm font-semibold bg-slate-900 dark:bg-white text-white dark:text-slate-900 hover:opacity-90 transition-opacity duration-150"
        >
          <svg class="size-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
          </svg>
          View on GitHub
        </a>
        <a
          href="https://github.com/opeolluwa/almonds/blob/main/README.md"
          target="_blank"
          rel="noopener"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-full text-sm font-semibold border border-slate-200 dark:border-white/10 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-white/5 transition-colors duration-150"
        >
          <UIcon name="i-heroicons-book-open" class="size-4" />
          Read the docs
        </a>
        <a
          href="https://github.com/opeolluwa/almonds/issues"
          target="_blank"
          rel="noopener"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-full text-sm font-semibold border border-slate-200 dark:border-white/10 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-white/5 transition-colors duration-150"
        >
          <UIcon name="i-heroicons-bug-ant" class="size-4" />
          Report an issue
        </a>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const techStack = [
  { name: 'Nuxt 4', icon: 'i-heroicons-sparkles' },
  { name: 'Vue 3', icon: 'i-heroicons-cube-transparent' },
  { name: 'Tauri v2', icon: 'i-heroicons-cpu-chip' },
  { name: 'Tailwind CSS', icon: 'i-heroicons-paint-brush' },
  { name: 'Pinia', icon: 'i-heroicons-circle-stack' },
  { name: 'SQLite', icon: 'i-heroicons-server' },
  { name: 'Ollama', icon: 'i-heroicons-chat-bubble-left-right' },
]
</script>
