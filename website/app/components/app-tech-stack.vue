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
                <UIcon name="ri:github-fill" class="size-5" />
          View on GitHub
        </a>
        <a
          href="#"
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
