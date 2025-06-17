<script setup lang="ts">
import BaseIcon from '@/components/models/icons/BaseIcon.vue';
import { EyeIcon } from 'lucide-vue-next';
</script>

<template>
  <BaseIcon class="icon-image" tooltip="Supports image uploads and analysis"><EyeIcon /></BaseIcon>
</template>

<style>
.icon-image {
  --color: oklch(0.86 0.07 180);
  --background-color: oklch(0.75 0.12 177 / 0.1);
}
</style>
