<template>
  <div class="min-h-screen" style="background-color: #0d1220; color: #f1f5f9;">

    <!-- ─── Navigation ─────────────────────────────────────────── -->
    <nav
      class="sticky top-0 z-50 backdrop-blur-md"
      style="background-color: rgba(13,18,32,0.85); border-bottom: 1px solid rgba(255,255,255,0.05);"
    >
      <div class="mx-auto max-w-7xl px-6 py-4 flex items-center justify-between">
        <!-- Logo -->
        <div class="flex items-center gap-3">
          <span
            class="text-xl font-bold tracking-tight gradient-text"
          >almonds</span>
        </div>

        <!-- Links -->
        <div class="hidden md:flex items-center gap-8 text-sm" style="color: #94a3b8;">
          <a href="#features" class="hover:text-white transition-colors duration-150">Features</a>
          <a href="#ai" class="hover:text-white transition-colors duration-150">AI</a>
          <a href="#stack" class="hover:text-white transition-colors duration-150">Stack</a>
          <a href="#download" class="hover:text-white transition-colors duration-150">Download</a>
          <a
            href="https://github.com/opeolluwa/almonds"
            target="_blank"
            rel="noopener"
            class="hover:text-white transition-colors duration-150 flex items-center gap-1.5"
          >
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
            </svg>
            GitHub
          </a>
        </div>

        <!-- CTA -->
        <a
          href="#download"
          class="rounded-full px-5 py-2 text-sm font-semibold text-white transition-all duration-200"
          style="background-color: #d02752;"
          @mouseenter="e => e.target.style.backgroundColor='#f63049'"
          @mouseleave="e => e.target.style.backgroundColor='#d02752'"
        >
          Download Free
        </a>
      </div>
    </nav>

    <!-- ─── Hero ───────────────────────────────────────────────── -->
    <section class="relative overflow-hidden px-6 pt-28 pb-36 text-center">
      <!-- Ambient glow -->
      <div
        class="pointer-events-none absolute inset-0 -z-10 flex justify-center"
        aria-hidden="true"
      >
        <div
          class="h-[500px] w-[900px] rounded-full"
          style="background: radial-gradient(ellipse, rgba(208,39,82,0.12) 0%, transparent 70%); filter: blur(40px); margin-top: -100px;"
        />
      </div>

      <!-- Badge -->
      <div
        class="inline-flex items-center gap-2 rounded-full px-4 py-1.5 text-sm mb-10"
        style="border: 1px solid rgba(208,39,82,0.35); background-color: rgba(208,39,82,0.1); color: #ed7694;"
      >
        <span
          class="h-1.5 w-1.5 rounded-full animate-pulse-glow"
          style="background-color: #f63049;"
        />
        Open Source · Free Forever · MIT License
      </div>

      <!-- Headline -->
      <h1
        class="mx-auto max-w-5xl text-5xl md:text-7xl font-black leading-[1.1] tracking-tight"
      >
        Your personal workspace,
        <span class="gradient-text"> reimagined.</span>
      </h1>

      <!-- Subtext -->
      <p
        class="mx-auto mt-7 max-w-2xl text-lg leading-relaxed"
        style="color: #94a3b8;"
      >
        Almonds unifies your notes, bookmarks, code snippets, tasks, and local AI
        conversations into one beautiful desktop workspace. Built for focus,
        designed for flow — no subscriptions, no cloud lock-in.
      </p>

      <!-- CTAs -->
      <div class="mt-10 flex flex-wrap items-center justify-center gap-4">
        <a
          href="#download"
          class="inline-flex items-center gap-2.5 rounded-full px-8 py-3.5 text-sm font-semibold text-white transition-all duration-200 glow-accent"
          style="background-color: #d02752;"
          @mouseenter="e => e.currentTarget.style.backgroundColor='#f63049'"
          @mouseleave="e => e.currentTarget.style.backgroundColor='#d02752'"
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
          Download Now
        </a>

        <a
          href="https://github.com/opeolluwa/almonds"
          target="_blank"
          rel="noopener"
          class="inline-flex items-center gap-2.5 rounded-full px-8 py-3.5 text-sm font-semibold text-white transition-all duration-200"
          style="border: 1px solid rgba(255,255,255,0.1); background-color: rgba(255,255,255,0.04);"
          @mouseenter="e => { e.currentTarget.style.borderColor='rgba(255,255,255,0.2)'; e.currentTarget.style.backgroundColor='rgba(255,255,255,0.08)' }"
          @mouseleave="e => { e.currentTarget.style.borderColor='rgba(255,255,255,0.1)'; e.currentTarget.style.backgroundColor='rgba(255,255,255,0.04)' }"
        >
          <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
          </svg>
          View on GitHub
        </a>
      </div>

      <!-- Platform support -->
      <div class="mt-10 flex items-center justify-center gap-8 text-xs" style="color: #4b5563;">
        <span class="flex items-center gap-2">
          <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
            <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.8-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
          </svg>
          macOS
        </span>
        <span class="flex items-center gap-2">
          <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
            <path d="M3 5.557L9.938 4.63V11H3zM10.625 4.527L21 3v8h-10.375zM3 12.03h6.938v6.396L3 17.499zM10.625 12.098H21V21l-10.375-1.535z" />
          </svg>
          Windows
        </span>
        <span class="flex items-center gap-2">
          <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12.504 0c-.155 0-.315.008-.48.021C7.309.332 3.926 3.555 3.01 7.665c-.421 1.985.12 3.972 1.031 5.625L3.97 14.4C3.317 15.566 2.91 16.9 2.91 18.299c0 3.149 2.553 5.701 5.7 5.701 1.4 0 2.682-.51 3.668-1.33L16.297 24H18.5v-2.203l-4.019-1.014c.07-.296.117-.599.117-.914 0-1.396-.406-2.729-1.058-3.896l4.596-1.16A8.504 8.504 0 0 0 21 9.5C21 4.26 17.24 0 12.504 0zm-.007 1.5C16.422 1.5 19.5 5.085 19.5 9.5a7.003 7.003 0 0 1-2.73 5.527l-5.478 1.382a4.18 4.18 0 0 0-3.082-1.409c-1.543 0-2.85.838-3.543 2.066-.693-1.228-.85-2.7-.355-4.065L5.5 11.56c.013-.045.03-.09.047-.134C6.213 9.18 7.7 7.44 9.627 6.59L12.504 5 15.5 7l-2.996 2-.003-.001C12.499 9 12.496 9 12.496 9l.001.001-.001-.001-3.036 1.35c-.497.22-.836.716-.836 1.285 0 .778.635 1.407 1.416 1.407.416 0 .79-.177 1.059-.459l4.26-4.26A5.518 5.518 0 0 0 12.5 6.5C9.71 6.5 7.5 8.71 7.5 11.5c0 .277.028.546.075.808l-.95.237C6.179 11.1 6 9.822 6 9.5 6 5.358 8.957 1.5 12.497 1.5z" />
          </svg>
          Linux
        </span>
      </div>
    </section>

    <!-- ─── App Preview Mock ────────────────────────────────────── -->
    <section class="px-6 pb-24">
      <div class="mx-auto max-w-5xl">
        <div
          class="rounded-2xl overflow-hidden"
          style="border: 1px solid rgba(255,255,255,0.07); background-color: #111825;"
        >
          <!-- Fake window chrome -->
          <div
            class="flex items-center gap-2 px-5 py-3.5"
            style="background-color: #0d1220; border-bottom: 1px solid rgba(255,255,255,0.05);"
          >
            <span class="h-3 w-3 rounded-full" style="background-color: #ef4444;" />
            <span class="h-3 w-3 rounded-full" style="background-color: #eab308;" />
            <span class="h-3 w-3 rounded-full" style="background-color: #22c55e;" />
            <span class="mx-auto text-xs" style="color: #4b5563;">almonds — Notes</span>
          </div>
          <!-- Mock app layout -->
          <div class="flex" style="min-height: 380px;">
            <!-- Sidebar -->
            <div
              class="w-56 shrink-0 p-4 flex flex-col gap-1"
              style="border-right: 1px solid rgba(255,255,255,0.05);"
            >
              <div
                v-for="item in sidebarItems"
                :key="item.label"
                class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm cursor-pointer transition-colors"
                :style="item.active
                  ? 'background-color: rgba(208,39,82,0.15); color: #ed7694;'
                  : 'color: #6b7280;'"
              >
                <span class="text-base" v-html="item.icon" />
                {{ item.label }}
              </div>
            </div>
            <!-- Content area -->
            <div class="flex-1 p-8">
              <div class="flex items-center justify-between mb-6">
                <div>
                  <h3 class="font-semibold text-base" style="color: #f1f5f9;">My Notes</h3>
                  <p class="text-xs mt-0.5" style="color: #4b5563;">12 notes</p>
                </div>
                <div
                  class="rounded-lg px-3 py-1.5 text-xs font-medium"
                  style="background-color: rgba(208,39,82,0.15); color: #ed7694; border: 1px solid rgba(208,39,82,0.25);"
                >
                  + New Note
                </div>
              </div>
              <div class="grid grid-cols-3 gap-4">
                <div
                  v-for="note in mockNotes"
                  :key="note.title"
                  class="rounded-xl p-4 cursor-pointer transition-all card-border"
                  style="background-color: #161e30;"
                >
                  <div class="text-xs mb-2" style="color: #d02752;">{{ note.tag }}</div>
                  <div class="font-medium text-sm mb-1.5" style="color: #f1f5f9;">{{ note.title }}</div>
                  <div class="text-xs leading-relaxed" style="color: #4b5563;">{{ note.preview }}</div>
                  <div class="mt-3 text-xs" style="color: #374151;">{{ note.date }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Features ───────────────────────────────────────────── -->
    <section id="features" class="px-6 py-24" style="border-top: 1px solid rgba(255,255,255,0.04);">
      <div class="mx-auto max-w-7xl">
        <!-- Section header -->
        <div class="text-center mb-20">
          <div
            class="inline-block rounded-full px-3 py-1 text-xs font-medium mb-4"
            style="background-color: rgba(208,39,82,0.1); color: #d02752; border: 1px solid rgba(208,39,82,0.2);"
          >
            Everything you need
          </div>
          <h2 class="text-4xl md:text-5xl font-bold tracking-tight">
            One workspace.<br />
            <span class="gradient-text">Zero distractions.</span>
          </h2>
          <p class="mt-5 text-base mx-auto max-w-xl" style="color: #64748b;">
            Stop juggling a dozen apps. Almonds brings every tool you need into a
            single, distraction-free desktop experience.
          </p>
        </div>

        <!-- Feature grid -->
        <div class="grid md:grid-cols-3 gap-5">
          <div
            v-for="feature in features"
            :key="feature.title"
            class="rounded-2xl p-7 group transition-all duration-300 card-border"
            style="background-color: #111825;"
          >
            <div
              class="mb-5 inline-flex h-11 w-11 items-center justify-center rounded-xl"
              style="background-color: rgba(208,39,82,0.12);"
            >
              <span class="text-xl" v-html="feature.icon" />
            </div>
            <h3 class="text-base font-semibold mb-2" style="color: #f1f5f9;">
              {{ feature.title }}
            </h3>
            <p class="text-sm leading-relaxed" style="color: #4b5563;">
              {{ feature.description }}
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── AI Highlight ────────────────────────────────────────── -->
    <section
      id="ai"
      class="px-6 py-24"
      style="border-top: 1px solid rgba(255,255,255,0.04);"
    >
      <div class="mx-auto max-w-7xl">
        <div
          class="rounded-3xl overflow-hidden grid md:grid-cols-2 gap-0"
          style="border: 1px solid rgba(208,39,82,0.15); background: linear-gradient(135deg, #111825 0%, #0f1620 100%);"
        >
          <!-- Text side -->
          <div class="p-12 flex flex-col justify-center">
            <div
              class="inline-block rounded-full px-3 py-1 text-xs font-medium mb-6 w-fit"
              style="background-color: rgba(208,39,82,0.1); color: #d02752; border: 1px solid rgba(208,39,82,0.2);"
            >
              Local AI · Privacy First
            </div>
            <h2 class="text-3xl md:text-4xl font-bold tracking-tight mb-5">
              AI that runs <span class="gradient-text">on your machine.</span>
            </h2>
            <p class="text-sm leading-relaxed mb-8" style="color: #64748b;">
              Almonds integrates natively with Ollama, letting you chat with powerful
              language models like Llama, Mistral, and Gemma — completely offline.
              Your conversations never leave your device.
            </p>
            <ul class="space-y-3">
              <li
                v-for="point in aiPoints"
                :key="point"
                class="flex items-start gap-3 text-sm"
                style="color: #94a3b8;"
              >
                <svg class="h-4 w-4 mt-0.5 shrink-0" style="color: #d02752;" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 13l4 4L19 7" />
                </svg>
                {{ point }}
              </li>
            </ul>
          </div>
          <!-- Visual side -->
          <div class="relative p-12 flex items-center justify-center">
            <div
              class="absolute inset-0"
              style="background: radial-gradient(ellipse at 60% 50%, rgba(208,39,82,0.08) 0%, transparent 70%);"
            />
            <!-- Mock chat UI -->
            <div class="relative w-full max-w-sm space-y-4">
              <div
                v-for="msg in aiMessages"
                :key="msg.id"
                class="rounded-xl px-4 py-3 text-sm"
                :style="msg.role === 'user'
                  ? 'background-color: rgba(208,39,82,0.15); color: #f1f5f9; margin-left: 2rem; border: 1px solid rgba(208,39,82,0.2);'
                  : 'background-color: #1a2235; color: #94a3b8; margin-right: 2rem; border: 1px solid rgba(255,255,255,0.06);'"
              >
                <div class="text-xs mb-1.5 font-medium" :style="msg.role === 'user' ? 'color: #ed7694;' : 'color: #4b5563;'">
                  {{ msg.role === 'user' ? 'You' : 'Ollama' }}
                </div>
                {{ msg.text }}
              </div>
              <!-- Typing indicator -->
              <div
                class="rounded-xl px-4 py-3 flex items-center gap-1.5 w-fit"
                style="background-color: #1a2235; border: 1px solid rgba(255,255,255,0.06);"
              >
                <span
                  v-for="i in 3"
                  :key="i"
                  class="h-1.5 w-1.5 rounded-full animate-pulse-glow"
                  style="background-color: #d02752;"
                  :style="`animation-delay: ${(i - 1) * 0.2}s`"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Tech Stack ──────────────────────────────────────────── -->
    <section
      id="stack"
      class="px-6 py-24"
      style="border-top: 1px solid rgba(255,255,255,0.04);"
    >
      <div class="mx-auto max-w-7xl text-center">
        <p class="text-sm font-medium mb-8" style="color: #374151;">Built with best-in-class open source technology</p>
        <div class="flex flex-wrap items-center justify-center gap-4">
          <div
            v-for="tech in techStack"
            :key="tech.name"
            class="flex items-center gap-2.5 rounded-xl px-5 py-3 text-sm font-medium transition-all duration-200 card-border"
            style="background-color: #111825; color: #94a3b8;"
          >
            <span class="text-lg" v-html="tech.icon" />
            {{ tech.name }}
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Download ────────────────────────────────────────────── -->
    <section id="download" class="px-6 py-24">
      <div class="mx-auto max-w-7xl">
        <div
          class="rounded-3xl px-8 py-20 text-center relative overflow-hidden"
          style="background: linear-gradient(135deg, #1a0a12 0%, #1c1028 50%, #0d1220 100%); border: 1px solid rgba(208,39,82,0.2);"
        >
          <!-- Glow -->
          <div
            class="pointer-events-none absolute inset-0 -z-10 flex justify-center items-center"
            aria-hidden="true"
          >
            <div
              class="h-[400px] w-[600px] rounded-full"
              style="background: radial-gradient(ellipse, rgba(208,39,82,0.15) 0%, transparent 70%); filter: blur(60px);"
            />
          </div>

          <div
            class="inline-block rounded-full px-3 py-1 text-xs font-medium mb-6"
            style="background-color: rgba(208,39,82,0.1); color: #d02752; border: 1px solid rgba(208,39,82,0.25);"
          >
            Available Now
          </div>

          <h2 class="text-4xl md:text-6xl font-black tracking-tight mb-5">
            Your workspace awaits.
          </h2>
          <p class="text-lg mx-auto max-w-md mb-12" style="color: #64748b;">
            Download Almonds for free. No account required.
            Your data stays on your device — always.
          </p>

          <div class="flex flex-wrap items-center justify-center gap-4 mb-10">
            <a
              v-for="platform in downloadPlatforms"
              :key="platform.name"
              href="https://github.com/opeolluwa/almonds/releases"
              target="_blank"
              rel="noopener"
              class="inline-flex items-center gap-3 rounded-xl px-7 py-4 font-semibold text-sm transition-all duration-200"
              :style="platform.primary
                ? 'background-color: #d02752; color: white;'
                : 'background-color: rgba(255,255,255,0.05); color: #f1f5f9; border: 1px solid rgba(255,255,255,0.1);'"
              @mouseenter="e => { if(platform.primary) e.currentTarget.style.backgroundColor='#f63049' }"
              @mouseleave="e => { if(platform.primary) e.currentTarget.style.backgroundColor='#d02752' }"
            >
              <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
              </svg>
              {{ platform.name }}
            </a>
          </div>

          <p class="text-xs" style="color: #374151;">
            macOS · Windows · Linux &nbsp;·&nbsp; Free, open source, MIT licensed
          </p>
        </div>
      </div>
    </section>

    <!-- ─── Footer ─────────────────────────────────────────────── -->
    <footer
      class="px-6 py-12"
      style="border-top: 1px solid rgba(255,255,255,0.05);"
    >
      <div class="mx-auto max-w-7xl flex flex-col md:flex-row items-center justify-between gap-6">
        <div class="flex items-center gap-3">
          <span class="font-bold tracking-tight gradient-text">almonds</span>
          <span class="text-xs" style="color: #374151;">· Personal Productivity · MIT License</span>
        </div>

        <div class="flex items-center gap-6 text-sm" style="color: #4b5563;">
          <a
            href="https://github.com/opeolluwa/almonds"
            target="_blank"
            rel="noopener"
            class="hover:text-white transition-colors"
          >
            GitHub
          </a>
          <a
            href="https://github.com/opeolluwa/almonds/releases"
            target="_blank"
            rel="noopener"
            class="hover:text-white transition-colors"
          >
            Releases
          </a>
          <a
            href="https://github.com/opeolluwa/almonds/issues"
            target="_blank"
            rel="noopener"
            class="hover:text-white transition-colors"
          >
            Feedback
          </a>
        </div>

        <p class="text-xs" style="color: #1f2937;">
          © {{ new Date().getFullYear() }} almonds. Built with ♥
        </p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
const sidebarItems = [
  { label: 'Notes', icon: '📝', active: true },
  { label: 'Bookmarks', icon: '🔖', active: false },
  { label: 'Snippets', icon: '💻', active: false },
  { label: 'Todos', icon: '✅', active: false },
  { label: 'AI Chat', icon: '🤖', active: false },
  { label: 'Scratch Pad', icon: '✏️', active: false },
]

const mockNotes = [
  { tag: 'Work', title: 'Project kickoff notes', preview: 'Discussed the Q3 roadmap and sprint goals with the team...', date: 'Today' },
  { tag: 'Ideas', title: 'Side project brainstorm', preview: 'Building a minimal CLI tool that watches file changes and...', date: 'Yesterday' },
  { tag: 'Personal', title: 'Books to read', preview: 'Thinking Fast and Slow, The Pragmatic Programmer, Deep Work...', date: 'Mon' },
]

const features = [
  {
    icon: '📝',
    title: 'Notes',
    description: 'Rich-text notes with tagging, full-text search, and a clean distraction-free editor. Your thoughts, organized.',
  },
  {
    icon: '🔖',
    title: 'Bookmarks',
    description: 'Save web links with automatic favicon fetch and folder organization. Never lose a useful resource again.',
  },
  {
    icon: '💻',
    title: 'Snippets',
    description: 'Store reusable code with syntax highlighting for 100+ languages. Copy to clipboard in one click.',
  },
  {
    icon: '✅',
    title: 'Todos',
    description: 'Simple, fast task tracking. Add due dates, set priorities, and stay on top of what matters.',
  },
  {
    icon: '✏️',
    title: 'Scratch Pad',
    description: 'A persistent quick-capture space. Jot down thoughts without ceremony — it auto-saves as you type.',
  },
  {
    icon: '🖼️',
    title: 'Moodboard',
    description: 'Collect visual inspiration in a drag-and-drop image layout. Perfect for creative projects and design work.',
  },
]

const aiPoints = [
  'Runs entirely on your device via Ollama',
  'Supports Llama 3, Mistral, Gemma, and more',
  'No API keys, no subscriptions, no data sent to the cloud',
  'Context-aware responses using your workspace content',
]

const aiMessages = [
  { id: 1, role: 'user', text: 'Summarise my notes from this week' },
  { id: 2, role: 'assistant', text: "You wrote 4 notes this week. Here's a quick summary: Project kickoff, side project ideas, and two research notes on Rust..." },
  { id: 3, role: 'user', text: 'What are my open todos?' },
]

const techStack = [
  { name: 'Nuxt 4', icon: '💚' },
  { name: 'Vue 3', icon: '💚' },
  { name: 'Tauri v2', icon: '🦀' },
  { name: 'Tailwind CSS', icon: '🎨' },
  { name: 'Pinia', icon: '🍍' },
  { name: 'SQLite', icon: '🗃️' },
  { name: 'Ollama', icon: '🤖' },
]

const downloadPlatforms = [
  { name: 'Download for macOS', primary: true },
  { name: 'Download for Windows', primary: false },
  { name: 'Download for Linux', primary: false },
]

useSeoMeta({
  title: 'Almonds — Your Personal Workspace, Reimagined',
  description:
    'A beautiful desktop app that unifies your notes, bookmarks, snippets, tasks, and local AI into one focused workspace. Free and open source.',
  ogTitle: 'Almonds — Your Personal Workspace, Reimagined',
  ogDescription:
    'Notes, bookmarks, snippets, tasks, and local AI — all in one beautiful desktop app. Free, open source, privacy-first.',
})
</script>
