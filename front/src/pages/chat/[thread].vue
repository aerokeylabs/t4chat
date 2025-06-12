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

const { data: dawa, error } = useReactiveQuery(api.messages.getMessagesByThread, args);

const messages = computed(() => {
  if (!dawa.value) return [];
  return dawa.value.messages as Message[];
});

const { message: streamingMessage, completed } = useStreamingMessage();

// show streaming message if it is not completed and if the last message in messages is no longer pending
const showStreamingMessage = computed(() => {
  const lastMessage = messages.value.length === 0 ? null : messages.value[messages.value.length - 1];
  if (!completed) return true;
  if (lastMessage?.role === 'assistant' && lastMessage.status === 'complete') return false;
  return true;
});
</script>

<template>
  <section v-if="messages.length || showStreamingMessage">
    <template v-for="message in messages" :key="message._id">
      <UserMessage v-if="message.role === 'user'" :message />
      <AssistantMessage v-else :message />
    </template>

    <StreamingMessage v-if="showStreamingMessage" :message="streamingMessage" />
  </section>
  <section v-else-if="error">
    <p>Error loading messages: {{ error.message }}</p>
  </section>
  <section v-else>
    <p>Loading messages...</p>
  </section>
</template>
