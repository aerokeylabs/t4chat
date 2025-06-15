<script setup lang="ts">
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useSettings } from '@/composables/settings';

const { keys } = useSettings();
</script>

<template>
  <h1 class="text-2xl">API Keys</h1>
  <span class="text-sm">Bring your own API keys for select models.</span>

  <Label class="text-secondary-foreground mt-4 flex flex-col items-start gap-4">
    <span>OpenRouter Key</span>
    <Input v-model="keys.openrouter" placeholder="sk-..." maxlength="64" class="font-mono" />
  </Label>

  <pre>{{ keys }}</pre>
</template>
