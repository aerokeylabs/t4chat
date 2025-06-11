import { ref, computed } from 'vue';

const value = ref<string>('');
const empty = computed(() => value.value.length === 0);
const hide = ref<boolean>(false);

export function useChatbox() {
  return { value, empty, hide };
}
