import { useLocalStorage } from '@vueuse/core';
import { defineStore, acceptHMRUpdate } from 'pinia';

const useKeys = defineStore('keys', () => {
  const openrouter = useLocalStorage<string>('keys_openrouter', '');

  return {
    openrouter,
  };
});

export const useSettings = defineStore('settings', () => {
  const keys = useKeys();

  return {
    keys,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useKeys, import.meta.hot));
  import.meta.hot.accept(acceptHMRUpdate(useSettings, import.meta.hot));
}
