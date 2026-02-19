<script setup>
const features = [
  {
    icon: '📝',
    title: 'Notes',
    description: 'Rich text notes with full-text search, tagging, and instant access from anywhere.',
  },
  {
    icon: '🔖',
    title: 'Bookmarks',
    description: 'Capture and organize web links so you never lose a resource again.',
  },
  {
    icon: '💻',
    title: 'Snippets',
    description: 'Store reusable code with syntax highlighting across every language.',
  },
  {
    icon: '✅',
    title: 'Todo',
    description: 'Task tracking with priorities, due dates, and recurring reminders.',
  },
  {
    icon: '🤖',
    title: 'Ollama AI',
    description: 'Chat with local AI models via Ollama, right inside your workspace.',
  },
  {
    icon: '🎨',
    title: 'Moodboard',
    description: 'A visual board for creative inspiration, references, and ideas.',
  },
]
</script>

<template>
  <div class="hp">
    <!-- ── Hero ──────────────────────────────────────────────────── -->
    <section class="hp-hero">
      <span class="hp-badge">Open Source · Free Forever</span>

      <h1 class="hp-title">
        Your Dev Workspace,<br />
        <span class="hp-accent">All in One Place.</span>
      </h1>

      <p class="hp-subtitle">
        Wild Almonds unifies notes, snippets, bookmarks, tasks, and AI
        conversations in a single beautiful desktop app.
      </p>

      <div class="hp-actions">
        <a href="/markdown-examples" class="hp-btn hp-btn-primary">Get Started</a>
        <a
          href="https://github.com/opeolluwa/daily"
          class="hp-btn hp-btn-alt"
          target="_blank"
          rel="noopener"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="currentColor"
            aria-hidden="true"
          >
            <path
              d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.342-3.369-1.342-.454-1.155-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0 1 12 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.202 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.741 0 .267.18.578.688.48C19.138 20.163 22 16.418 22 12c0-5.523-4.477-10-10-10z"
            />
          </svg>
          View on GitHub
        </a>
      </div>
    </section>

    <!-- ── App Screenshot ─────────────────────────────────────────── -->
    <section class="hp-shot-section">
      <div class="hp-shot-wrap">
        <img src="/app-screenshot.png" alt="Wild Almonds App" class="hp-shot-img" />
      </div>
    </section>

    <!-- ── Features ───────────────────────────────────────────────── -->
    <section class="hp-features">
      <h2 class="hp-features-title">Everything you need, nothing you don't.</h2>
      <div class="hp-features-grid">
        <div v-for="f in features" :key="f.title" class="hp-card">
          <div class="hp-card-icon">{{ f.icon }}</div>
          <h3 class="hp-card-name">{{ f.title }}</h3>
          <p class="hp-card-desc">{{ f.description }}</p>
        </div>
      </div>
    </section>

    <!-- ── Footer CTA ─────────────────────────────────────────────── -->
    <section class="hp-cta">
      <h2 class="hp-cta-title">Ready to get organized?</h2>
      <p class="hp-cta-sub">Download Wild Almonds and take control of your workflow today.</p>
      <div class="hp-actions">
        <a href="/markdown-examples" class="hp-btn hp-btn-primary">Get Started</a>
        <a
          href="https://github.com/opeolluwa/daily"
          class="hp-btn hp-btn-alt"
          target="_blank"
          rel="noopener"
        >Star on GitHub</a>
      </div>
    </section>
  </div>
</template>

<style scoped>
/* ── Layout ─────────────────────────────────────────────────────── */
.hp {
  max-width: 1100px;
  margin: 0 auto;
  padding: 0 28px 96px;
}

/* ── Hero ────────────────────────────────────────────────────────── */
.hp-hero {
  text-align: center;
  padding: 64px 0 52px;
}

.hp-badge {
  display: inline-block;
  padding: 5px 16px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.07em;
  text-transform: uppercase;
  margin-bottom: 28px;
  background: rgba(208, 39, 82, 0.08);
  color: #d02752;
  border: 1px solid rgba(208, 39, 82, 0.22);
}

:global(.dark) .hp-badge {
  background: rgba(244, 169, 185, 0.1);
  color: #f4a9b9;
  border-color: rgba(244, 169, 185, 0.25);
}

.hp-title {
  font-size: clamp(2.2rem, 5.5vw, 3.75rem);
  font-weight: 700;
  line-height: 1.13;
  letter-spacing: -0.025em;
  margin: 0 0 22px;
  color: var(--vp-c-text-1);
}

.hp-accent {
  background: linear-gradient(115deg, #f4a9b9 5%, #d02752 55%, #a11d3f 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hp-subtitle {
  font-size: 1.1rem;
  color: var(--vp-c-text-2);
  line-height: 1.75;
  max-width: 560px;
  margin: 0 auto 40px;
}

/* ── Buttons ─────────────────────────────────────────────────────── */
.hp-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  flex-wrap: wrap;
}

.hp-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 13px 30px;
  border-radius: 9px;
  font-weight: 600;
  font-size: 0.95rem;
  text-decoration: none;
  transition: all 0.18s ease;
  white-space: nowrap;
}

.hp-btn-primary {
  background: #d02752;
  color: #fff !important;
  border: 2px solid transparent;
}

.hp-btn-primary:hover {
  background: #bf2249;
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(208, 39, 82, 0.35);
}

.hp-btn-alt {
  background: transparent;
  color: var(--vp-c-text-1) !important;
  border: 2px solid var(--vp-c-divider);
}

.hp-btn-alt:hover {
  border-color: #d02752;
  color: #d02752 !important;
  transform: translateY(-2px);
}

:global(.dark) .hp-btn-alt:hover {
  border-color: #f4a9b9;
  color: #f4a9b9 !important;
}

/* ── Screenshot ──────────────────────────────────────────────────── */
.hp-shot-section {
  padding-bottom: 80px;
}

.hp-shot-wrap {
  border-radius: 16px;
  overflow: hidden;
  border: 1px solid var(--vp-c-divider);
  box-shadow:
    0 2px 8px rgba(0, 0, 0, 0.06),
    0 16px 48px rgba(0, 0, 0, 0.1),
    0 32px 80px rgba(0, 0, 0, 0.06);
}

:global(.dark) .hp-shot-wrap {
  box-shadow:
    0 2px 8px rgba(0, 0, 0, 0.3),
    0 16px 48px rgba(0, 0, 0, 0.5),
    0 32px 80px rgba(0, 0, 0, 0.4);
}

.hp-shot-img {
  width: 100%;
  display: block;
}

/* ── Features ────────────────────────────────────────────────────── */
.hp-features {
  padding-bottom: 80px;
  text-align: center;
}

.hp-features-title {
  font-size: 1.75rem;
  font-weight: 700;
  letter-spacing: -0.015em;
  margin-bottom: 44px;
  color: var(--vp-c-text-1);
}

.hp-features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(290px, 1fr));
  gap: 18px;
}

.hp-card {
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  border-radius: 14px;
  padding: 28px 24px;
  text-align: left;
  transition: border-color 0.18s, transform 0.18s, box-shadow 0.18s;
}

.hp-card:hover {
  border-color: rgba(208, 39, 82, 0.45);
  transform: translateY(-3px);
  box-shadow: 0 12px 32px rgba(208, 39, 82, 0.08);
}

:global(.dark) .hp-card:hover {
  border-color: rgba(244, 169, 185, 0.4);
  box-shadow: 0 12px 32px rgba(244, 169, 185, 0.06);
}

.hp-card-icon {
  font-size: 1.75rem;
  margin-bottom: 14px;
}

.hp-card-name {
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 8px;
  color: var(--vp-c-text-1);
}

.hp-card-desc {
  font-size: 0.88rem;
  color: var(--vp-c-text-2);
  line-height: 1.65;
  margin: 0;
}

/* ── Footer CTA ──────────────────────────────────────────────────── */
.hp-cta {
  text-align: center;
  padding: 64px 40px;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  border-radius: 20px;
}

.hp-cta-title {
  font-size: 2rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin: 0 0 12px;
  color: var(--vp-c-text-1);
}

.hp-cta-sub {
  font-size: 1rem;
  color: var(--vp-c-text-2);
  margin: 0 0 32px;
}
</style>
