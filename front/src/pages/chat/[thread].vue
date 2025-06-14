<script setup lang="ts">
import AssistantMessage from '@/components/messages/AssistantMessage.vue';
import StreamingMessage from '@/components/messages/StreamingMessage.vue';
import UserMessage from '@/components/messages/UserMessage.vue';
import { useReactiveQuery } from '@/composables/convex';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
import { cancelMessage } from '@/lib/api';
import type { Message } from '@/lib/types/convex';
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const args = computed(() => ({ threadId: route.params.thread as string }));

const { data, error } = useReactiveQuery(api.messages.getByThreadId, args);

const messages = computed(() => {
  if (!data.value) return [];
  return data.value.messages as Message[];
});

const {
  message: streamingMessage,
  completed,
  cancelled,
  currentThreadId,
  cancelStreaming,
  isStreaming,
} = useStreamingMessage();

// show streaming message if it is not completed or if the last message is still pending
const showStreamingMessage = computed(() => {
  const lastMessage = messages.value.length === 0 ? null : messages.value[messages.value.length - 1];
  if (!completed.value) return true;
  if (lastMessage?.role === 'assistant' && lastMessage.status === 'complete') return false;
  return true;
});

// Show cancel button if currently streaming for this thread
const canCancelMessage = computed(() => {
  return isStreaming() && currentThreadId.value === route.params.thread;
});

async function handleCancelMessage() {
  if (!currentThreadId.value || !isStreaming()) {
    return;
  }

  try {
    console.info('Cancelling message for thread:', currentThreadId.value);
    
    // Call the cancel API
    const result = await cancelMessage(currentThreadId.value);
    console.debug('Cancel result:', result);

    // Update the UI state
    cancelStreaming();
    
  } catch (error) {
    console.error('Error cancelling message:', error);
    // Still update UI state even if API call fails
    cancelStreaming();
  }
}
</script>

<template>
  <section v-if="messages.length || showStreamingMessage">
    <template v-for="message in messages" :key="message._id">
      <UserMessage v-if="message.role === 'user'" :message />
      <AssistantMessage v-else :message />
    </template>

    <StreamingMessage 
      v-if="showStreamingMessage" 
      :message="streamingMessage" 
      :cancelled="cancelled"
      :can-cancel="canCancelMessage"
      @cancel="handleCancelMessage"
    />
  </section>
  <section v-else-if="error">
    <p>Error loading messages: {{ error.message }}</p>
  </section>
  <section v-else>
    <p>Loading messages...</p>
  </section>
</template>
