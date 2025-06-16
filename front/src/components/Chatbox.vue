<script setup lang="ts">
import ModelSelect from '@/components/ModelSelect.vue';
import { Button } from '@/components/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { useChatbox } from '@/composables/chatbox';
import { useSelectedModel } from '@/composables/selectedModel';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { type ModelId } from '@/lib/models';
import { displayModelName } from '@/lib/utils';
import { useEventListener } from '@vueuse/core';
import { ChevronDownIcon, GlobeIcon, PaperclipIcon, SendIcon, StopCircleIcon } from 'lucide-vue-next';
import { computed, nextTick, ref, useTemplateRef } from 'vue';

const selected = useSelectedModel();

const emit = defineEmits<{
  (e: 'send', message: string): void;
  (e: 'cancel'): void;
  (e: 'select-model', model: ModelId): void;
}>();

const textarea = useTemplateRef('textarea');

const { value: message } = useChatbox();
const { isStreaming } = useStreamingMessage();

const messageValid = computed(() => {
  return message.value.trim() !== '';
});

function send() {
  if (message.value.trim() === '') return;

  if (isStreaming.value) {
    cancel();
  }

  emit('send', message.value);

  message.value = '';

  nextTick(updateTextareaHeight);
}

useEventListener(textarea, 'keydown', (event: KeyboardEvent) => {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();

    send();
  }
});

function updateTextareaHeight() {
  if (textarea.value == null) return;
  textarea.value.style.height = '0px';
  textarea.value.style.height = `${Math.max(textarea.value.scrollHeight, 64)}px`;
}

useEventListener(textarea, 'input', updateTextareaHeight);

const selectModelOpen = ref(false);

function cancel() {
  emit('cancel');
}
</script>

<template>
  <div class="chatbox">
    <textarea ref="textarea" v-model="message" placeholder="Type your message here..."></textarea>

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Popover v-model:open="selectModelOpen">
          <PopoverTrigger class="w-full">
            <Button variant="ghost" class="flex items-center pl-2 pr-1">
              <span>{{ displayModelName(selected.model?.name ?? '') }}</span>
              <ChevronDownIcon class="mt-0.5 size-4" />
            </Button>
          </PopoverTrigger>
          <PopoverContent class="flex w-min flex-col gap-2 p-2" align="start">
            <ModelSelect />
          </PopoverContent>
        </Popover>

        <Button variant="outline" size="sm">
          <GlobeIcon class="size-4" />
          <span class="ml-1">Search</span>
        </Button>

        <Button variant="outline" size="icon-sm">
          <span class="sr-only">Settings</span>
          <PaperclipIcon class="size-4" />
        </Button>
      </div>

      <Button v-if="isStreaming" size="icon" variant="ghost" @click="cancel">
        <StopCircleIcon class="size-5" />
      </Button>

      <Button v-else size="icon" variant="ghost" :disabled="!messageValid" @click="send">
        <SendIcon class="size-5" />
      </Button>
    </div>
  </div>
</template>

<style>
.chatbox {
  width: 100%;
  height: min-content;

  display: flex;
  flex-direction: column;

  padding: calc(var(--spacing) * 2);
  border-radius: var(--radius);

  gap: calc(var(--spacing) * 2);

  > textarea {
    appearance: none;
    resize: none;

    width: 100%;
    height: 100%;

    max-height: 256px;

    padding: calc(var(--spacing) * 2);

    &:focus {
      outline: none;
    }
  }
}
</style>
