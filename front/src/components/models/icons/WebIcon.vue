<script setup lang="ts">
import BaseIcon from '@/components/models/icons/BaseIcon.vue';
import { GlobeIcon } from 'lucide-vue-next';
</script>

<template>
  <BaseIcon class="icon-web" tooltip="Uses search to answer questions"><GlobeIcon /></BaseIcon>
</template>

<style>
.icon-web {
  --color: oklch(0.78 0.06 245);
  --background-color: oklch(0.61 0.12 247 / 0.1);
}
</style>
