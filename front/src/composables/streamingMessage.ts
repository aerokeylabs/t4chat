import { ref } from 'vue';

const message = ref('');
const completed = ref(true);

export function useStreamingMessage() {
  return { message, completed };
}
