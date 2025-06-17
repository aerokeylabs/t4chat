<script setup lang="ts">
import { mouseEventToRange, roundToStep } from '@/lib/utils';
import { useVModel } from '@vueuse/core';
import { onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  modelValue?: number;
  defaultValue?: number;
  min: number;
  max: number;
  step?: number;
}>();

const emits = defineEmits<(e: 'update:modelValue', payload: number) => void>();

const modelValue = useVModel(props, 'modelValue', emits, {
  passive: true,
  defaultValue: props.defaultValue,
});

const slider = useTemplateRef('slider');

const left = ref(0);

function update() {
  if (!slider.value) return;
  left.value = Math.max(props.min, Math.min(1, (modelValue.value ?? 0) / props.max)) * slider.value.clientWidth;
}

watch(modelValue, update);

let isMouseDown = false;

function onMouseMove(e: MouseEvent) {
  if (!isMouseDown || !slider.value) return;
  modelValue.value = roundToStep(mouseEventToRange(e, slider.value, props.min, props.max), props.step ?? 1);
  update();
}

function onMouseDown(e: MouseEvent) {
  e.preventDefault();

  isMouseDown = true;
  onMouseMove(e);
}

function onMouseUp() {
  isMouseDown = false;
}

onMounted(() => {
  update();

  slider.value?.addEventListener('mousedown', onMouseDown);

  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);
});

onUnmounted(() => {
  slider.value?.removeEventListener('mousedown', onMouseDown);

  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', onMouseUp);
});
</script>

<template>
  <div ref="slider" class="slider" :style="{ '--x': `${left}px` }" />
</template>

<style>
.slider {
  --x: 0;

  position: relative;
  width: 100%;
  height: 8px;
  margin: 8px 0;
  z-index: 100;
  border-radius: 4px;
  pointer-events: all;

  &::before {
    content: '';
    position: absolute;
    left: -4px;
    right: -4px;
    top: -4px;
    bottom: -4px;

    height: 16px;
  }

  &::after {
    content: '';
    position: absolute;
    top: 0;
    left: var(--x);
    width: 8px;
    height: 16px;
    border-radius: 4px;
    background-color: white;
    box-shadow: var(--shadow-floating);
    transform: translateX(-50%) translateY(-4px);
    pointer-events: none;

    /* slider gets weirdly laggy with rapidly changing values and transition */
    /* transition: left 40ms ease-in-out; */
  }
}
</style>
