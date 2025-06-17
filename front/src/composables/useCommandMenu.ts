import { acceptHMRUpdate, defineStore } from 'pinia';
import { ref } from 'vue';

export const useCommandMenu = defineStore('commandMenu', () => {
  const open = ref(false);

  const toggle = () => (open.value = !open.value);
  const close = () => (open.value = false);

  return { open, toggle, close };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useCommandMenu, import.meta.hot));
}
