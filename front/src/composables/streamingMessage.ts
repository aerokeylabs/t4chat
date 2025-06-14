import { ref } from 'vue';

const message = ref('');
const completed = ref(true);
const cancelled = ref(false);
const currentThreadId = ref<string | null>(null);

export function useStreamingMessage() {
  const startStreaming = (threadId: string) => {
    message.value = '';
    completed.value = false;
    cancelled.value = false;
    currentThreadId.value = threadId;
  };

  const completeStreaming = () => {
    completed.value = true;
    currentThreadId.value = null;
  };

  const cancelStreaming = () => {
    cancelled.value = true;
    completed.value = true;
    currentThreadId.value = null;
  };

  const isStreaming = () => {
    return !completed.value && !cancelled.value;
  };

  return {
    message,
    completed,
    cancelled,
    currentThreadId,
    startStreaming,
    completeStreaming,
    cancelStreaming,
    isStreaming,
  };
}
