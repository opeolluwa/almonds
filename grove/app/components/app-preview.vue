<template>
  <section class="px-6 pb-0">
    <div class="mx-auto max-w-6xl">
      <!-- Outer glow frame -->
      <div class="relative">
        <div
          class="absolute -inset-1 rounded-2xl opacity-30 dark:opacity-20 blur-xl"
          style="background: radial-gradient(ellipse, rgba(208, 39, 82, 0.4) 0%, transparent 70%)"
          aria-hidden="true"
        />
        <div class="relative rounded-2xl overflow-hidden shadow-2xl ring-1 ring-black/10 dark:ring-white/5">
          <NuxtImg
            :src="screenshot"
            alt="Almonds App Preview"
            class="w-full h-auto block"
          />
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const colorMode = useColorMode()
const isDark = computed(() => colorMode.value === 'dark')
const screenshot = computed(()=> isDark.value ? '/app-screenshot-dark.png' : '/app-screenshot-light.png')
</script>
