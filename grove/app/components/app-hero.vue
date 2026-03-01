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
      conversations into one beautiful desktop workspace.
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
        class="inline-flex items-center gap-2.5 rounded-full px-8 py-3.5 text-sm font-semibold transition-all duration-200 text-slate-700 dark:text-white border border-slate-200 dark:border-white/10 bg-white dark:bg-white/4 hover:bg-slate-50 dark:hover:bg-white/8 hover:border-slate-300 dark:hover:border-white/20"
      >
        <UIcon name="ri:github-fill" class="size-5" />

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
