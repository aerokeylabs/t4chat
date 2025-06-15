import { useLocalStorage } from '@vueuse/core';

const openrouter = useLocalStorage<string>('openrouter-key', '');

export function useApiKeys() {
  return {
    openrouter,
  };
}
