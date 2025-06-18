<script setup lang="ts">
import type { TextPart } from '@/lib/types/convex';

defineProps<{
  part: TextPart;
}>();
</script>

<template>
  <Prose :source="part.text" />
</template>
