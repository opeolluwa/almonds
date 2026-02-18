<script setup lang="ts">
const displayName = ref("Nick woods");
const email = ref("nick.woods@gmail.com");
</script>

<template>
  <div class="flex flex-col gap-4 mt-4">
    <div
      class="bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 p-5"
    >
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
        Profile
      </h2>
      <div class="flex items-center gap-4 mb-6">
        <div class="relative shrink-0">
          <img
            src="https://i.pravatar.cc/150?u=nick-woods"
            class="size-16 rounded-full object-cover"
            alt="Avatar"
          />
          <button
            class="absolute -bottom-1 -right-1 size-6 bg-accent-500 rounded-full flex items-center justify-center hover:bg-accent-600 transition-colors"
          >
            <UIcon name="heroicons:camera" class="size-3 text-white" />
          </button>
        </div>
        <div>
          <p class="text-sm font-medium text-gray-800 dark:text-gray-100">
            {{ displayName }}
          </p>
          <p class="text-xs text-gray-400">{{ email }}</p>
        </div>
      </div>
      <div class="flex flex-col gap-4">
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
            >Display Name</label
          >
          <input
            v-model="displayName"
            type="text"
            class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
          />
        </div>
        <div>
          <label
            class="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5"
            >Email</label
          >
          <input
            v-model="email"
            type="email"
            class="w-full bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 text-sm text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-600 outline-none focus:ring-2 focus:ring-accent-300 dark:focus:ring-accent-600 focus:border-transparent"
          />
        </div>
      </div>
      <div class="mt-5 flex justify-end">
        <button
          class="px-4 py-2 bg-accent-500 text-white text-sm font-medium rounded-lg hover:bg-accent-600 transition-colors"
        >
          Save changes
        </button>
      </div>
    </div>
  </div>
</template>
