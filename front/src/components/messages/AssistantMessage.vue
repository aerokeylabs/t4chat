<script setup lang="ts">
import Prose from '@/components/Prose.vue';
import type { AssistantMessage } from '@/lib/types/convex';

defineProps<{
  message: AssistantMessage;
}>();
</script>

<template>
  <div class="text-primary-foreground mb-12">
    <template v-for="part in message.parts">
      <Prose v-if="part.type === 'text'" :source="part.text" />
      <div v-else>{{ { part } }}</div>
    </template>
  </div>
</template>
