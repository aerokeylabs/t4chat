<script setup lang="ts">
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Input } from '@/components/ui/input';
import { useVModel } from '@vueuse/core';
import { ChevronDownIcon, CopyIcon, EyeIcon, EyeOffIcon } from 'lucide-vue-next';
import { ref, type HTMLAttributes } from 'vue';
import { toast } from 'vue-sonner';

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

const concealed = ref(true);

function copy() {
  navigator.clipboard.writeText(modelValue.value as string).catch((error) => {
    console.error('failed to copy input value:', error);
    toast.error('Failed to copy input value');
  });
}
</script>

<template>
  <div class="sensitive-input" :class="props.class">
    <Input
      v-model="modelValue"
      :default-value="defaultValue"
      v-bind="$attrs"
      :type="concealed ? 'password' : 'text'"
      data-1p-ignore
    />
    <DropdownMenu>
      <DropdownMenuTrigger>
        <EyeOffIcon v-if="concealed" />
        <EyeIcon v-else />

        <ChevronDownIcon />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" side="bottom">
        <DropdownMenuItem @click="concealed = !concealed">
          <EyeIcon v-if="concealed" />
          <EyeOffIcon v-else />

          {{ concealed ? 'Reveal' : 'Conceal' }}
        </DropdownMenuItem>
        <DropdownMenuItem @click="copy">
          <CopyIcon />
          <span>Copy</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
</template>

<style>
.sensitive-input {
  position: relative;

  width: 100%;

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;

  > input {
    width: 100%;
    padding-right: calc(var(--spacing) * 14);
  }

  > [data-slot='dropdown-menu-trigger'] {
    position: absolute;
    padding-right: calc(var(--spacing) * 2);
    inset-block: 0;
    inset-inline-end: 0;

    display: flex;
    align-items: center;
    gap: var(--spacing);

    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    padding: 0 calc(var(--spacing) * 1.5);
    margin: var(--spacing);

    cursor: pointer;

    &:hover,
    &[data-state='open'] {
      border-color: var(--input);
    }

    transition: border-color 0.2s ease-in-out;

    > * {
      color: var(--muted-foreground);
      width: calc(var(--spacing) * 3);
      height: calc(var(--spacing) * 3);
    }
  }
}
</style>
