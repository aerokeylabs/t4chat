<script setup lang="ts">
import MessagePartAttachment from '@/components/messages/MessagePartAttachment.vue';
import MessagePartText from '@/components/messages/MessagePartText.vue';
import Prose from '@/components/Prose.vue';
import { Button } from '@/components/ui/button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useReactiveQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import type { AssistantMessage } from '@/lib/types/convex';
import { copyToClipboard, displayModelName } from '@/lib/utils';
import { ChevronDownIcon, ClockIcon, CopyIcon, CpuIcon, RefreshCcwIcon, SplitIcon, ZapIcon } from 'lucide-vue-next';
import moment from 'moment';
import { computed } from 'vue';

const props = defineProps<{
  message: AssistantMessage;
}>();

const args = computed(() => ({ id: props.message.model }));
const { data: model } = useReactiveQuery(api.models.getByOpenrouterId, args);
const modelName = computed(() => displayModelName(model.value?.name ?? 'Unknown Model'));

const hasAnnotations = computed(() => {
  return props.message.annotations != null && props.message.annotations.length > 0;
});

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
</script>

<template>
  <div class="assistant-message" v-if="message.status === 'complete'">
    <div v-if="reasoning != null" class="reasoning-container">
      <Collapsible class="reasoning" :default-open="false">
        <CollapsibleTrigger class="reasoning-trigger">
          <span class="text-muted-foreground">Reasoning</span>
          <ChevronDownIcon />
        </CollapsibleTrigger>
        <CollapsibleContent class="reasoning-content">
          <div class="reasoning-text">
            <Prose :source="reasoning" />
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>

    <template v-for="part in message.parts">
      <MessagePartText v-if="part.type === 'text'" :part />
      <MessagePartAttachment v-else-if="part.type === 'attachment'" :part />
    </template>

    <div v-if="message.status === 'complete'" class="annotations" :class="{ 'has-annotations': hasAnnotations }">
      <div class="annotation-pills">
        <a
          v-for="(annotation, index) in message.annotations"
          :key="index"
          :href="annotation.url"
          target="_blank"
          rel="noopener noreferrer"
          class="annotation-pill"
          :title="annotation.content"
        >
          {{ annotation.title || 'Source ' + (index + 1) }}
        </a>
      </div>
    </div>

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

  .annotations {
    margin-top: calc(var(--spacing) * 3);
    font-size: var(--text-sm);
    padding-top: calc(var(--spacing) * 2);

    &.has-annotations {
      border-top: 1px solid var(--color-border);
    }

    .annotations-title {
      color: var(--color-muted-foreground);
      margin-bottom: calc(var(--spacing) * 1.5);
      font-weight: 500;
      font-size: var(--text-xs);
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }

    .annotation-pills {
      display: flex;
      flex-wrap: wrap;
      gap: var(--spacing);
      margin-top: calc(var(--spacing) * 1);
    }

    .annotation-pill {
      align-items: center;
      background-color: var(--color-secondary);
      color: var(--color-secondary-foreground);
      padding: calc(var(--spacing) * 0.75) calc(var(--spacing) * 1.5);

      border-radius: 6px;
      font-size: var(--text-xs);
      font-weight: 500;
      text-decoration: none;
      transition: all 0.2s ease;

      white-space: nowrap;
      text-overflow: ellipsis;
      overflow: hidden;

      max-width: 240px;
      border: 1px solid var(--color-border);
      box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);

      &:hover {
        background-color: var(--color-secondary-hover);
        transform: translateY(-1px);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      }

      &:active {
        transform: translateY(0);
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
      }

      &::before {
        content: '🔗';
        margin-right: 0.5em;
        opacity: 0.7;
      }
    }

    .annotations-loading {
      display: flex;
      gap: var(--spacing);
      align-items: center;
      color: var(--color-muted-foreground);
      font-size: var(--text-xs);

      .loading-dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background-color: var(--color-muted-foreground);
        opacity: 0.6;
        animation: pulse 1.5s infinite ease-in-out;

        &:nth-child(2) {
          animation-delay: 0.2s;
        }

        &:nth-child(3) {
          animation-delay: 0.4s;
        }
      }
    }

    .annotations-error {
      display: flex;
      align-items: center;
      gap: var(--spacing);
      color: var(--color-destructive);
      font-size: var(--text-xs);

      .retry-button {
        margin-left: var(--spacing);
        font-size: var(--text-xs);
        height: auto;
        padding: 0.25rem 0.5rem;
      }

      .error-text {
        display: flex;
        align-items: center;
        gap: 0.25rem;

        &::before {
          content: '⚠️';
          font-size: 1em;
        }
      }
    }

    .annotations-empty {
      color: var(--color-muted-foreground);
      font-style: italic;
      font-size: var(--text-xs);
      opacity: 0.8;
    }

    @keyframes pulse {
      0%,
      100% {
        opacity: 0.3;
        transform: scale(0.8);
      }
      50% {
        opacity: 1;
        transform: scale(1);
      }
    }
  }

  .reasoning-container {
    margin-bottom: calc(var(--spacing) * 4);

    .reasoning {
      .reasoning-trigger {
        display: flex;
        align-items: center;
        gap: var(--spacing);
        color: var(--color-muted-foreground);
        font-size: var(--text-sm);
        padding: calc(var(--spacing) * 1) 0;
        cursor: pointer;
        transition: color 0.2s ease;
        user-select: none;

        &:hover {
          color: var(--color-foreground);
        }

        svg {
          transition: transform 0.2s ease;
          width: 16px;
          height: 16px;
        }

        &[data-state='open'] svg {
          transform: rotate(180deg);
        }
      }

      .reasoning-content {
        .reasoning-text {
          color: var(--color-muted-foreground);
          font-size: var(--text-sm);
          line-height: 1.5;
          padding: calc(var(--spacing) * 2) 0;
          margin-left: calc(var(--spacing) * 2);
          border-left: 2px solid var(--color-border);
          padding-left: calc(var(--spacing) * 3);
        }
      }
    }

    .divider {
      height: 1px;
      background-color: var(--color-border);
      margin: calc(var(--spacing) * 2) 0;
    }
  }

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

.reasoning-text * {
  color: var(--color-muted-foreground) !important;
}
</style>
