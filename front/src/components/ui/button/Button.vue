<script setup lang="ts">
import { cn } from '@/lib/utils';
import { Primitive, type PrimitiveProps } from 'reka-ui';
import { computed, type HTMLAttributes } from 'vue';
import { type ButtonVariants, buttonVariants } from '.';

interface Props extends PrimitiveProps {
  variant?: ButtonVariants['variant'];
  size?: ButtonVariants['size'];
  class?: HTMLAttributes['class'];
  active?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  as: 'button',
});

const activeClass = computed(() => {
  if (!props.active) return '';
  switch (props.variant) {
    case 'default':
      return 'bg-primary/50';
    case 'ghost':
    case 'outline':
    case 'secondary':
      return 'bg-secondary/50';
    case 'destructive':
      return 'bg-destructive/50';
    default:
      return '';
  }
});
</script>

<template>
  <Primitive
    data-slot="button"
    :as="as"
    :as-child="asChild"
    :class="cn(buttonVariants({ variant, size }), props.class, activeClass)"
  >
    <slot />
  </Primitive>
</template>
