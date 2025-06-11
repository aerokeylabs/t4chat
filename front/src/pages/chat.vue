<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue';
import Chatbox from '@/components/Chatbox.vue';
import Prose from '@/components/Prose.vue';
import { SidebarProvider } from '@/components/ui/sidebar';
import { useChatbox } from '@/composables/chatbox';
import { getApiUrl } from '@/lib/api';
import { Routes } from '@/lib/types';
import { SSE } from 'sse.js';
import { computed, ref } from 'vue';
import { RouterView, useRoute } from 'vue-router';

const route = useRoute();
const threadId = computed(() => route.params.thread as string);
const isInThread = computed(() => threadId.value != null);

// const createThreadMutation = useMutation(api.threads.createThread);

const stream = ref('');

const { hide } = useChatbox();

async function onSend(message: string) {
  if (isInThread.value) {
    console.info('send message to thread', threadId.value, 'with content', message);
  }

  console.info('create new thread with content', message);

  // const thread = await createThreadMutation({
  //   model: 'openai/gpt-4o-mini',
  //   modelParams: { includeSearch: false, reasoningEffort: 'medium' },
  //   message,
  // });

  // console.info('created thread', thread.threadId, 'with assistant message', thread.assistantMessageId);

  const source = new SSE(getApiUrl(Routes.chat()), {
    headers: {
      'Content-Type': 'application/json',
    },
    payload: JSON.stringify({
      threadId: '', // thread.threadId,
      responseMessageId: '', // thread.assistantMessageId,
      messageParts: [{ type: 'text', text: message }],
      model: 'openai/gpt-4o-mini',
      modelParams: { includeSearch: false, reasoningEffort: 'medium' },
    }),
    method: 'POST',
  });

  stream.value = '';
  hide.value = false;

  source.addEventListener('message', (event: { data: string }) => {
    stream.value += event.data;
    hide.value = true;
  });
}

const chatboxHeight = ref(300);
</script>

<template>
  <SidebarProvider>
    <AppSidebar />

    <main class="chat">
      <div class="messages" :style="{ '--chatbox-height': `${chatboxHeight}px` }">
        <RouterView />

        <div class="max-w-4xl p-4">
          <Prose :source="stream" />
        </div>
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
