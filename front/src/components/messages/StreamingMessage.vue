<script setup lang="ts">
import Prose from '@/components/Prose.vue';

defineProps<{
  message: string;
  error?: string | null;
}>();
</script>

<template>
  <div class="text-primary-foreground relative mb-12">
    <div v-if="error != null" class="text-muted-foreground mb-2 px-4 pt-2 italic">{{ error }}</div>
    <Prose v-else :source="message" />
  </div>
</template>
