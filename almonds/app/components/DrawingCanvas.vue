<script setup lang="ts">
import { createElement } from 'react'
import { createRoot, Root } from 'react-dom/client'
import { onMounted, onUnmounted, useTemplateRef } from 'vue'
import { ExcalidrawWrapper, ExcalidrawImperativeAPI } from './ExcalidrawWrapper'
import { exportToSvg } from '@excalidraw/excalidraw'

const containerEl = useTemplateRef('container')
let root: Root | null = null
let excalidrawAPI: ExcalidrawImperativeAPI | null = null

const onApiReady = (api: ExcalidrawImperativeAPI) => {
  excalidrawAPI = api
}

const exportSvg = async () => {
  if (!excalidrawAPI) return

  const elements = excalidrawAPI.getSceneElements()
  if (elements.length === 0) {
    alert('Draw something first!')
    return
  }

  const svg = await exportToSvg({
    elements,
    appState: excalidrawAPI.getAppState(),
    files: excalidrawAPI.getFiles()
  })

  const svgString = new XMLSerializer().serializeToString(svg)
  const blob = new Blob([svgString], { type: 'image/svg+xml' })
  const url = URL.createObjectURL(blob)

  const link = document.createElement('a')
  link.href = url
  link.download = 'drawing.svg'
  link.click()

  URL.revokeObjectURL(url)
}

onMounted(() => {
  if (!containerEl.value) return
  root = createRoot(containerEl.value)
  root.render(createElement(ExcalidrawWrapper, { onApiReady }))
})

onUnmounted(() => {
  if (root) {
    root.unmount()
    root = null
  }
  excalidrawAPI = null
})
</script>

<template>
  <div class="drawing-wrapper">
    <div class="toolbar">
      <button @click="exportSvg" class="export-btn">
        Export SVG
      </button>
    </div>
    <div ref="container" class="canvas-container"></div>
  </div>
</template>

<style scoped>
.drawing-wrapper {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.toolbar {
  padding: 8px 16px;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
}

.export-btn {
  padding: 8px 16px;
  background: #6965db;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.export-btn:hover {
  background: #5b57c7;
}

.canvas-container {
  flex: 1;
  height: 500vh;
}
</style>