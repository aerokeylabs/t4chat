<script setup lang="ts">
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';

defineProps<{
  tooltip: string;
}>();
</script>

<template>
  <Tooltip>
    <TooltipTrigger>
      <div class="model-icon" :class="$attrs.class" v-bind="$attrs">
        <slot></slot>
      </div>
    </TooltipTrigger>
    <TooltipContent>{{ tooltip }}</TooltipContent>
  </Tooltip>
</template>

<style>
.model-icon {
  border-radius: var(--radius);
  padding: 6px;

  color: var(--color);
  background-color: var(--background-color);
}
</style>
