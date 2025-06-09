<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue';
import { SidebarProvider } from '@/components/ui/sidebar';
import { useResizeObserver } from '@vueuse/core';
import { ref, useTemplateRef } from 'vue';
import Chatbox from './components/Chatbox.vue';
import type { Message } from './lib/types';
import messagesData from '@/lib/messages.json';

const messages = ref<Message[]>(messagesData as Message[]);

function onSend(content: string) {
  messages.value.push({
    id: String(messages.value.length + 1),
    content,
    role: 'user',
  });
}

const chatboxContainer = useTemplateRef('chatbox-container');
const chatboxHeight = ref(0);

useResizeObserver(chatboxContainer, () => {
  if (chatboxContainer.value) chatboxHeight.value = chatboxContainer.value.clientHeight;
});
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <main>
      <div :style="{ '--chatbox-height': `${chatboxHeight}px` }">
        <section>
          <template v-for="message in messages" :key="message.id">
            <div v-if="message.role === 'user'" class="mb-12 self-end">
              <p class="bg-secondary text-secondary-foreground prose rounded-lg p-4">{{ message.content }}</p>
            </div>
            <div v-else>
              <div class="mb-12">
                <p class="text-foreground prose">{{ message.content }}</p>
              </div>
            </div>
          </template>
        </section>
      </div>

      <div ref="chatbox-container" class="chatbox-container">
        <Chatbox @send="onSend" />
      </div>
    </main>
  </SidebarProvider>
</template>

<style>
main {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;

  width: 100%;
  height: 100vh;

  overflow: hidden;

  padding-bottom: calc(var(--spacing) * 4);

  .chatbox-container {
    position: absolute;
    bottom: calc(var(--spacing) * 4);
    z-index: 10;

    height: min-content;
    max-width: var(--container-4xl);

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

  > div {
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
}

.prose {
  font-size: 1rem;
  line-height: 1.75;
}
</style>
