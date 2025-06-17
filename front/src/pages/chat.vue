<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue';
import Chatbox from '@/components/Chatbox.vue';
import LoadingDots from '@/components/LoadingDots.vue';
import { SidebarProvider } from '@/components/ui/sidebar';
import { useMutation } from '@/composables/convex';
import { useSelectedModel } from '@/composables/selectedModel';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
import type { Id } from '@/convex/_generated/dataModel';
import { apiPostSse, cancelMessage } from '@/lib/api';
import { Routes, type CreateMessageRequest } from '@/lib/types';
import { useLocalStorage } from '@vueuse/core';
import { SSE, type SSEvent } from 'sse.js';
import { computed, onMounted, ref, useTemplateRef } from 'vue';
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
  setMessagesContainer,
  scrollToBottom,
  showScrollToBottomPill,
  resetScrollState,
} = useStreamingMessage();

const isWaitingForFirstChunk = ref(false);

const selected = useSelectedModel();

let eventSource: SSE | null = null;

async function onSend(message: string) {
  if (selected.model == null) {
    console.warn('No model selected, cannot send message');
    toast.error('Please select a model before sending a message');
    return;
  }

  // Reset scroll state and force scroll to bottom when sending a new message
  resetScrollState();
  scrollToBottom(true);

  const modelParams = { includeSearch: selected.searchEnabled, reasoningEffort: 'medium' };

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
    // Set waiting for first chunk to true
    isWaitingForFirstChunk.value = true;

    if (isInThread.value) {
      console.info('send message to thread', threadId.value, 'with content', message);
      const modelSlug = selected.searchEnabled ? `${selected.model.slug}:online` : selected.model.slug;
      const result = await createMessageMutation({
        threadId: threadId.value as Id<'threads'>,
        messageParts: [{ type: 'text', text: message }],
        model: modelSlug,
        modelParams,
      });

      if (result == null) {
        console.error('Failed to create message in thread', threadId.value);
        return;
      }

      console.debug('created message', result.assistantMessageId, 'in thread', threadId.value);

      const modelId = selected.searchEnabled ? `${selected.model.id}:online` : selected.model.id;
      eventSource = apiPostSse<CreateMessageRequest>(Routes.message(), {
        threadId: threadId.value,
        responseMessageId: result.assistantMessageId,
        model: modelId,
        modelParams,
      });

      activeThreadId = threadId.value;
    } else {
      console.debug('create new thread with content', message);

      const modelSlug = selected.searchEnabled ? `${selected.model.slug}:online` : selected.model.slug;
      const thread = await createThreadMutation({ model: modelSlug, modelParams, message });

      console.debug('created thread', thread.threadId, 'with assistant message', thread.assistantMessageId);

      router.push(`/chat/${thread.threadId}`);

      const modelId = selected.searchEnabled ? `${selected.model.id}:online` : selected.model.id;
      eventSource = apiPostSse<CreateMessageRequest>(Routes.message(), {
        threadId: thread.threadId,
        responseMessageId: thread.assistantMessageId,
        model: modelId,
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
      const i = event.data.indexOf(':');

      if (i === -1) {
        console.error('Invalid SSE message format:', event.data);
        return;
      }

      const [type, value] = [event.data.slice(0, i), event.data.slice(i + 1)] as ChatEvent;

      switch (type) {
        case '0': {
          // Hide loading indicator when first chunk arrives
          if (isWaitingForFirstChunk.value) {
            isWaitingForFirstChunk.value = false;
          }
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

const chatboxHeight = ref(300);
const messagesContainer = useTemplateRef('messages-container');

onMounted(() => {
  setMessagesContainer(messagesContainer.value);
});

const sidebarOpen = useLocalStorage('sidebar-open', false);
</script>

<template>
  <SidebarProvider v-model:open="sidebarOpen">
    <AppSidebar :open="sidebarOpen" />

    <main class="chat">
      <div
        ref="messages-container"
        class="messages custom-scrollbar"
        :style="{ '--chatbox-height': `${chatboxHeight}px` }"
      >
        <RouterView />

        <div v-if="isWaitingForFirstChunk" class="loading-indicator-container">
          <LoadingDots />
        </div>
      </div>

      <div v-if="showScrollToBottomPill" class="scroll-to-bottom-pill" @click="scrollToBottom(true)">
        <span class="pill-text">Scroll to bottom</span>
        <span class="pill-icon">↓</span>
      </div>

      <div ref="chatbox-container" class="chatbox-container">
        <Chatbox @send="onSend" @cancel="onCancel" />
      </div>
    </main>
  </SidebarProvider>
</template>

<style>
:root {
  --chatbox-spacing: calc(var(--spacing) * 4);
  --scrollbar-width: 8px;
  --scrollbar-track: rgba(0, 0, 0, 0.05);
  --scrollbar-thumb: color-mix(in oklab, var(--secondary) 50%, transparent);
  --scrollbar-thumb-hover: color-mix(in oklab, var(--secondary) 70%, transparent);
}

/* Global custom scrollbar style */
.custom-scrollbar {
  /* Modern Firefox */
  scrollbar-width: thin;
  scrollbar-color: var(--scrollbar-thumb) var(--scrollbar-track);

  /* Webkit browsers (Chrome, Safari, Edge) */
  &::-webkit-scrollbar {
    width: var(--scrollbar-width);
    height: var(--scrollbar-width);
  }

  &::-webkit-scrollbar-track {
    background: var(--scrollbar-track);
    border-radius: 4px;
  }

  &::-webkit-scrollbar-thumb {
    background: var(--scrollbar-thumb);
    border-radius: 4px;
    backdrop-filter: blur(10px);
  }

  &::-webkit-scrollbar-thumb:hover {
    background: var(--scrollbar-thumb-hover);
  }
}

@keyframes popIn {
  0% {
    transform: translateX(-50%) scale(0.95);
  }
  60% {
    transform: translateX(-50%) scale(1.05);
  }
  100% {
    transform: translateX(-50%) scale(1);
  }
}

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

  /* Add custom scrollbar */
  @extend .custom-scrollbar;

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

.loading-indicator-container {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  width: 100%;
  max-width: var(--container-4xl);
  padding: 0 calc(var(--spacing) * 4);
  margin-bottom: 12px;
}

.scroll-to-bottom-pill {
  position: absolute;
  bottom: calc(var(--spacing) * 42);
  left: 50%;
  transform: translateX(-50%);
  z-index: 20;

  /* Animation properties */
  animation: popIn 0.4s cubic-bezier(0.18, 0.89, 0.32, 1.28) forwards;

  display: flex;
  align-items: center;
  justify-content: center;
  gap: calc(var(--spacing) * 1.5);
  padding: calc(var(--spacing) * 1.5) calc(var(--spacing) * 3);

  border-radius: 12px;
  cursor: pointer;
  user-select: none;
  transition: transform 0.2s;

  /* Match chatbox style with backdrop filter */
  &::before {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: -1;
    border-radius: inherit;
    backdrop-filter: blur(18px) saturate(1.5);
    background-color: color-mix(in oklab, var(--secondary) 30%, transparent);
  }

  &:hover {
    transform: translateX(-50%) scale(1.05);
  }

  .pill-text {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-accent-foreground);
  }

  .pill-icon {
    font-size: 1.25rem;
    font-weight: bold;
    color: var(--text-accent-foreground);
  }
}
</style>
