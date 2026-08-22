<script setup lang="ts">
defineProps({
  hideHeaderAndNav: Boolean,
});

const kbInset = useKeyboardInset();
</script>
<template>
  <div
    id="viewport_mobile"
    class="min-h-0 flex-1 overscroll-contain overflow-y-auto p-6"
    :class="hideHeaderAndNav ? 'pb-6 pt-15' : 'pb-24 pt-20'"
    :style="kbInset > 0 ? { paddingBottom: `${kbInset}px` } : undefined"
  >
    <slot />
  </div>
</template>
