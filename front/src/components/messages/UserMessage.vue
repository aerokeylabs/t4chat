<script setup lang="ts">
import Prose from '@/components/Prose.vue';
import type { UserMessage } from '@/lib/types/convex';

defineProps<{
  message: UserMessage;
}>();
</script>

<template>
  <div class="bg-secondary text-secondary-foreground mb-12 self-end rounded-lg p-4">
    <template v-for="part in message.parts">
      <Prose v-if="part.type === 'text'" :source="part.text" />
      <div v-else>{{ { part } }}</div>
    </template>
  </div>
</template>
