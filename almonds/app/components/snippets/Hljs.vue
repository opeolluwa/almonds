<template>
  <highlightjs language="js" code="console.log('Hello World');" />
</template>

<script>
import "highlight.js/lib/common";
import hljsVuePlugin from "@highlightjs/vue-plugin";

export default {
  components: {
    highlightjs: hljsVuePlugin.component,
  },
};
</script>
