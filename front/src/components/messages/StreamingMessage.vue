<script setup lang="ts">
import Prose from '@/components/Prose.vue';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { useStreamingMessage } from '@/composables/streamingMessage';
import { computed } from 'vue';

const streamingMessage = useStreamingMessage();

const error = computed(() => {
  if (streamingMessage.failed) return 'Error generating response';
  return null;
});

const reasoning = computed(() => {
  const reasoning = streamingMessage.reasoning.value;

  if (reasoning == null || reasoning === '') return null;

  return reasoning;
});
</script>

<template>
  <div class="streaming-assistant-message">
    <div v-if="error != null" class="text-muted-foreground mb-2 px-4 pt-2 italic">{{ error }}</div>

    <template v-else>
      <Collapsible class="reasoning" v-if="reasoning != null">
        <CollapsibleTrigger>
          <span class="text-muted-foreground">Reasoning</span>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <Prose :source="reasoning" />
        </CollapsibleContent>
      </Collapsible>
      <Prose :source="streamingMessage.response.value" />
    </template>
  </div>
</template>

<style>
.streaming-assistant-message {
  color: var(--color-primary-foreground);
  margin-bottom: calc(var(--spacing) * 12);
}
</style>
