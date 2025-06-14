<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue';
import Chatbox from '@/components/Chatbox.vue';
import { SidebarProvider } from '@/components/ui/sidebar';
import { useChatbox } from '@/composables/chatbox';
import { useMutation } from '@/composables/convex';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
import type { Id } from '@/convex/_generated/dataModel';
import { apiPostSse } from '@/lib/api';
import { Routes, type CreateMessageRequest } from '@/lib/types';
import type { SSE } from 'sse.js';
import { computed, ref } from 'vue';
import { RouterView, useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const threadId = computed(() => route.params.thread as string);
const isInThread = computed(() => threadId.value != null);

const createThreadMutation = useMutation(api.threads.create);
const createMessageMutation = useMutation(api.threads.createMessage);

const {
  message: streamingMessage,
  startStreaming,
  completeStreaming,
} = useStreamingMessage();

const { hide } = useChatbox();

// eslint-disable-next-line @typescript-eslint/no-unused-vars
let currentSseSource: SSE | null = null;

async function onSend(message: string) {
  const model = 'openai/gpt-4o-mini';
  const modelParams = { includeSearch: false, reasoningEffort: 'medium' };

  let source: SSE;
  let activeThreadId: string;

  if (isInThread.value) {
    console.info('send message to thread', threadId.value, 'with content', message);
    const result = await createMessageMutation({
      threadId: threadId.value as Id<'threads'>,
      messageParts: [{ type: 'text', text: message }],
      model,
      modelParams,
    });

    if (result == null) {
      console.error('Failed to create message in thread', threadId.value);
      return;
    }

    console.debug('created message', result.assistantMessageId, 'in thread', threadId.value);

    source = apiPostSse<CreateMessageRequest>(Routes.message(), {
      threadId: threadId.value,
      responseMessageId: result.assistantMessageId,
      messageParts: [{ type: 'text', text: message }],
      model,
      modelParams,
    });
    
    activeThreadId = threadId.value;
  } else {
    console.debug('create new thread with content', message);

    const thread = await createThreadMutation({ model, modelParams, message });

    console.debug('created thread', thread.threadId, 'with assistant message', thread.assistantMessageId);

    router.push(`/chat/${thread.threadId}`);

    source = apiPostSse<CreateMessageRequest>(Routes.message(), {
      threadId: thread.threadId,
      responseMessageId: thread.assistantMessageId,
      messageParts: [{ type: 'text', text: message }],
      model,
      modelParams,
    });
    
    activeThreadId = thread.threadId;
  }

  try {
    startStreaming(activeThreadId);
    hide.value = false;
    currentSseSource = source;

    source.addEventListener('message', (event: { data: string }) => {
      streamingMessage.value += event.data;
      hide.value = true;
    });

    source.addEventListener('end', () => {
      completeStreaming();
      currentSseSource = null;
    });

    source.addEventListener('error', (error: any) => {
      console.error('SSE error:', error);
      completeStreaming();
      currentSseSource = null;
    });

  } catch (error) {
    console.error('Error sending message:', error);
    completeStreaming();
    streamingMessage.value = '';
    currentSseSource = null;
  }
}

const chatboxHeight = ref(300);
</script>

<template>
  <SidebarProvider>
    <AppSidebar />

    <main class="chat">
      <div class="messages" :style="{ '--chatbox-height': `${chatboxHeight}px` }">
        <RouterView />
      </div>

      <div ref="chatbox-container" class="chatbox-container">
        <Chatbox @send="onSend" />
      </div>
    </main>
  </SidebarProvider>
</template>

<style>
.chat {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;

  width: 100%;
  height: 100vh;

  overflow: hidden;

  padding-bottom: calc(var(--spacing) * 4);
}

.prose {
  font-size: 1rem;
  line-height: 1.75;
}

.messages {
  display: flex;
  flex-direction: column;
  align-items: center;

  overflow-y: auto;
  overflow-x: hidden;

  width: 100%;
  height: 100%;
  min-height: 0;

  padding-top: calc(var(--spacing) * 32);
  padding-bottom: var(--chatbox-height);

  scrollbar-width: thin;

  > section {
    width: 100%;
    max-width: var(--container-4xl);
    padding: 0 calc(var(--spacing) * 4);

    display: flex;
    flex-direction: column;
  }
}

.chatbox-container {
  position: absolute;
  bottom: calc(var(--spacing) * 4);
  z-index: 10;

  height: min-content;
  max-width: var(--container-4xl);
  width: 100%;

  padding: 12px;

  &::before {
    content: '';

    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: -10;

    backdrop-filter: blur(18px) saturate(1.5);
    clip-path: inset(12px 12px 12px 12px round 12px);
    transition: background 0.4s;
    will-change: opacity;

    background-color: color-mix(in oklab, var(--secondary) 30%, transparent);
  }
}
</style>
