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
  <div class="flex items-center gap-2">
    <span>{{ displayModelName(model.name) }}</span>
    <Tooltip>
      <TooltipTrigger><InfoIcon /></TooltipTrigger>
      <TooltipContent class="max-w-sm">{{ model.description }}</TooltipContent>
    </Tooltip>
  </div>
  <div class="flex gap-2">
    <component :is="icon" v-for="icon in icons" />
  </div>
</template>
