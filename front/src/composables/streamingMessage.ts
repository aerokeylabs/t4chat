import { ref } from 'vue';

const message = ref('');
const completed = ref(false);

export function useStreamingMessage() {
  return { message, completed };
}
