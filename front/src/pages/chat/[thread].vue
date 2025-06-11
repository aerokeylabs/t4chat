<script setup lang="ts">
import AssistantMessage from '@/components/messages/AssistantMessage.vue';
import UserMessage from '@/components/messages/UserMessage.vue';
import { useReactiveQuery } from '@/composables/convex';
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
</script>

<template>
  <section v-if="messages.length">
    <template v-for="message in messages" :key="message._id">
      <UserMessage v-if="message.role === 'user'" :message="message" />
      <AssistantMessage v-else-if="message.role === 'assistant'" :message="message" />
    </template>
  </section>
  <section v-else-if="error">
    <p>Error loading messages: {{ error.message }}</p>
  </section>
  <section v-else>
    <p>Loading messages...</p>
  </section>
</template>
