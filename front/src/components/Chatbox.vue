<script setup lang="ts">
import ModelSelect from '@/components/models/ModelSelect.vue';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useChatbox } from '@/composables/chatbox';
import { useSelectedModel, type ReasoningEffort } from '@/composables/selectedModel';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { displayModelName } from '@/lib/utils';
import { useDropZone, useEventListener, useFileDialog } from '@vueuse/core';
import {
  ChevronDownIcon,
  GlobeIcon,
  PaperclipIcon,
  SendIcon,
  Settings2Icon,
  SignalHighIcon,
  SignalLowIcon,
  SignalMediumIcon,
  SignalZeroIcon,
  StopCircleIcon,
  XIcon,
} from 'lucide-vue-next';
import { computed, nextTick, ref, toValue, useTemplateRef, watch } from 'vue';
import { toast } from 'vue-sonner';

const selected = useSelectedModel();

const emit = defineEmits<{
  (e: 'send', message: string, files?: File[]): void;
  (e: 'cancel'): void;
}>();

const textarea = useTemplateRef('textarea');

const { value: message } = useChatbox();
const streamingMessage = useStreamingMessage();
const selectedFiles = ref<File[]>([]);

const messageValid = computed(() => {
  return message.value.trim() !== '' || selectedFiles.value.length > 0;
});

async function send() {
  if (message.value.trim() === '' && selectedFiles.value.length === 0) return;

  if (streamingMessage.isStreaming) {
    cancel();
  }

  emit('send', message.value, toValue(selectedFiles));

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

const fileDropZone = useTemplateRef('file-drop-zone');

function tryAddFile(file: File) {
  if (file.size > FILE_SIZE_LIMIT_BYTES) {
    toast.error(`File size exceeds the limit of ${FILE_SIZE_LIMIT_MB} MB: ${file.name}`);
    return;
  }

  if (selectedFiles.value.some((f) => f.name === file.name)) {
    return;
  }

  selectedFiles.value.push(file);
}

const { isOverDropZone } = useDropZone(fileDropZone, {
  onDrop(files) {
    if (files == null) return;
    for (const file of files) tryAddFile(file);
  },
});

const { open: openFileDialog, onChange } = useFileDialog({
  accept: 'image/*,application/pdf',
  multiple: true,
});

watch(
  selectedFiles,
  () => {
    nextTick(updateTextareaHeight);
  },
  { deep: true },
);

const FILE_SIZE_LIMIT_MB = 50;
const FILE_SIZE_LIMIT_BYTES = FILE_SIZE_LIMIT_MB * 1024 * 1024;

onChange((files: FileList | null) => {
  if (files == null) return;
  for (const file of files) tryAddFile(file);
});

function removeFile(index: number) {
  selectedFiles.value.splice(index, 1);
}

// Helper function to format file size
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / 1048576).toFixed(1) + ' MB';
}

function setReasoningEffort(effort: ReasoningEffort) {
  selected.reasoningEffort = effort;
}
</script>

<template>
  <div ref="file-drop-zone" class="chatbox" :class="{ 'is-over': isOverDropZone }">
    <div class="input-area">
      <textarea ref="textarea" v-model="message" placeholder="Type your message here..."></textarea>

      <!-- File attachments display -->
      <div v-if="selectedFiles.length > 0" class="file-attachments">
        <div v-for="(file, index) in selectedFiles" :key="index" class="file-attachment">
          <div class="file-info">
            <Tooltip>
              <TooltipTrigger>
                <span class="file-name">{{ file.name }}</span>
              </TooltipTrigger>
              <TooltipContent>
                <span>{{ file.name }}</span>
              </TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger>
                <span class="file-size">{{ formatFileSize(file.size) }}</span>
              </TooltipTrigger>
              <TooltipContent>
                <span>{{ Intl.NumberFormat('en-us').format(file.size) }} bytes</span>
              </TooltipContent>
            </Tooltip>
          </div>

          <Button variant="ghost" size="icon-sm" @click="removeFile(index)" class="remove-file hover:text-red-500">
            <XIcon class="size-5" />
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
          <PopoverContent class="bg-background flex w-min flex-col gap-2 p-2" align="start">
            <ModelSelect />
          </PopoverContent>
        </Popover>

        <Button variant="outline" size="sm" :active="selected.searchEnabled" @click="toggleSearch">
          <GlobeIcon class="size-4" />
          <span class="ml-1">Search</span>
        </Button>

        <DropdownMenu>
          <DropdownMenuTrigger>
            <Button variant="outline" size="icon-sm">
              <Settings2Icon class="size-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start">
            <DropdownMenuGroup>
              <DropdownMenuLabel>Reasoning Effort</DropdownMenuLabel>
              <DropdownMenuItem @click="setReasoningEffort('high')" :active="selected.reasoningEffort === 'high'">
                <SignalHighIcon class="stroke-3 size-5" />
                <span>High</span>
              </DropdownMenuItem>
              <DropdownMenuItem @click="setReasoningEffort('medium')" :active="selected.reasoningEffort === 'medium'">
                <SignalMediumIcon class="stroke-3 size-5" />
                <span>Medium</span>
              </DropdownMenuItem>
              <DropdownMenuItem @click="setReasoningEffort('low')" :active="selected.reasoningEffort === 'low'">
                <SignalLowIcon class="stroke-3 size-5" />
                <span>Low</span>
              </DropdownMenuItem>
              <DropdownMenuItem @click="setReasoningEffort(null)" :active="selected.reasoningEffort == null">
                <SignalZeroIcon class="stroke-3 size-5" />
                <span>None</span>
              </DropdownMenuItem>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>

        <Button variant="outline" size="icon-sm" @click="openFileDialog">
          <span class="sr-only">Add attachment</span>
          <PaperclipIcon class="size-4" />
        </Button>
      </div>

      <Button v-if="streamingMessage.isStreaming" size="icon" variant="ghost" @click="cancel">
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

  &.is-over {
    outline: 2px dashed var(--color-accent);
  }

  > .input-area {
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

    > .file-attachments {
      display: flex;
      flex-direction: column;
      margin-top: calc(var(--spacing) * 2);
      padding: 0 calc(var(--spacing) * 2);
      max-height: 200px;
      overflow-y: auto;

      .file-attachment {
        display: flex;
        align-items: center;
        justify-content: space-between;

        padding: calc(var(--spacing) * 1.5) 0;
        border-bottom: 1px solid var(--color-border);

        &:last-child {
          border-bottom: none;
        }

        > .file-info {
          display: flex;
          flex-direction: row;
          align-items: center;
          gap: calc(var(--spacing) * 2);

          .file-name {
            font-size: var(--text-base);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 200px;
          }

          .file-size {
            font-size: var(--text-sm);
            color: var(--color-muted);
            /* to vertically align */
            margin-top: 2px;
          }
        }
      }
    }
  }
}
</style>
