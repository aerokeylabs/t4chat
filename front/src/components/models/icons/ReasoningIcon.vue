<script setup lang="ts">
import BaseIcon from '@/components/models/icons/BaseIcon.vue';
import { BrainIcon } from 'lucide-vue-next';
</script>

<template>
  <BaseIcon class="icon-reasoning" tooltip="Has reasoning capabilities"><BrainIcon /></BaseIcon>
</template>

<style>
.icon-reasoning {
  --color: oklch(0.74 0.11 301);
  --background-color: oklch(0.52 0.20 296 / 0.1);
}
</style>
