<script setup lang="ts">
import { ref } from 'vue';
const value = ref('## Hello, World!\n\nThis is a markdown editor. You can write your notes here.');

definePageMeta({
  layout: false
});


import type { EditorToolbarItem } from '@nuxt/ui'

const items: EditorToolbarItem[][] = [
  [
    {
      icon: 'i-lucide-heading',
      tooltip: { text: 'Headings' },
      content: {
        align: 'start'
      },
      items: [
        {
          kind: 'heading',
          level: 1,
          icon: 'i-lucide-heading-1',
          label: 'Heading 1'
        },
        {
          kind: 'heading',
          level: 2,
          icon: 'i-lucide-heading-2',
          label: 'Heading 2'
        },
        {
          kind: 'heading',
          level: 3,
          icon: 'i-lucide-heading-3',
          label: 'Heading 3'
        },
        {
          kind: 'heading',
          level: 4,
          icon: 'i-lucide-heading-4',
          label: 'Heading 4'
        }
      ]
    }
  ],
  [
    {
      kind: 'mark',
      mark: 'bold',
      icon: 'i-lucide-bold',
      tooltip: { text: 'Bold' }
    },
    {
      kind: 'mark',
      mark: 'italic',
      icon: 'i-lucide-italic',
      tooltip: { text: 'Italic' }
    },
    {
      kind: 'mark',
      mark: 'underline',
      icon: 'i-lucide-underline',
      tooltip: { text: 'Underline' }
    },
    {
      kind: 'mark',
      mark: 'strike',
      icon: 'i-lucide-strikethrough',
      tooltip: { text: 'Strikethrough' }
    },
    {
      kind: 'mark',
      mark: 'code',
      icon: 'i-lucide-code',
      tooltip: { text: 'Code' }
    }
  ]
]
</script>

<template>
    <NuxtLayout name="default">
        <template #main_content>
        <h1 class="text-2xl font-bold mb-4">Notes</h1>
        <p class="text-gray-700">This is the notes page. You can customize it as needed.</p>

          <UEditor v-slot="{ editor }" v-model="value" content-type="markdown" placeholder="Start writing..." >
            
        
         <UEditorToolbar :editor="editor" :items="items" layout="bubble" /></UEditor>
        </template>
    
        <template #side_content>
        <h2 class="text-xl font-semibold mb-3">Recently modified</h2>
        <p class="text-gray-600">This is the side content area. You can add widgets, links, or any additional information here.</p>
        </template>
    </NuxtLayout>
</template>
