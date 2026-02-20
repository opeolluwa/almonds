<template>
  <section
    id="stack"
    class="px-6 py-24 border-t border-slate-100 dark:border-white/[0.04]"
  >
    <div class="mx-auto max-w-7xl text-center">
      <p class="text-sm font-medium mb-8 text-slate-400 dark:text-gray-700">
        Built with best-in-class open source technology
      </p>
      <div class="flex flex-wrap items-center justify-center gap-4">
        <div
          v-for="tech in techStack"
          :key="tech.name"
          class="flex items-center gap-2.5 rounded-xl px-5 py-3 text-sm font-medium transition-all duration-200 card-border bg-slate-50 dark:bg-[#111825] text-slate-600 dark:text-slate-400"
        >
          <UIcon :name="tech.icon" class="size-4 text-accent-500 dark:text-accent-400" />
          {{ tech.name }}
        </div>
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
