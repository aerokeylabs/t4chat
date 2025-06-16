<script setup lang="ts">
import type { HTMLAttributes } from 'vue';
import { reactiveOmit } from '@vueuse/core';
import { TagsInputInput, type TagsInputInputProps, useForwardProps } from 'reka-ui';
import { cn } from '@/lib/utils';

const props = defineProps<TagsInputInputProps & { class?: HTMLAttributes['class'] }>();

const delegatedProps = reactiveOmit(props, 'class');

const forwardedProps = useForwardProps(delegatedProps);
</script>

<template>
  <TagsInputInput
    v-bind="forwardedProps"
    :class="cn('min-h-5 flex-1 bg-transparent px-1 text-sm focus:outline-none', props.class)"
  />
</template>
