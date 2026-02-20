<template>
  <section class="relative overflow-hidden px-6 pt-28 pb-36 text-center">
    <!-- Ambient glow (dark mode) -->
    <div
      class="pointer-events-none absolute inset-0 -z-10 flex justify-center"
      aria-hidden="true"
    >
      <div
        class="hidden dark:block h-[500px] w-[900px] rounded-full"
        style="
          background: radial-gradient(
            ellipse,
            rgba(208, 39, 82, 0.12) 0%,
            transparent 70%
          );
          filter: blur(40px);
          margin-top: -100px;
        "
      />
    </div>

    <!-- Badge -->
    <div
      class="inline-flex items-center gap-2 rounded-full px-4 py-1.5 text-sm mb-10 border bg-accent-500/10 text-accent-600 dark:text-accent-300 border-accent-200 dark:border-accent-500/35"
    >
      <span
        class="h-1.5 w-1.5 rounded-full bg-accent-500 dark:bg-accent-400 animate-pulse-glow"
      />
      Open Source · Free Forever · MIT License
    </div>

    <!-- Headline -->
    <h1
      class="mx-auto max-w-5xl text-5xl md:text-7xl font-black leading-[1.1] tracking-tight text-slate-900 dark:text-slate-100"
    >
      Your personal workspace,
      <span class="gradient-text"> reimagined.</span>
    </h1>

    <!-- Subtext -->
    <p
      class="mx-auto mt-7 max-w-2xl text-lg leading-relaxed text-slate-500 dark:text-slate-400"
    >
      Almonds unifies your notes, bookmarks, code snippets, tasks, and local AI
      conversations into one beautiful desktop workspace. Built for focus,
      designed for flow — no subscriptions, no cloud lock-in.
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
        <svg
          class="size-4"
          fill="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path
            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
          />
        </svg>
        View on GitHub
      </a>
    </div>

    <!-- Tech stack + Platform -->
    <div class="mt-14 flex flex-col items-center gap-8">
      <!-- Divider with "Available on" -->
      <div
        class="flex items-center gap-4 text-xs text-slate-400 dark:text-slate-600"
      >
        <div class="h-px w-16 bg-slate-200 dark:bg-white/10" />
        <span class="font-medium uppercase tracking-widest">Available on</span>
        <div class="h-px w-16 bg-slate-200 dark:bg-white/10" />
      </div>

      <!-- Platform support -->
      <div class="flex items-center justify-center gap-10">
        <span
          class="flex flex-col items-center gap-2 text-slate-500 dark:text-slate-400"
        >
          <UIcon name="i-simple-icons-apple" class="size-8" />
          <span class="text-xs font-medium">macOS</span>
        </span>
        <span
          class="flex flex-col items-center gap-2 text-slate-500 dark:text-slate-400"
        >
          <UIcon name="i-simple-icons-windows" class="size-8" />
          <span class="text-xs font-medium">Windows</span>
        </span>
        <span
          class="flex flex-col items-center gap-2 text-slate-500 dark:text-slate-400"
        >
          <UIcon name="i-simple-icons-linux" class="size-8" />
          <span class="text-xs font-medium">Linux</span>
        </span>
      </div>
    </div>
  </section>
</template>
