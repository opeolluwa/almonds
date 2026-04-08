<template>
  <section id="download" class="px-6 py-24">
    <div class="mx-auto max-w-7xl">
      <div
        class="rounded-3xl px-8 py-20 text-center relative overflow-hidden border border-accent-500/20 bg-gradient-to-br from-slate-50 via-white to-slate-50 dark:from-[#1a0a12] dark:via-[#1c1028] dark:to-[#0d1220]"
      >
        <!-- Glow -->
        <div
          class="pointer-events-none absolute inset-0 -z-10 flex justify-center items-center"
          aria-hidden="true"
        >
          <div
            class="h-[400px] w-[600px] rounded-full"
            style="
              background: radial-gradient(ellipse, rgba(208, 39, 82, 0.12) 0%, transparent 70%);
              filter: blur(60px);
            "
          />
        </div>

        <div
          class="inline-block rounded-full px-3 py-1 text-xs font-medium mb-6 bg-accent-500/10 text-accent-600 dark:text-accent-400 border border-accent-200 dark:border-accent-500/25"
        >
          Available Now
        </div>

        <h2 class="text-4xl md:text-6xl font-black tracking-tight mb-5 text-slate-900 dark:text-slate-100">
          Your workspace awaits.
        </h2>
        <p class="text-lg mx-auto max-w-md mb-12 text-slate-500 dark:text-slate-500">
          Download Almonds for free. No account required. Your data stays on your device — always.
        </p>

        <div class="flex flex-wrap items-center justify-center gap-4 mb-10">
          <a
            v-for="platform in downloadPlatforms"
            :key="platform.name"
            href="https://github.com/opeolluwa/almonds/releases"
            target="_blank"
            rel="noopener"
            class="inline-flex items-center gap-3 rounded-xl px-7 py-4 font-semibold text-sm transition-all duration-200"
            :class="
              platform.primary
                ? 'bg-accent-500 hover:bg-accent-400 text-white'
                : 'bg-white dark:bg-white/5 text-slate-700 dark:text-slate-100 border border-slate-200 dark:border-white/10 hover:border-slate-300 dark:hover:border-white/20 hover:bg-slate-50 dark:hover:bg-white/[0.08]'
            "
          >
            <UIcon name="i-heroicons-arrow-down-tray" class="size-5" />
            {{ platform.name }}
          </a>
        </div>

        <p class="text-xs text-slate-400 dark:text-gray-700">
          macOS · Windows · Linux &nbsp;·&nbsp; Free, open source, MIT licensed
        </p>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const downloadPlatforms = [
  { name: 'Download for macOS', primary: true },
  { name: 'Download for Windows', primary: false },
  { name: 'Download for Linux', primary: false },
]
</script>
