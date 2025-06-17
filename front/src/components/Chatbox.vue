<script setup lang="ts">
import ModelSelect from '@/components/models/ModelSelect.vue';
import { Button } from '@/components/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { useChatbox } from '@/composables/chatbox';
import { useSelectedModel } from '@/composables/selectedModel';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { displayModelName } from '@/lib/utils';
import { useEventListener } from '@vueuse/core';
import { ChevronDownIcon, GlobeIcon, PaperclipIcon, SendIcon, StopCircleIcon, XIcon } from 'lucide-vue-next';
import { computed, nextTick, ref, useTemplateRef } from 'vue';

const selected = useSelectedModel();

const emit = defineEmits<{
  (e: 'send', message: string, encodedFiles?: Array<{name: string, type: string, size: number, data: string}>): void;
  (e: 'cancel'): void;
}>();

const textarea = useTemplateRef('textarea');
const fileInput = useTemplateRef<HTMLInputElement>('fileInput');

const { value: message } = useChatbox();
const { isStreaming } = useStreamingMessage();
const selectedFiles = ref<File[]>([]);

const messageValid = computed(() => {
  return message.value.trim() !== '' || selectedFiles.value.length > 0;
});

// Function to encode a file as base64
async function encodeFileAsBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      // The result is a data URL like "data:application/pdf;base64,JVBERi..."
      // We need to extract just the base64 part
      const base64String = reader.result as string;
      resolve(base64String);
    };
    reader.onerror = (error) => reject(error);
    reader.readAsDataURL(file);
  });
}

// Process files for sending
async function prepareFilesForSend(): Promise<Array<{name: string, type: string, size: number, data: string}>> {
  const encodedFiles = [];
  
  for (const file of selectedFiles.value) {
    try {
      const base64Data = await encodeFileAsBase64(file);
      encodedFiles.push({
        name: file.name,
        type: file.type,
        size: file.size,
        data: base64Data
      });
    } catch (error) {
      console.error('Failed to encode file:', file.name, error);
      // Continue with other files
    }
  }
  
  return encodedFiles;
}

async function send() {
  if (message.value.trim() === '' && selectedFiles.value.length === 0) return;

  if (isStreaming.value) {
    cancel();
  }

  let encodedFiles: Array<{name: string, type: string, size: number, data: string}> = [];
  if (selectedFiles.value.length > 0) {
    // Show some loading state if needed
    encodedFiles = await prepareFilesForSend();
  }

  emit('send', message.value, encodedFiles);

  message.value = '';
  selectedFiles.value = [];

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

function toggleSearch() {
  selected.searchEnabled = !selected.searchEnabled;
}

function handleFileSelect() {
  if (fileInput.value) {
    fileInput.value.click();
  }
}

function onFileInputChange(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files) {
    selectedFiles.value = [...selectedFiles.value, ...Array.from(target.files)];
    target.value = ''; // Reset input
  }
}

function removeFile(index: number) {
  selectedFiles.value.splice(index, 1);
}

// Helper function to format file size
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / 1048576).toFixed(1) + ' MB';
}
</script>

<template>
  <div class="chatbox">
    <div>
      <textarea ref="textarea" v-model="message" placeholder="Type your message here..."></textarea>
      
      <!-- File attachments display -->
      <div v-if="selectedFiles.length > 0" class="file-attachments">
        <div v-for="(file, index) in selectedFiles" :key="index" class="file-attachment">
          <div class="file-info">
            <span class="file-name">{{ file.name }}</span>
            <span class="file-size">{{ formatFileSize(file.size) }}</span>
          </div>
          <Button variant="ghost" size="icon-sm" @click="removeFile(index)" class="remove-file">
            <XIcon class="size-3" />
          </Button>
        </div>
      </div>
    </div>

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Popover v-model:open="selectModelOpen">
          <PopoverTrigger class="w-full">
            <Button variant="ghost" class="flex items-center pl-2 pr-1">
              <span>{{ displayModelName(selected.model?.name ?? '') }}</span>
              <ChevronDownIcon class="mt-0.5 size-4" />
            </Button>
          </PopoverTrigger>
          <PopoverContent class="flex w-min flex-col gap-2 p-2 bg-background" align="start">
            <ModelSelect />
          </PopoverContent>
        </Popover>

        <Button variant="outline" size="sm" :active="selected.searchEnabled" @click="toggleSearch">
          <GlobeIcon class="size-4" />
          <span class="ml-1">Search</span>
        </Button>

        <Button variant="outline" size="icon-sm" @click="handleFileSelect">
          <span class="sr-only">Add attachment</span>
          <PaperclipIcon class="size-4" />
          <input
            ref="fileInput"
            type="file"
            class="hidden"
            multiple
            @change="onFileInputChange"
          />
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

  .file-attachments {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 8px;
    max-height: 200px;
    overflow-y: auto;
  }

  .file-attachment {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 8px;
    background-color: rgba(0, 0, 0, 0.05);
    border-radius: var(--radius);
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 8px;
    max-width: calc(100% - 30px);
  }

  .file-name {
    font-size: 0.9rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 200px;
  }

  .file-size {
    font-size: 0.75rem;
    color: var(--color-muted);
  }

  .hidden {
    display: none;
  }

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
