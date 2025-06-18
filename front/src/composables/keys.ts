import { useLocalStorage } from '@vueuse/core';
import { acceptHMRUpdate, defineStore } from 'pinia';

export const useKeys = defineStore('keys', () => {
  const openrouter = useLocalStorage<string>('keys_openrouter', '');

  return {
    openrouter,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useKeys, import.meta.hot));
}
