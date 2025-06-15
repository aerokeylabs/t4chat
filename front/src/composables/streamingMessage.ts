import { computed, ref } from 'vue';

const message = ref('');
const completed = ref(true);
const cancelled = ref(false);
const failed = ref(false);
const currentThreadId = ref<string | null>(null);

export function useStreamingMessage() {
  function onStreamStarted(threadId: string) {
    message.value = '';
    completed.value = false;
    cancelled.value = false;
    failed.value = false;
    currentThreadId.value = threadId;
  }

  function onStreamCompleted() {
    completed.value = true;
    cancelled.value = false;
    failed.value = false;
    currentThreadId.value = null;
  }

  function onStreamCancelled() {
    cancelled.value = true;
    completed.value = true;
    failed.value = false;
    currentThreadId.value = null;
  }

  function onStreamFailed() {
    failed.value = true;
    completed.value = true;
    cancelled.value = false;
    currentThreadId.value = null;
  }

  const isStreaming = computed(() => !completed.value && !cancelled.value && !failed.value);

  return {
    message,
    completed,
    cancelled,
    failed,
    currentThreadId,
    onStreamStarted,
    onStreamCompleted,
    onStreamCancelled,
    onStreamFailed,
    isStreaming,
  };
}
