<template>
  <section class="relative overflow-hidden px-6 pt-28 pb-36 text-center">
    <!-- Ambient glow (dark mode) -->
    <div class="pointer-events-none absolute inset-0 -z-10 flex justify-center" aria-hidden="true">
      <div
        class="hidden dark:block h-[500px] w-[900px] rounded-full"
        style="
          background: radial-gradient(ellipse, rgba(208, 39, 82, 0.12) 0%, transparent 70%);
          filter: blur(40px);
          margin-top: -100px;
        "
      />
    </div>

    <!-- Badge -->
    <div
      class="inline-flex items-center gap-2 rounded-full px-4 py-1.5 text-sm mb-10 border bg-accent-500/10 text-accent-600 dark:text-accent-300 border-accent-200 dark:border-accent-500/35"
    >
      <span class="h-1.5 w-1.5 rounded-full bg-accent-500 dark:bg-accent-400 animate-pulse-glow" />
      Open Source · Free Forever · MIT License
    </div>

    <!-- Headline -->
    <h1 class="mx-auto max-w-5xl text-5xl md:text-7xl font-black leading-[1.1] tracking-tight text-slate-900 dark:text-slate-100">
      Your personal workspace,
      <span class="gradient-text"> reimagined.</span>
    </h1>

    <!-- Subtext -->
    <p class="mx-auto mt-7 max-w-2xl text-lg leading-relaxed text-slate-500 dark:text-slate-400">
      Almonds unifies your notes, bookmarks, code snippets, tasks, and local AI conversations into
      one beautiful desktop workspace. Built for focus, designed for flow — no subscriptions, no
      cloud lock-in.
    </p>

    <!-- CTAs -->
    <div class="mt-10 flex flex-wrap items-center justify-center gap-4">
      <a
        href="#download"
        class="inline-flex items-center gap-2.5 rounded-full px-8 py-3.5 text-sm font-semibold text-white bg-accent-500 hover:bg-accent-400 transition-all duration-200 glow-accent"
      >
        <UIcon name="i-heroicons-arrow-down-tray" class="size-4" />
        Download Now
      </a>

      <a
        href="https://github.com/opeolluwa/almonds"
        target="_blank"
        rel="noopener"
        class="inline-flex items-center gap-2.5 rounded-full px-8 py-3.5 text-sm font-semibold transition-all duration-200 text-slate-700 dark:text-white border border-slate-200 dark:border-white/10 bg-white dark:bg-white/[0.04] hover:bg-slate-50 dark:hover:bg-white/[0.08] hover:border-slate-300 dark:hover:border-white/20"
      >
        <svg class="size-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
        </svg>
        View on GitHub
      </a>
    </div>

    <!-- Platform support -->
    <div class="mt-10 flex items-center justify-center gap-8 text-xs text-slate-400 dark:text-slate-600">
      <span class="flex items-center gap-2">
        <!-- Apple brand SVG (not in heroicons) -->
        <svg class="size-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.8-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
        </svg>
        macOS
      </span>
      <span class="flex items-center gap-2">
        <!-- Windows brand SVG (not in heroicons) -->
        <svg class="size-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M3 5.557L9.938 4.63V11H3zM10.625 4.527L21 3v8h-10.375zM3 12.03h6.938v6.396L3 17.499zM10.625 12.098H21V21l-10.375-1.535z" />
        </svg>
        Windows
      </span>
      <span class="flex items-center gap-2">
        <UIcon name="i-heroicons-command-line" class="size-4" />
        Linux
      </span>
    </div>
  </section>
</template>
