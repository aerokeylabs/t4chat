import { useLocalStorage } from '@vueuse/core';
import { defineStore } from 'pinia';
import { watch } from 'vue';

export type ThemeMode = 'system' | 'light' | 'dark';

export const useTheme = defineStore('theme', () => {
  const mode = useLocalStorage<ThemeMode>('theme', 'system');
  const chroma = useLocalStorage<number>('chroma', 0);
  const hue = useLocalStorage<number>('hue', 0);

  watch(mode, (value) => {
    switch (value) {
      case 'dark': {
        document.documentElement.classList.remove('light');
        document.documentElement.classList.add('dark');
        break;
      }
      case 'light': {
        document.documentElement.classList.remove('dark');
        document.documentElement.classList.add('light');
        break;
      }
      default: {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        document.documentElement.classList.remove('dark', 'light');
        document.documentElement.classList.add(prefersDark ? 'dark' : 'light');
        break;
      }
    }
  });

  watch(chroma, (value) => {
    document.documentElement.style.setProperty('--c', value.toFixed(2));
  });

  watch(hue, (value) => {
    document.documentElement.style.setProperty('--h', value.toFixed(2));
  });

  function toggle() {
    switch (mode.value) {
      case 'system': {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        mode.value = prefersDark ? 'light' : 'dark';
        break;
      }
      case 'dark': {
        mode.value = 'light';
        break;
      }
      case 'light': {
        mode.value = 'dark';
        break;
      }
      default: {
        mode.value = 'system';
      }
    }
  }

  return {
    mode,
    chroma,
    hue,

    toggle,
  };
});
