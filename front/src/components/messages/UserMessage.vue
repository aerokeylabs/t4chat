<script setup lang="ts">
import MessagePartAttachment from '@/components/messages/MessagePartAttachment.vue';
import MessagePartText from '@/components/messages/MessagePartText.vue';
import { Button } from '@/components/ui/button';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useRetryEventBus } from '@/composables/retryEventBus';
import type { UserMessage } from '@/lib/types/convex';
import { copyToClipboard } from '@/lib/utils';
import { CopyIcon, EditIcon, RefreshCcwIcon } from 'lucide-vue-next';

const props = defineProps<{
  message: UserMessage;
}>();

// todo: maybe move to function
// todo: maybe copy attachment url
function copy() {
  copyToClipboard(
    props.message.parts
      .filter((part) => part.type === 'text')
      .map((part) => part.text)
      .join('\n'),
  );
}

const eventBus = useRetryEventBus();

function retryMessage() {
  eventBus.emit(props.message._id);
}
</script>

<template>
  <div class="user-message">
    <div class="message-content">
      <template v-for="part in message.parts">
        <MessagePartText v-if="part.type === 'text'" :part />
        <MessagePartAttachment v-else-if="part.type === 'attachment'" :part />
      </template>
    </div>

    <div class="message-controls">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" @click="retryMessage">
            <RefreshCcwIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">Retry Message</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm">
            <EditIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">Edit Message</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" @click="copy">
            <CopyIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">Copy Message</TooltipContent>
      </Tooltip>
    </div>
  </div>
</template>

<style>
.user-message {
  display: flex;
  flex-direction: column;

  align-items: flex-end;
  align-self: flex-end;

  margin-bottom: calc(var(--spacing) * 12);

  &:hover > .message-controls {
    opacity: 1;
    pointer-events: auto;
  }

  > .message-content {
    display: flex;
    flex-direction: column;

    gap: calc(var(--spacing) * 4);

    background-color: var(--secondary);
    color: var(--secondary-foreground);
    border-radius: var(--radius);
    padding: calc(var(--spacing) * 4);
  }

  > .message-controls {
    opacity: 0;
    pointer-events: none;

    transition: opacity 0.2s ease-in-out;

    display: flex;
    flex-direction: row;
    align-items: center;
    gap: calc(var(--spacing) * 2);
    margin-top: calc(var(--spacing) * 2);

    svg {
      width: calc(var(--spacing) * 4);
      height: calc(var(--spacing) * 4);
      color: var(--color-secondary-foreground);
    }
  }
}
</style>
