import vue from '@vitejs/plugin-vue'
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [
    react(),
    vue()
  ]
})