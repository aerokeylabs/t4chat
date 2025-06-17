<script setup lang="ts">
import BaseIcon from '@/components/models/icons/BaseIcon.vue';
import { FileTextIcon } from 'lucide-vue-next';
</script>

<template>
  <BaseIcon class="icon-file" tooltip="Supports PDF uploads and analysis"><FileTextIcon /></BaseIcon>
</template>

<style>
.icon-file {
  --color: oklch(0.73 0.12 281);
  --background-color: oklch(0.53 0.17 276 / 0.1);
}
</style>
