<template>
  <section
    id="pricing"
    class="px-6 py-24 border-t border-slate-100 dark:border-white/[0.04]"
  >
    <div class="mx-auto max-w-7xl">
      <div class="text-center mb-12">
        <p class="text-xs font-semibold uppercase tracking-widest text-accent-500 dark:text-accent-400 mb-3">
          Pricing
        </p>
        <h2 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white mb-4">
          Simple, transparent pricing
        </h2>
        <p class="text-slate-500 dark:text-slate-400 max-w-xl mx-auto text-sm leading-relaxed">
          Start for free, upgrade when you need more. No hidden fees, no surprise bills.
        </p>
      </div>

      <!-- Billing toggle -->
      <div class="flex items-center justify-center gap-3 mb-10">
        <span
          class="text-sm transition-colors"
          :class="billingCycle === 'monthly' ? 'text-slate-900 dark:text-white font-medium' : 'text-slate-400'"
        >Monthly</span>
        <button
          class="relative w-10 h-6 rounded-full transition-colors"
          :class="billingCycle === 'yearly' ? 'bg-accent-500' : 'bg-slate-200 dark:bg-slate-700'"
          @click="billingCycle = billingCycle === 'monthly' ? 'yearly' : 'monthly'"
        >
          <span
            class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
            :class="billingCycle === 'yearly' ? 'translate-x-4' : 'translate-x-0'"
          />
        </button>
        <span
          class="text-sm transition-colors"
          :class="billingCycle === 'yearly' ? 'text-slate-900 dark:text-white font-medium' : 'text-slate-400'"
        >Yearly</span>
        <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-emerald-100 dark:bg-emerald-950 text-emerald-700 dark:text-emerald-400">
          Save {{ yearlyDiscount }}%
        </span>
      </div>

      <!-- Plan cards -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-5 max-w-5xl mx-auto">
        <div
          v-for="plan in plans"
          :key="plan.key"
          class="flex flex-col rounded-2xl border p-6 transition-shadow"
          :class="
            plan.highlight
              ? 'border-accent-400 dark:border-accent-600 bg-white dark:bg-[#111825] shadow-lg ring-1 ring-accent-400 dark:ring-accent-600'
              : 'border-slate-200 dark:border-white/[0.06] bg-white dark:bg-[#111825]'
          "
        >
          <!-- Header -->
          <div class="flex items-start justify-between mb-4">
            <div>
              <p class="text-sm font-semibold text-slate-900 dark:text-white">{{ plan.name }}</p>
              <p class="text-xs text-slate-400 mt-0.5">{{ plan.desc }}</p>
            </div>
            <span
              v-if="plan.highlight"
              class="text-xs font-medium px-2 py-0.5 rounded-full bg-accent-100 dark:bg-accent-950 text-accent-700 dark:text-accent-300 shrink-0 ml-2"
            >
              Popular
            </span>
          </div>

          <!-- Price -->
          <div class="mb-6">
            <div class="flex items-end gap-1">
              <span class="text-4xl font-bold text-slate-900 dark:text-white">
                ${{ plan.price[billingCycle] }}
              </span>
              <span v-if="plan.price[billingCycle] > 0" class="text-xs text-slate-400 mb-2">/ mo</span>
              <span v-else class="text-xs text-slate-400 mb-2">forever</span>
            </div>
            <p v-if="billingCycle === 'yearly' && plan.price.yearly > 0" class="text-xs text-slate-400 mt-1">
              Billed ${{ plan.price.yearly * 12 }}/year
            </p>
          </div>

          <!-- CTA -->
          <a
            href="#download"
            class="w-full py-2 rounded-xl text-sm font-semibold text-center transition-colors mb-6"
            :class="
              plan.highlight
                ? 'bg-accent-500 text-white hover:bg-accent-400'
                : 'border border-slate-200 dark:border-white/10 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-white/5'
            "
          >
            {{ plan.cta }}
          </a>

          <!-- Features -->
          <ul class="flex flex-col gap-2.5 flex-1">
            <li
              v-for="f in plan.features"
              :key="f"
              class="flex items-start gap-2 text-xs text-slate-600 dark:text-slate-300"
            >
              <UIcon name="i-heroicons-check" class="size-3.5 text-emerald-500 shrink-0 mt-0.5" />
              {{ f }}
            </li>
            <li
              v-for="f in plan.missing"
              :key="f"
              class="flex items-start gap-2 text-xs text-slate-300 dark:text-slate-600 line-through"
            >
              <UIcon name="i-heroicons-minus" class="size-3.5 shrink-0 mt-0.5" />
              {{ f }}
            </li>
          </ul>
        </div>
      </div>

      <!-- Self-hosted callout -->
      <div class="mt-8 max-w-5xl mx-auto flex flex-col sm:flex-row items-start sm:items-center gap-4 sm:gap-0 justify-between rounded-xl border border-slate-200 dark:border-white/[0.06] bg-white dark:bg-[#111825] py-4 px-5">
        <div class="flex items-center gap-3">
          <div class="size-9 rounded-lg bg-slate-100 dark:bg-white/5 flex items-center justify-center shrink-0">
            <UIcon name="i-heroicons-server" class="size-4 text-slate-500 dark:text-slate-400" />
          </div>
          <div>
            <p class="text-sm font-medium text-slate-900 dark:text-white">Self-hosted</p>
            <p class="text-xs text-slate-400 mt-0.5">Run Almond Cloud on your own infrastructure. Free forever.</p>
          </div>
        </div>
        <a
          href="https://github.com/opeolluwa/almonds"
          target="_blank"
          rel="noopener"
          class="shrink-0 flex items-center gap-1.5 text-sm font-medium text-accent-600 dark:text-accent-400 hover:underline"
        >
          Learn more
          <UIcon name="i-heroicons-arrow-right" class="size-3.5" />
        </a>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const billingCycle = ref<'monthly' | 'yearly'>('monthly')

const yearlyDiscount = 28

const plans = computed(() => [
  {
    key: 'free',
    name: 'Free',
    desc: 'For personal use on a single device.',
    price: { monthly: 0, yearly: 0 },
    cta: 'Get started',
    highlight: false,
    features: [
      'Local storage only',
      'All core modules (Notes, Tasks, Todo, Calendar)',
      'Unlimited entries',
      'AI via local Ollama',
    ],
    missing: ['Cloud sync', 'Multi-device access', 'Priority support'],
  },
  {
    key: 'pro',
    name: 'Pro',
    desc: 'Sync seamlessly across all your devices.',
    price: { monthly: 7, yearly: 5 },
    cta: 'Upgrade to Pro',
    highlight: true,
    features: [
      'Everything in Free',
      'Almond Cloud sync',
      'Up to 5 devices',
      '10 GB cloud storage',
      'Encrypted backups',
      'Priority support',
    ],
    missing: ['Custom domain', 'Team workspaces'],
  },
  {
    key: 'team',
    name: 'Team',
    desc: 'Shared workspaces for teams that build together.',
    price: { monthly: 18, yearly: 14 },
    cta: 'Start team trial',
    highlight: false,
    features: [
      'Everything in Pro',
      'Unlimited devices',
      '50 GB shared storage',
      'Shared workspaces',
      'Admin & permissions',
      'SSO / SAML',
      'Dedicated support',
    ],
    missing: [],
  },
])
</script>
