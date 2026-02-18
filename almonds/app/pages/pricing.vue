<script setup lang="ts">
definePageMeta({ layout: false });

const billingCycle = ref<"monthly" | "yearly">("monthly");

const plans = computed(() => [
  {
    key: "free",
    name: "Free",
    desc: "For personal use on a single device.",
    price: { monthly: 0, yearly: 0 },
    cta: "Get started",
    ctaVariant: "outline" as const,
    highlight: false,
    features: [
      "Local storage only",
      "All core modules (Notes, Tasks, Todo, Calendar)",
      "Unlimited entries",
      "AI via local Ollama",
    ],
    missing: ["Cloud sync", "Multi-device access", "Priority support"],
  },
  {
    key: "pro",
    name: "Pro",
    desc: "Sync seamlessly across all your devices.",
    price: { monthly: 7, yearly: 5 },
    cta: "Upgrade to Pro",
    ctaVariant: "solid" as const,
    highlight: true,
    features: [
      "Everything in Free",
      "Almond Cloud sync",
      "Up to 5 devices",
      "10 GB cloud storage",
      "Encrypted backups",
      "Priority support",
    ],
    missing: ["Custom domain", "Team workspaces"],
  },
  {
    key: "team",
    name: "Team",
    desc: "Shared workspaces for teams that build together.",
    price: { monthly: 18, yearly: 14 },
    cta: "Start team trial",
    ctaVariant: "outline" as const,
    highlight: false,
    features: [
      "Everything in Pro",
      "Unlimited devices",
      "50 GB shared storage",
      "Shared workspaces",
      "Admin & permissions",
      "SSO / SAML",
      "Dedicated support",
    ],
    missing: [],
  },
]);

const yearlyDiscount = 28;
</script>

<template>
  <NuxtLayout name="default">
    <template #main_content>
      <div class="max-w-4xl mt-6">
        <!-- Billing toggle -->
        <div class="flex items-center gap-3 mb-8">
          <span
            class="text-sm"
            :class="
              billingCycle === 'monthly'
                ? 'text-gray-800 dark:text-gray-100 font-medium'
                : 'text-gray-400'
            "
            >Monthly</span
          >
          <button
            class="relative w-10 h-6 rounded-full transition-colors"
            :class="
              billingCycle === 'yearly'
                ? 'bg-accent-500'
                : 'bg-gray-200 dark:bg-gray-600'
            "
            @click="
              billingCycle =
                billingCycle === 'monthly' ? 'yearly' : 'monthly'
            "
          >
            <span
              class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
              :class="
                billingCycle === 'yearly' ? 'translate-x-4' : 'translate-x-0'
              "
            />
          </button>
          <span
            class="text-sm"
            :class="
              billingCycle === 'yearly'
                ? 'text-gray-800 dark:text-gray-100 font-medium'
                : 'text-gray-400'
            "
            >Yearly</span
          >
          <span
            class="text-xs font-medium px-2 py-0.5 rounded-full bg-emerald-100 dark:bg-emerald-950 text-emerald-700 dark:text-emerald-400"
          >
            Save {{ yearlyDiscount }}%
          </span>
        </div>

        <!-- Plan cards -->
        <div class="grid grid-cols-3 gap-4">
          <div
            v-for="plan in plans"
            :key="plan.key"
            class="flex flex-col rounded-xl border p-5 transition-shadow"
            :class="
              plan.highlight
                ? 'border-accent-400 dark:border-accent-600 bg-white dark:bg-gray-800 shadow-md ring-1 ring-accent-400 dark:ring-accent-600'
                : 'border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800'
            "
          >
            <!-- Badge -->
            <div class="flex items-start justify-between mb-4">
              <div>
                <p class="text-sm font-semibold text-gray-800 dark:text-gray-100">
                  {{ plan.name }}
                </p>
                <p class="text-xs text-gray-400 mt-0.5">{{ plan.desc }}</p>
              </div>
              <span
                v-if="plan.highlight"
                class="text-xs font-medium px-2 py-0.5 rounded-full bg-accent-100 dark:bg-accent-950 text-accent-700 dark:text-accent-300 shrink-0 ml-2"
              >
                Popular
              </span>
            </div>

            <!-- Price -->
            <div class="mb-5">
              <div class="flex items-end gap-1">
                <span class="text-3xl font-bold text-gray-900 dark:text-white">
                  ${{ plan.price[billingCycle] }}
                </span>
                <span
                  v-if="plan.price[billingCycle] > 0"
                  class="text-xs text-gray-400 mb-1.5"
                >
                  / mo
                </span>
                <span
                  v-else
                  class="text-xs text-gray-400 mb-1.5"
                >
                  forever
                </span>
              </div>
              <p
                v-if="billingCycle === 'yearly' && plan.price.yearly > 0"
                class="text-xs text-gray-400 mt-0.5"
              >
                Billed ${{ plan.price.yearly * 12 }}/year
              </p>
            </div>

            <!-- CTA -->
            <button
              class="w-full py-2 rounded-lg text-sm font-medium transition-colors mb-5"
              :class="
                plan.ctaVariant === 'solid'
                  ? 'bg-accent-500 text-white hover:bg-accent-600'
                  : 'border border-gray-200 dark:border-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700'
              "
            >
              {{ plan.cta }}
            </button>

            <!-- Features -->
            <ul class="flex flex-col gap-2 flex-1">
              <li
                v-for="f in plan.features"
                :key="f"
                class="flex items-start gap-2 text-xs text-gray-600 dark:text-gray-300"
              >
                <UIcon
                  name="heroicons:check"
                  class="size-3.5 text-emerald-500 shrink-0 mt-0.5"
                />
                {{ f }}
              </li>
              <li
                v-for="f in plan.missing"
                :key="f"
                class="flex items-start gap-2 text-xs text-gray-300 dark:text-gray-600 line-through"
              >
                <UIcon
                  name="heroicons:minus"
                  class="size-3.5 shrink-0 mt-0.5"
                />
                {{ f }}
              </li>
            </ul>
          </div>
        </div>

        <!-- Self-hosted callout -->
        <div
          class="mt-6 flex items-center justify-between rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 py-4 px-4"
        >
          <div class="flex items-center gap-3">
            <div
              class="size-9 rounded-lg bg-gray-100 dark:bg-gray-700 flex items-center justify-center shrink-0"
            >
              <UIcon
                name="heroicons:server"
                class="size-4 text-gray-500 dark:text-gray-400"
              />
            </div>
            <div>
              <p class="text-sm font-medium text-gray-800 dark:text-gray-100">
                Self-hosted
              </p>
              <p class="text-xs text-gray-400 mt-0.5">
                Run Almond Cloud on your own infrastructure. Free forever.
              </p>
            </div>
          </div>
          <NuxtLink
            to="/settings"
            class="shrink-0 flex items-center gap-1.5 text-sm font-medium text-accent-600 dark:text-accent-400 hover:underline"
          >
            Configure
            <UIcon name="heroicons:arrow-right" class="size-3.5" />
          </NuxtLink>
        </div>
      </div>
    </template>
  </NuxtLayout>
</template>
