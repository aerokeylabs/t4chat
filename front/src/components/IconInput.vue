<script setup lang="ts">
import type { HTMLAttributes } from 'vue';
import { useVModel } from '@vueuse/core';
import { Input } from '@/components/ui/input';

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
  class?: HTMLAttributes['class'];
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', payload: string | number): void;
}>();

const modelValue = useVModel(props, 'modelValue', emits, {
  passive: true,
  defaultValue: props.defaultValue,
});
</script>

<template>
  <div class="icon-input" :class="props.class">
    <Input v-model="modelValue" :default-value="defaultValue" v-bind="$attrs" />
    <span>
      <slot />
    </span>
  </div>
</template>

<style>
.icon-input {
  position: relative;

  width: 100%;
  max-width: var(--container-sm);

  align-items: center;

  > input {
    width: 100%;
    padding-left: calc(var(--spacing) * 7);
  }

  > span {
    position: absolute;
    display: flex;

    align-items: center;
    justify-content: center;

    padding-left: calc(var(--spacing) * 2);

    inset-block: 0;
    inset-inline-start: 0;

    > * {
      color: var(--muted-foreground);
      width: calc(var(--spacing) * 4);
      height: calc(var(--spacing) * 4);
    }
  }
}
</style>
