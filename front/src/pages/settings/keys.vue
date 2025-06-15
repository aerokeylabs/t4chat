<script setup lang="ts">
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useApiKeys } from '@/composables/keys';

const { openrouter } = useApiKeys();
</script>

<template>
  <h1 class="text-2xl">API Keys</h1>
  <span class="text-sm">Bring your own API keys for select models.</span>

  <Label class="text-secondary-foreground mt-4 flex flex-col items-start gap-4">
    <span>OpenRouter Key</span>
    <Input v-model="openrouter" placeholder="sk-..." class="font-mono" />
  </Label>
</template>
