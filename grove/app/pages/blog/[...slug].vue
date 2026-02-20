<template>
  <div class="min-h-screen bg-white dark:bg-[#0d1220] text-slate-900 dark:text-slate-100">
    <AppNav />

    <main class="mx-auto max-w-3xl px-6 py-20">
      <NuxtLink
        to="/blog"
        class="inline-flex items-center gap-1.5 text-sm text-slate-400 hover:text-slate-900 dark:hover:text-white transition-colors duration-150 mb-10"
      >
        <UIcon name="i-heroicons-arrow-left" class="size-3.5" />
        Back to blog
      </NuxtLink>

      <article v-if="post">
        <div class="mb-8">
          <div class="flex items-center gap-2 mb-4">
            <time class="text-xs text-slate-400" :datetime="post.date">
              {{ formatDate(post.date) }}
            </time>
            <span v-if="post.author" class="text-xs text-slate-300 dark:text-slate-700">·</span>
            <span v-if="post.author" class="text-xs text-slate-400">{{ post.author }}</span>
          </div>
          <h1 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white mb-3">
            {{ post.title }}
          </h1>
          <p class="text-slate-500 dark:text-slate-400 text-sm leading-relaxed">
            {{ post.description }}
          </p>
          <div v-if="post.tags?.length" class="flex flex-wrap gap-1.5 mt-4">
            <span
              v-for="tag in post.tags"
              :key="tag"
              class="text-xs px-2 py-0.5 rounded-full bg-slate-100 dark:bg-white/5 text-slate-500 dark:text-slate-400"
            >
              {{ tag }}
            </span>
          </div>
        </div>

        <div class="prose prose-slate dark:prose-invert prose-sm max-w-none">
          <ContentRenderer :value="post" />
        </div>
      </article>

      <div v-else class="py-16 text-center text-slate-400">
        <p class="text-sm">Post not found.</p>
      </div>
    </main>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
const route = useRoute()

const { data: post } = await useAsyncData(`blog-${route.path}`, () =>
  queryContent(route.path).findOne()
)

useSeoMeta({
  title: post.value ? `${post.value.title} — Almonds Blog` : 'Almonds Blog',
  description: post.value?.description,
})

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}
</script>
