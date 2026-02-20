<template>
  <footer class="px-6 py-12 border-t border-slate-100 dark:border-white/5">
    <div class="mx-auto max-w-7xl flex flex-col md:flex-row items-center justify-between gap-6">
      <div class="flex items-center gap-3">
        <span class="font-bold tracking-tight gradient-text">almonds</span>
        <span class="text-xs text-slate-400 dark:text-gray-700">
          · Personal Productivity · MIT License
        </span>
      </div>

      <div class="flex items-center gap-6 text-sm text-slate-400 dark:text-slate-600">
        <a
          href="https://github.com/opeolluwa/almonds"
          target="_blank"
          rel="noopener"
          class="hover:text-slate-900 dark:hover:text-white transition-colors"
        >
          GitHub
        </a>
        <a
          href="https://github.com/opeolluwa/almonds/releases"
          target="_blank"
          rel="noopener"
          class="hover:text-slate-900 dark:hover:text-white transition-colors"
        >
          Releases
        </a>
        <a
          href="https://github.com/opeolluwa/almonds/issues"
          target="_blank"
          rel="noopener"
          class="hover:text-slate-900 dark:hover:text-white transition-colors"
        >
          Feedback
        </a>
      </div>

      <p class="text-xs text-slate-300 dark:text-gray-800 flex items-center gap-1">
        &copy; {{ new Date().getFullYear() }} almonds. Built with
        <UIcon name="i-heroicons-heart" class="size-3 text-accent-500" />
      </p>
    </div>
  </footer>
</template>
