import type { Message } from '@/lib/types';
import { ref } from 'vue';

const messages = ref<Message[]>([]);

export function useMessages() {
  return messages;
}
