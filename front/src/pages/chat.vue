<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue';
import Chatbox from '@/components/Chatbox.vue';
import LoadingDots from '@/components/LoadingDots.vue';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { useMutation } from '@/composables/convex';
import { useSelectedModel } from '@/composables/selectedModel';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { api } from '@/convex/_generated/api';
import type { Id } from '@/convex/_generated/dataModel';
import { apiPostSse, cancelMessage } from '@/lib/api';
import { Routes, type AnnotationResponse, type CreateMessageRequest } from '@/lib/types';
import { useEventListener, useLocalStorage, useResizeObserver } from '@vueuse/core';
import { ChevronDownIcon } from 'lucide-vue-next';
import { SSE, type SSEvent } from 'sse.js';
import { computed, ref, useTemplateRef } from 'vue';
import { RouterView, useRoute, useRouter } from 'vue-router';
import { toast } from 'vue-sonner';

const route = useRoute();
const router = useRouter();
const threadId = computed(() => route.params.thread as string);
const isInThread = computed(() => threadId.value != null);

const createThreadMutation = useMutation(api.threads.create);
const createMessageMutation = useMutation(api.threads.createMessage);

const streamingMessage = useStreamingMessage();

const isWaitingForFirstChunk = ref(false);

const selected = useSelectedModel();

let eventSource: SSE | null = null;

async function onSend(message: string) {
  if (selected.model == null) {
    console.warn('No model selected, cannot send message');
    toast.error('Please select a model before sending a message');
    return;
  }

  scrollToBottom(true);

  const modelParams = { includeSearch: selected.searchEnabled, reasoningEffort: 'medium' };

  let activeThreadId: string;

  if (eventSource != null) {
    console.warn('Cancelling previous streaming message');

    if (streamingMessage.isStreaming) {
      streamingMessage.onStreamCancelled();
    }

    try {
      eventSource.close();
      eventSource = null;
    } catch (error) {
      console.error('Error closing previous SSE connection:', error);
    }

    streamingMessage.clear();
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

    streamingMessage.onStreamStarted(activeThreadId);

    let ended = false;

    // 0: text
    // 1: reasoning
    // 2: annotations
    // 3: error
    // 4: cancelled
    // 5: refusal
    // 6: end
    // 7: unauthorized

    type ChatEvent =
      | ['0', string]
      | ['1', string]
      | ['2', AnnotationResponse[]]
      | ['3']
      | ['4']
      | ['5', string]
      | ['6']
      | ['7'];

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
          streamingMessage.addResponseChunk(value);
          break;
        }
        case '1': {
          streamingMessage.addReasoningChunk(value);
          break;
        }
        case '2': {
          // todo: annotations
          break;
        }
        case '3': {
          console.error('Error in SSE stream');
          streamingMessage.onStreamFailed();
          break;
        }
        case '4': {
          console.warn('SSE stream cancelled');
          streamingMessage.onStreamCancelled();
          break;
        }
        case '5': {
          console.warn('SSE stream refusal:', value);
          streamingMessage.onStreamCancelled();
          break;
        }
        case '6': {
          console.info('SSE stream ended');
          ended = true;
          streamingMessage.onStreamCompleted();
          eventSource?.close();
          break;
        }
        case '7': {
          console.error('Unauthorized SSE stream:', value);
          streamingMessage.onStreamFailed();
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
      streamingMessage.onStreamCompleted();
      eventSource?.close();
    });

    eventSource.addEventListener('error', (event: SSEvent) => {
      if (ended) return;
      ended = true;
      console.error('SSE error:', event);
      streamingMessage.onStreamFailed();
      eventSource?.close();
    });
  } catch (error) {
    console.error('Error sending message:', error);
    streamingMessage.onStreamFailed();
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

const sidebarOpen = useLocalStorage('sidebar-open', false);

const chatboxContainer = useTemplateRef('chatbox-container');

useResizeObserver(chatboxContainer, () => {
  if (chatboxContainer.value) {
    chatboxHeight.value = chatboxContainer.value.offsetHeight;
    scrollToBottom();
  }
});

const chatboxHeightStyle = computed(() => ({ '--chatbox-height': `${chatboxHeight.value}px` }));

const messagesContainer = useTemplateRef('messages-container');
const scrollTarget = useTemplateRef('scroll-target');

const notAtBottom = ref(false);

function scrollToBottom(force = false) {
  if (!notAtBottom.value && !force) return;

  if (messagesContainer.value && scrollTarget.value) {
    messagesContainer.value.scrollTo({
      top: scrollTarget.value.offsetTop,
      behavior: 'smooth',
    });
    notAtBottom.value = false;
  }
}

function checkForScroll(auto: boolean) {
  if (messagesContainer.value) {
    const containerHeight = messagesContainer.value.clientHeight;
    const scrollHeight = messagesContainer.value.scrollHeight;

    notAtBottom.value = !(messagesContainer.value.scrollTop + containerHeight >= scrollHeight - 256);

    if (notAtBottom.value && auto) {
      scrollToBottom();
    }
  }
}

useResizeObserver(messagesContainer, () => checkForScroll(true));
useEventListener(messagesContainer, 'scroll', () => checkForScroll(false));
</script>

<template>
  <SidebarProvider v-model:open="sidebarOpen">
    <AppSidebar :open="sidebarOpen" />

    <SidebarInset>
      <main class="chat">
        <div ref="messages-container" class="messages custom-scrollbar" :style="chatboxHeightStyle">
          <RouterView />

          <div v-if="isWaitingForFirstChunk" class="loading-indicator-container">
            <LoadingDots />
          </div>

          <div ref="scroll-target"></div>
        </div>

        <div v-if="notAtBottom" class="scroll-to-bottom-pill" @click="scrollToBottom(true)" :style="chatboxHeightStyle">
          <span>Scroll to bottom</span>
          <ChevronDownIcon />
        </div>

        <div ref="chatbox-container" class="chatbox-container">
          <Chatbox @send="onSend" @cancel="onCancel" />
        </div>
      </main>
    </SidebarInset>
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
  bottom: calc((var(--spacing) * 4) + var(--chatbox-height));
  left: 50%;
  transform: translateX(-50%);
  z-index: 20;

  animation: popIn 0.4s cubic-bezier(0.18, 0.89, 0.32, 1.28) forwards;

  display: flex;
  align-items: center;
  justify-content: center;
  gap: calc(var(--spacing) * 1.5);
  padding: calc(var(--spacing) * 1.5) calc(var(--spacing) * 3);

  border-radius: var(--radius-md);
  cursor: pointer;
  user-select: none;
  transition: transform 0.2s;

  color: var(--text-accent-foreground);

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
    background-color: color-mix(in oklab, var(--secondary) 50%, transparent);
  }

  &:hover {
    transform: translateX(-50%) scale(1.05);
  }

  > span {
    font-size: var(--text-sm);
    font-weight: var(--font-weight-medium);
  }

  > svg {
    height: calc(var(--spacing) * 5);
    width: calc(var(--spacing) * 5);
  }
}
</style>
