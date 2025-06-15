<script setup lang="ts">
import AssistantMessage from '@/components/messages/AssistantMessage.vue';
import StreamingMessage from '@/components/messages/StreamingMessage.vue';
import UserMessage from '@/components/messages/UserMessage.vue';
import { useReactiveQuery } from '@/composables/convex';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
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

const { message: streamingMessage, completed, failed } = useStreamingMessage();

// show streaming message if it is not completed or if the last message is still pending
const showStreamingMessage = computed(() => {
  const lastMessage = messages.value.length === 0 ? null : messages.value[messages.value.length - 1];
  if (!completed.value) return true;
  if (lastMessage?.role === 'assistant' && lastMessage.status === 'complete') return false;
  return true;
});

const streamError = computed(() => {
  if (failed.value) return 'Error generating response';
  return null;
});
</script>

<template>
  <section v-if="messages.length || showStreamingMessage">
    <template v-for="message in messages" :key="message._id">
      <UserMessage v-if="message.role === 'user'" :message />
      <AssistantMessage v-else :message />
    </template>

    <StreamingMessage v-if="showStreamingMessage" :message="streamingMessage" :error="streamError" />
  </section>
  <section v-else-if="error">
    <p>Error loading messages: {{ error.message }}</p>
  </section>
  <section v-else>
    <p>Loading messages...</p>
  </section>
</template>
