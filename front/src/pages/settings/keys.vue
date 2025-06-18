<script setup lang="ts">
import SensitiveInput from '@/components/input/SensitiveInput.vue';
import { Label } from '@/components/ui/label';
import { useKeys } from '@/composables/keys';

const keys = useKeys();
</script>

<template>
  <h1 class="text-2xl">API Keys</h1>
  <span class="text-sm">Bring your own API keys for select models.</span>

  <Label class="text-secondary-foreground mt-4 flex flex-col items-start gap-4">
    <span>OpenRouter Key</span>
    <SensitiveInput v-model="keys.openrouter" placeholder="sk-..." maxlength="256" class="font-mono" />
  </Label>
</template>
