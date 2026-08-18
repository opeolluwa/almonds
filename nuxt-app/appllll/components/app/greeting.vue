<template>
  <div class="quotes-container">
    <div :key="quoteKey" class="quote">{{ current.quote }}</div>
    <small :key="quoteKey + '-author'" class="quoter">{{
      current.quoter
    }}</small>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { data } from "../../data/quotes";

type Quote = (typeof data)[number];

const DURATION = 15000;

function randomIndex() {
  return Math.round((data.length - 1) * Math.random());
}

const current = ref<Quote>(data[randomIndex()] ?? { quote: "", quoter: "" });
const quoteKey = ref(0);

let timer: ReturnType<typeof setInterval>;

onMounted(() => {
  timer = setInterval(() => {
    current.value = data[randomIndex()] ?? { quote: "", quoter: "" };
    quoteKey.value++;
  }, DURATION);
});

onUnmounted(() => clearInterval(timer));
</script>

<style scoped>
.quotes-container {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.quote {
  font-size: 110%;
  font-weight: 200;
  text-align: center;
  margin-bottom: 1rem;
  /* max-width:0%; */
  animation: text-focus-in 1000ms cubic-bezier(0.55, 0.085, 0.68, 0.53) both;
}

.quote::before {
  content: "\201C\00A0";
}

.quote::after {
  content: "\00A0\201D";
}

.quoter {
  text-align: center;
  display: block;
  font-size: 80%;
  color: #6c757d;
  animation: text-focus-in 1000ms cubic-bezier(0.55, 0.085, 0.68, 0.53) both;
}

.quoter::before {
  content: "\2014\00A0";
}

@keyframes text-focus-in {
  0% {
    filter: blur(12px);
    opacity: 0;
  }
  100% {
    filter: blur(0);
    opacity: 1;
  }
}
</style>
