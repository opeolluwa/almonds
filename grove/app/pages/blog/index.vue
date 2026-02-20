<template>
  <div class="min-h-screen bg-white dark:bg-[#0d1220] text-slate-900 dark:text-slate-100">
    <AppNav />

    <main class="mx-auto max-w-3xl px-6 py-20">
      <!-- Header -->
      <div class="mb-14">
        <p class="text-xs font-semibold uppercase tracking-widest text-accent-500 dark:text-accent-400 mb-3">
          Blog
        </p>
        <h1 class="text-4xl font-bold tracking-tight text-slate-900 dark:text-white">
          From the team
        </h1>
        <p class="mt-3 text-slate-500 dark:text-slate-400 text-sm leading-relaxed">
          Updates, stories, and thinking from the people building Almonds.
        </p>
      </div>

      <!-- Posts -->
      <div v-if="posts && posts.length" class="flex flex-col divide-y divide-slate-100 dark:divide-white/5">
        <article
          v-for="post in posts"
          :key="post._path"
          class="py-8 group"
        >
          <NuxtLink :to="post._path" class="block">
            <div class="flex items-center gap-2 mb-2">
              <time class="text-xs text-slate-400" :datetime="post.date">
                {{ formatDate(post.date) }}
              </time>
              <span v-if="post.author" class="text-xs text-slate-300 dark:text-slate-700">·</span>
              <span v-if="post.author" class="text-xs text-slate-400">{{ post.author }}</span>
            </div>
            <h2 class="text-lg font-semibold text-slate-900 dark:text-white group-hover:text-accent-500 dark:group-hover:text-accent-400 transition-colors duration-150 mb-1.5">
              {{ post.title }}
            </h2>
            <p class="text-sm text-slate-500 dark:text-slate-400 leading-relaxed mb-3">
              {{ post.description }}
            </p>
            <div v-if="post.tags?.length" class="flex flex-wrap gap-1.5">
              <span
                v-for="tag in post.tags"
                :key="tag"
                class="text-xs px-2 py-0.5 rounded-full bg-slate-100 dark:bg-white/5 text-slate-500 dark:text-slate-400"
              >
                {{ tag }}
              </span>
            </div>
          </NuxtLink>
        </article>
      </div>

      <!-- Empty state -->
      <div v-else class="py-16 text-center text-slate-400 dark:text-slate-600">
        <UIcon name="i-heroicons-document-text" class="size-10 mx-auto mb-4 opacity-40" />
        <p class="text-sm">No posts yet. Check back soon.</p>
      </div>
    </main>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
useSeoMeta({
  title: 'Blog — Almonds',
  description: 'Updates, stories, and thinking from the people building Almonds.',
})

const { data: posts } = await useAsyncData('blog-posts', () =>
  queryContent('blog').sort({ date: -1 }).find()
)

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}
</script>
