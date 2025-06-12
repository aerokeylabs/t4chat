<script setup lang="ts">
import Prose from '@/components/Prose.vue';

defineProps<{
  message: string;
}>();
</script>

<template>
  <div class="text-primary-foreground mb-12">
    <Prose :source="message" />
  </div>
</template>
