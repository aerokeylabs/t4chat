<script setup lang="ts">
import AssistantMessage from '@/components/messages/AssistantMessage.vue';
import StreamingMessage from '@/components/messages/StreamingMessage.vue';
import UserMessage from '@/components/messages/UserMessage.vue';
import { useReactiveQuery } from '@/composables/convex';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
import type { Id } from '@/convex/_generated/dataModel';
import type { Message } from '@/lib/types/convex';
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const args = computed(() => ({ threadId: route.params.thread as Id<'threads'> }));

const { data, error } = useReactiveQuery(api.messages.getByThreadId, args);

const messages = computed(() => {
  if (!data.value) return [];
  return data.value.messages as Message[];
});

const streamingMessage = useStreamingMessage();

// show streaming message if it is not completed or if the last message is still pending
const showStreamingMessage = computed(() => {
  const lastMessage = messages.value.length === 0 ? null : messages.value[messages.value.length - 1];
  if (!streamingMessage.completed) return true;
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

    <StreamingMessage v-if="showStreamingMessage" />
  </section>
  <section v-else-if="error">
    <p>Error loading messages: {{ error.message }}</p>
  </section>
  <section v-else>
    <p>Loading messages...</p>
  </section>
</template>
