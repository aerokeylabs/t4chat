<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { ClipboardIcon, WrapTextIcon } from 'lucide-vue-next';

defineProps<{
  language: string;
  onCopy: () => void;
  onWrap: () => void;
}>();
</script>

<template>
  <div class="codeblock-header flex items-center justify-between gap-2">
    <span>{{ language }}</span>

    <div class="flex items-center gap-2">
      <Button variant="ghost" size="icon" @click="onWrap">
        <WrapTextIcon class="size-4" />
      </Button>

      <Button variant="ghost" size="icon" @click="onCopy">
        <ClipboardIcon class="size-4" />
      </Button>
    </div>
  </div>
</template>

<style>
.codeblock-header {
  display: flex;
  align-items: center;
  justify-content: space-between;

  gap: calc(var(--spacing) * 2);
  padding: calc(var(--spacing) * 2);
  padding-left: calc(var(--spacing) * 4);

  background-color: color-mix(in oklab, var(--color-secondary) 30%, transparent);

  border-top-left-radius: var(--radius-lg);
  border-top-right-radius: var(--radius-lg);

  > span {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-muted-foreground);
  }
}
</style>
