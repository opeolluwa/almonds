<script setup lang="ts">
definePageMeta({ layout: "auth" });
import {
  kPage,
  kNavbar,
  kBlock,
  kButton,
  kList,
  kListItem,
  kLink,
  kBlockTitle,
} from 'konsta/vue';

</script>

<template>
  <k-page>
    <k-navbar title="My App" />

    <k-block strong>
      <p>Here is your Nuxt & Konsta UI app. Let's see what we have here.</p>
    </k-block>
    <k-block-title>Navigation</k-block-title>
    <k-list>
      <k-list-item href="/about/" title="About" />
      <k-list-item href="/form/" title="Form" />
    </k-list>

    <k-block strong class="flex space-x-4">
      <k-button>Button 1</k-button>
      <k-button>Button 2</k-button>
    </k-block>
  </k-page>
</template>
