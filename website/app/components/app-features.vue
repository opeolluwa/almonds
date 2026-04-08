<template>
  <section
    id="features"
    class="px-6 py-24 border-t border-slate-100 dark:border-white/[0.04]"
  >
    <div class="mx-auto max-w-7xl">
      <!-- Section header -->
      <div class="text-center mb-20">
        <div
          class="inline-block rounded-full px-3 py-1 text-xs font-medium mb-4 bg-accent-500/10 text-accent-600 dark:text-accent-400 border border-accent-200 dark:border-accent-500/20"
        >
          Everything you need
        </div>
        <h2 class="text-4xl md:text-5xl font-bold tracking-tight text-slate-900 dark:text-slate-100">
          One workspace.<br />
          <span class="gradient-text">Zero distractions.</span>
        </h2>
        <p class="mt-5 text-base mx-auto max-w-xl text-slate-500 dark:text-slate-500">
          Stop juggling a dozen apps. Almonds brings every tool you need into a single,
          distraction-free desktop experience.
        </p>
      </div>

      <!-- Feature grid -->
      <div class="grid md:grid-cols-3 gap-5">
        <div
          v-for="feature in features"
          :key="feature.title"
          class="rounded-2xl p-7 group transition-all duration-300 card-border bg-slate-50 dark:bg-[#111825]"
        >
          <div
            class="mb-5 inline-flex h-11 w-11 items-center justify-center rounded-xl bg-accent-500/[0.12]"
          >
            <UIcon :name="feature.icon" class="size-5 text-accent-500 dark:text-accent-400" />
          </div>
          <h3 class="text-base font-semibold mb-2 text-slate-900 dark:text-slate-100">
            {{ feature.title }}
          </h3>
          <p class="text-sm leading-relaxed text-slate-500 dark:text-slate-500">
            {{ feature.description }}
          </p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const features = [
  {
    icon: 'i-heroicons-document-text',
    title: 'Notes',
    description:
      'Rich-text notes with tagging, full-text search, and a clean distraction-free editor. Your thoughts, organized.',
  },
  {
    icon: 'i-heroicons-bookmark',
    title: 'Bookmarks',
    description:
      'Save web links with automatic favicon fetch and folder organization. Never lose a useful resource again.',
  },
  {
    icon: 'i-heroicons-code-bracket',
    title: 'Snippets',
    description:
      'Store reusable code with syntax highlighting for 100+ languages. Copy to clipboard in one click.',
  },
  {
    icon: 'i-heroicons-check-circle',
    title: 'Todos',
    description:
      'Simple, fast task tracking. Add due dates, set priorities, and stay on top of what matters.',
  },
  {
    icon: 'i-heroicons-pencil-square',
    title: 'Scratch Pad',
    description:
      'A persistent quick-capture space. Jot down thoughts without ceremony — it auto-saves as you type.',
  },
  {
    icon: 'i-heroicons-photo',
    title: 'Moodboard',
    description:
      'Collect visual inspiration in a drag-and-drop image layout. Perfect for creative projects and design work.',
  },
]
</script>
