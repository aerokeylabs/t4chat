import { ref, computed } from 'vue';

const value = ref<string>('');
const empty = computed(() => value.value.length === 0);

export function useChatbox() {
  return { value, empty };
}
