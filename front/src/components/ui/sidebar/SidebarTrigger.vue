<script setup lang="ts">
import type { HTMLAttributes } from 'vue';
import { PanelLeft } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { useSidebar } from './utils';

defineProps<{
  class?: HTMLAttributes['class'];
}>();

const { toggleSidebar } = useSidebar();
</script>

<template>
  <Button
    data-sidebar="trigger"
    data-slot="sidebar-trigger"
    variant="ghost"
    size="icon-sm"
    :class
    @click="toggleSidebar"
  >
    <PanelLeft />
    <span class="sr-only">Toggle Sidebar</span>
  </Button>
</template>
