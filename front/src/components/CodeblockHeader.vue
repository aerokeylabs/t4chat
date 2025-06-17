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
  <div class="codeblock-header">
    <span>{{ language }}</span>

    <div>
      <Button variant="ghost" size="icon-sm" @click="onWrap">
        <WrapTextIcon class="size-4" />
      </Button>

      <Button variant="ghost" size="icon-sm" @click="onCopy">
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
  padding: var(--spacing);
  padding-left: calc(var(--spacing) * 4);

  background-color: color-mix(in oklab, var(--color-sidebar) 50%, transparent);
  color: var(--color-sidebar-foreground);

  border-top-left-radius: var(--radius-lg);
  border-top-right-radius: var(--radius-lg);

  border: 1px solid var(--color-border);
  border-bottom: none;

  > span {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-muted-foreground);
  }
}
</style>
