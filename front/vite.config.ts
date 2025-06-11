import path from 'node:path';

import { defineConfig } from 'vite';

import vue from '@vitejs/plugin-vue';
import tailwindcss from '@tailwindcss/vite';
import router from 'unplugin-vue-router/vite';

export default defineConfig({
  plugins: [router(), vue(), tailwindcss()],
  resolve: {
    alias: {
      '@/convex': path.resolve(__dirname, './convex'),
      '@': path.resolve(__dirname, './src'),
    },
  },
});
