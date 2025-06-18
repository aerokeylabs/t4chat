<script setup lang="ts">
import Prose from '@/components/Prose.vue';
import { Button } from '@/components/ui/button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useReactiveQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import type { AssistantMessage } from '@/lib/types/convex';
import { copyToClipboard, displayModelName } from '@/lib/utils';
import { ClockIcon, CopyIcon, CpuIcon, RefreshCcwIcon, SplitIcon, ZapIcon } from 'lucide-vue-next';
import moment from 'moment';
import { computed } from 'vue';

const props = defineProps<{
  message: AssistantMessage;
}>();

const args = computed(() => ({ id: props.message.model }));
const { data: model } = useReactiveQuery(api.models.getByOpenrouterId, args);
const modelName = computed(() => displayModelName(model.value?.name ?? 'Unknown Model'));

const floatFormat = new Intl.NumberFormat('en-US', {
  style: 'decimal',
  maximumFractionDigits: 2,
});
const intFormat = new Intl.NumberFormat('en-US', {
  style: 'decimal',
  maximumFractionDigits: 0,
});

function durationFormat(ms: number): string {
  return moment.duration(ms, 'millisecond').asSeconds().toFixed(2);
}

const metrics = computed(() => {
  if (props.message.status !== 'complete') return null;

  let tokensPerSecond = floatFormat.format(props.message.tokensPerSecond);
  let tokenCount = intFormat.format(props.message.tokenCount);
  let timeToFirstTokenMs = durationFormat(props.message.timeToFirstTokenMs);

  return { tokensPerSecond, tokenCount, timeToFirstTokenMs };
});

const reasoning = computed(() => {
  if (props.message.status !== 'complete') return null;

  const reasoning = props.message.reasoning;

  if (reasoning == null || reasoning === '') return null;

  return reasoning;
});

function copy() {
  copyToClipboard(props.message.parts.map((part) => part.text).join('\n'));
}
</script>

<template>
  <div class="assistant-message" v-if="message.status === 'complete'">
    <Collapsible v-if="reasoning != null" class="reasoning">
      <CollapsibleTrigger>
        <span class="text-muted-foreground">Reasoning</span>
      </CollapsibleTrigger>
      <CollapsibleContent>
        <Prose :source="reasoning" />
      </CollapsibleContent>
    </Collapsible>

    <template v-for="part in message.parts">
      <Prose v-if="part.type === 'text'" :source="part.text" />
      <div v-else>{{ { part } }}</div>
    </template>

    <div class="message-controls">
      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm" @click="copy">
            <CopyIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">Copy Message</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm">
            <SplitIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">Branch off</TooltipContent>
      </Tooltip>

      <Tooltip>
        <TooltipTrigger>
          <Button variant="ghost" size="icon-sm">
            <RefreshCcwIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">Retry Message</TooltipContent>
      </Tooltip>

      <span>{{ modelName }}</span>

      <template v-if="metrics != null">
        <span>
          <ZapIcon />
          {{ metrics.tokensPerSecond }} tok/sec
        </span>
        <span>
          <CpuIcon />
          {{ metrics.tokenCount }} tokens
        </span>
        <span>
          <ClockIcon />
          {{ metrics.timeToFirstTokenMs }} sec
        </span>
      </template>
    </div>
  </div>
</template>

<style>
.assistant-message {
  color: var(--color-primary-foreground);
  margin-bottom: calc(var(--spacing) * 12);

  &:hover > .message-controls {
    opacity: 1;
    pointer-events: auto;
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

    span {
      display: flex;
      align-items: center;
      gap: var(--spacing);

      color: var(--color-muted-foreground);
      font-size: var(--text-sm);

      svg {
        width: calc(var(--spacing) * 4);
        height: calc(var(--spacing) * 4);
        margin-top: 2px;
        color: var(--color-muted-foreground);
      }
    }

    svg {
      width: calc(var(--spacing) * 4);
      height: calc(var(--spacing) * 4);
      color: var(--color-secondary-foreground);
    }
  }
}
</style>
