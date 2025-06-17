<script setup lang="ts">
import { ICONS } from '@/components/models/icons';
import type { Model } from '@/lib/types/convex';
import { displayModelName } from '@/lib/utils';
import { InfoIcon } from 'lucide-vue-next';
import { computed } from 'vue';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';

const props = defineProps<{
  model: Model;
}>();

const icons = computed(() => {
  const icons = [ICONS.web];

  if (props.model.image) icons.push(ICONS.image);
  if (props.model.reasoning) icons.push(ICONS.reasoning);
  if (props.model.speed > 100) icons.push(ICONS.fast);

  return icons;
});
</script>

<template>
  <div class="model-item">
    <div>
      <span>{{ displayModelName(model.name) }}</span>

      <Tooltip>
        <TooltipTrigger><InfoIcon class="text-muted-foreground" /></TooltipTrigger>
        <TooltipContent class="max-w-[300px] text-center">{{ model.description }}</TooltipContent>
      </Tooltip>
    </div>
    <div>
      <component :is="icon" v-for="icon in icons" />
    </div>
  </div>
</template>

<style>
.model-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  gap: calc(var(--spacing) * 2);

  > div {
    display: flex;
    align-items: center;
    gap: calc(var(--spacing) * 2);

    > span {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
}
</style>
