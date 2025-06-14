<script setup lang="ts">
import IconInput from '@/components/IconInput.vue';
import { Button } from '@/components/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { useChatbox } from '@/composables/chatbox';
import { MODELS, type ModelId } from '@/lib/models';
import { useEventListener } from '@vueuse/core';
import { ChevronDownIcon, GlobeIcon, PaperclipIcon, SearchIcon, Send } from 'lucide-vue-next';
import { computed, nextTick, ref, useTemplateRef } from 'vue';

const FEATURED_MODEL_IDS: ModelId[] = [
  'openai/gpt-4o-mini',
  'google/gemini-2.0-flash-001',
  'anthropic/claude-3.7-sonnet',
  'google/gemini-2.5-pro-preview-05-06',
  'google/gemini-2.5-flash-preview',
  'anthropic/claude-sonnet-4',
  'deepseek/deepseek-chat-v3-0324:free',
  'deepseek/deepseek-chat-v3-0324',
  'google/gemini-2.5-flash-preview-05-20',
  'openai/gpt-4.1',
];

const FEATURED_MODELS = FEATURED_MODEL_IDS.map((modelId) => MODELS.get(modelId)!);

const model = ref(FEATURED_MODELS[0].id);

const props = defineProps<{
  disabled?: boolean;
}>();
const emit = defineEmits<{
  (e: 'send', message: string): void;
}>();

const textarea = useTemplateRef('textarea');

const { value: message } = useChatbox();

const messageValid = computed(() => {
  return message.value.trim() !== '';
});

function send() {
  if (message.value.trim() === '' || props.disabled) return;

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

// text after first :
function displayModelName(name: string) {
  const colonIndex = name.indexOf(':');
  return colonIndex !== -1 ? name.slice(colonIndex + 1).trim() : name;
}

const selectModelOpen = ref(false);

function selectModel(id: ModelId) {
  selectModelOpen.value = false;
  model.value = id;
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
              <span>{{ displayModelName(FEATURED_MODELS.find((m) => m.id === model)?.name || '') }}</span>
              <ChevronDownIcon class="mt-0.5 size-4" />
            </Button>
          </PopoverTrigger>
          <PopoverContent class="flex w-96 flex-col gap-2 p-2" align="start">
            <IconInput class="w-full" type="text" placeholder="Search models...">
              <SearchIcon />
            </IconInput>

            <div class="flex flex-col gap-2">
              <template v-for="m in FEATURED_MODELS" :key="m.id">
                <Button
                  variant="ghost"
                  class="w-full justify-start"
                  :class="{ 'bg-secondary': model === m.id }"
                  @click="selectModel(m.id)"
                >
                  {{ displayModelName(m.name) }}
                </Button>
              </template>
            </div>
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

      <Button size="icon" variant="ghost" :disabled="disabled || !messageValid" @click="send">
        <Send class="size-5" />
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
