<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue';
import Chatbox from '@/components/Chatbox.vue';
import { SidebarProvider } from '@/components/ui/sidebar';
import { useMutation } from '@/composables/convex';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
import type { Id } from '@/convex/_generated/dataModel';
import { apiPostSse, cancelMessage } from '@/lib/api';
import { Routes, type CreateMessageRequest } from '@/lib/types';
import { SSE, type SSEvent } from 'sse.js';
import { computed, ref } from 'vue';
import { RouterView, useRoute, useRouter } from 'vue-router';
import { toast } from 'vue-sonner';

const route = useRoute();
const router = useRouter();
const threadId = computed(() => route.params.thread as string);
const isInThread = computed(() => threadId.value != null);

const createThreadMutation = useMutation(api.threads.create);
const createMessageMutation = useMutation(api.threads.createMessage);

const {
  message: streamingMessage,
  onStreamStarted,
  onStreamCompleted,
  onStreamCancelled,
  onStreamFailed,
  isStreaming,
  addChunk,
} = useStreamingMessage();

const model = ref('');

let eventSource: SSE | null = null;

async function onSend(message: string) {
  const modelParams = { includeSearch: false, reasoningEffort: 'medium' };

  let activeThreadId: string;

  if (eventSource != null) {
    console.warn('Cancelling previous streaming message');

    if (isStreaming.value) {
      onStreamCancelled();
    }

    try {
      eventSource.close();
      eventSource = null;
    } catch (error) {
      console.error('Error closing previous SSE connection:', error);
    }

    streamingMessage.value = '';
  }

  try {
    if (isInThread.value) {
      console.info('send message to thread', threadId.value, 'with content', message);
      const result = await createMessageMutation({
        threadId: threadId.value as Id<'threads'>,
        messageParts: [{ type: 'text', text: message }],
        model: model.value,
        modelParams,
      });

      if (result == null) {
        console.error('Failed to create message in thread', threadId.value);
        return;
      }

      console.debug('created message', result.assistantMessageId, 'in thread', threadId.value);

      eventSource = apiPostSse<CreateMessageRequest>(Routes.message(), {
        threadId: threadId.value,
        responseMessageId: result.assistantMessageId,
        messageParts: [{ type: 'text', text: message }],
        model: model.value,
        modelParams,
      });

      activeThreadId = threadId.value;
    } else {
      console.debug('create new thread with content', message);

      const thread = await createThreadMutation({ model: model.value, modelParams, message });

      console.debug('created thread', thread.threadId, 'with assistant message', thread.assistantMessageId);

      router.push(`/chat/${thread.threadId}`);

      eventSource = apiPostSse<CreateMessageRequest>(Routes.message(), {
        threadId: thread.threadId,
        responseMessageId: thread.assistantMessageId,
        messageParts: [{ type: 'text', text: message }],
        model: model.value,
        modelParams,
      });

      activeThreadId = thread.threadId;
    }

    onStreamStarted(activeThreadId);

    let ended = false;

    // 0: text
    // 1: error
    // 2: cancelled
    // 3: refusal
    // 4: end
    // 5: unauthorized
    type ChatEvent = ['0', string] | ['1'] | ['2'] | ['3', string] | ['4'] | ['5'];

    eventSource.addEventListener('message', (event: SSEvent) => {
      // event.data format is 'type:value'
      const [type, value] = event.data.split(':', 2) as ChatEvent;

      switch (type) {
        case '0': {
          addChunk(value);
          break;
        }
        case '1': {
          console.error('Error in SSE stream');
          onStreamFailed();
          break;
        }
        case '2': {
          console.warn('SSE stream cancelled');
          onStreamCancelled();
          break;
        }
        case '3': {
          console.warn('SSE stream refusal:', value);
          onStreamCancelled();
          break;
        }
        case '4': {
          console.info('SSE stream ended');
          ended = true;
          onStreamCompleted();
          eventSource?.close();
          break;
        }
        case '5': {
          console.error('Unauthorized SSE stream:', value);
          onStreamFailed();
          eventSource?.close();
          toast.error('Provider returned 401 Unauthorized for provided API key', {
            action: {
              label: 'Go to Settings',
              onClick: () => {
                router.push('/settings/keys');
              },
            },
          });
          break;
        }
      }
    });

    eventSource.addEventListener('end', (event: SSEvent) => {
      if (ended) return;
      ended = true;
      console.info('SSE stream ended:', event);
      onStreamCompleted();
      eventSource?.close();
    });

    eventSource.addEventListener('error', (event: SSEvent) => {
      if (ended) return;
      ended = true;
      console.error('SSE error:', event);
      onStreamFailed();
      eventSource?.close();
    });
  } catch (error) {
    console.error('Error sending message:', error);
    onStreamFailed();
    if (eventSource) {
      eventSource.close();
      eventSource = null;
    }
  }
}

async function onCancel() {
  const result = await cancelMessage(threadId.value);

  if (result.success) {
    console.info('Message cancelled successfully in thread', threadId.value);
  } else {
    console.warn('Failed to cancel message in thread', threadId.value);
  }
}

function onSelectModel(modelId: string) {
  model.value = modelId;
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
        <Chatbox @send="onSend" @cancel="onCancel" @select-model="onSelectModel" />
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
