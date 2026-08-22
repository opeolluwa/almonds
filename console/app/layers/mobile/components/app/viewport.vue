<script setup lang="ts">
defineProps({
  hideHeaderAndNav: Boolean,
});
</script>
<template>
  <div
    id="viewport_mobile"
    class="min-h-0 flex-1 overscroll-contain overflow-y-auto p-6"
    :class="hideHeaderAndNav ? 'pb-6 pt-12' : 'pb-24 pt-20'"
  >
    <slot />
  </div>
</template>
