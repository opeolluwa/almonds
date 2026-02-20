<script setup lang="ts">
const colorMode = useColorMode();
const isDark = computed({
  get: () => colorMode.value === "dark",
  set: (v) => (colorMode.preference = v ? "dark" : "light"),
});
const { fontSize, setFontSize } = useFontSize();
const { accent: selectedAccent, setAccent } = useAccentColor();
const { darkTheme: selectedDarkTheme, themes: darkThemes, setDarkTheme } = useDarkTheme();
const accentOptions: { key: AccentKey; label: string; bg: string }[] = [
  { key: "rose", label: "Rose", bg: "bg-rose-600" },
  { key: "emerald", label: "Emerald", bg: "bg-emerald-500" },
  { key: "sky", label: "Sky", bg: "bg-sky-500" },
  { key: "amber", label: "Amber", bg: "bg-amber-500" },
];
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
        Appearance
      </h2>
      <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-700">
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">Dark mode</p>
            <p class="text-xs text-gray-400 mt-0.5">
              Switch between light and dark theme
            </p>
          </div>
          <button
            class="relative w-10 h-6 rounded-full transition-colors"
            :class="isDark ? 'bg-accent-500' : 'bg-gray-200 dark:bg-gray-600'"
            @click="isDark = !isDark"
          >
            <span
              class="absolute top-0.5 left-0.5 size-5 bg-white rounded-full shadow transition-transform duration-200"
              :class="isDark ? 'translate-x-4' : 'translate-x-0'"
            />
          </button>
        </div>
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">Font size</p>
            <p class="text-xs text-gray-400 mt-0.5">
              Adjust text size across the app
            </p>
          </div>
          <div class="flex gap-1 bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5">
            <button
              v-for="sz in ['sm', 'md', 'lg'] as const"
              :key="sz"
              class="px-3 py-1.5 rounded-md text-xs font-medium transition-colors uppercase"
              :class="
                fontSize === sz
                  ? 'bg-white dark:bg-gray-600 text-gray-800 dark:text-gray-100 shadow-sm'
                  : 'text-gray-500 dark:text-gray-400'
              "
              @click="setFontSize(sz)"
            >
              {{ sz }}
            </button>
          </div>
        </div>
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">Accent color</p>
            <p class="text-xs text-gray-400 mt-0.5">Primary highlight color</p>
          </div>
          <div class="flex gap-2">
            <div
              v-for="a in accentOptions"
              :key="a.key"
              class="relative group flex flex-col items-center"
            >
              <span
                class="absolute -top-7 left-1/2 -translate-x-1/2 px-1.5 py-0.5 rounded text-[10px] font-medium bg-gray-800 dark:bg-gray-950 text-white whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"
              >
                {{ a.label }}
              </span>
              <button
                class="size-6 rounded-full transition-transform hover:scale-110"
                :class="[
                  a.bg,
                  selectedAccent === a.key
                    ? 'ring-2 ring-offset-2 ring-gray-400 scale-110'
                    : '',
                ]"
                @click="setAccent(a.key)"
              />
            </div>
          </div>
        </div>
        <div class="flex items-center justify-between py-3">
          <div>
            <p class="text-sm text-gray-700 dark:text-gray-200">Dark theme</p>
            <p class="text-xs text-gray-400 mt-0.5">Background tint in dark mode</p>
          </div>
          <div class="flex gap-2">
            <div
              v-for="(theme, key) in darkThemes"
              :key="key"
              class="relative group flex flex-col items-center"
            >
              <span
                class="absolute -top-7 left-1/2 -translate-x-1/2 px-1.5 py-0.5 rounded text-[10px] font-medium bg-gray-800 dark:bg-gray-950 text-white whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"
              >
                {{ theme.label }}
              </span>
              <button
                class="size-6 rounded-full transition-transform hover:scale-110"
                :style="{ backgroundColor: theme.swatch }"
                :class="
                  selectedDarkTheme === key
                    ? 'ring-2 ring-offset-2 ring-gray-400 scale-110'
                    : ''
                "
                @click="setDarkTheme(key)"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
