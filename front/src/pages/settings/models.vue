<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { useLocalStorage } from '@vueuse/core';

// Define model interface and feature enum
interface Model {
  id: string;
  name: string;
  provider: string;
  capabilities: string[];
  enabled: boolean;
}

// Mock models data - in production this would come from OpenRouter API
const useModelsStore = defineStore('models', () => {
  const models = useLocalStorage<Model[]>('models_list', [
    {
      id: 'openai/gpt-4o',
      name: 'GPT-4o',
      provider: 'OpenAI',
      capabilities: ['image-uploads', 'pdf-uploads', 'reasoning'],
      enabled: true,
    },
    {
      id: 'openai/gpt-4-turbo',
      name: 'GPT-4 Turbo',
      provider: 'OpenAI',
      capabilities: ['image-uploads', 'pdf-uploads', 'reasoning'],
      enabled: true,
    },
    {
      id: 'openai/gpt-3.5-turbo',
      name: 'GPT-3.5 Turbo',
      provider: 'OpenAI',
      capabilities: ['reasoning'],
      enabled: true,
    },
    {
      id: 'anthropic/claude-3-opus',
      name: 'Claude 3 Opus',
      provider: 'Anthropic',
      capabilities: ['image-uploads', 'pdf-uploads', 'reasoning'],
      enabled: false,
    },
    {
      id: 'anthropic/claude-3-sonnet',
      name: 'Claude 3 Sonnet',
      provider: 'Anthropic',
      capabilities: ['image-uploads', 'reasoning'],
      enabled: true,
    },
    {
      id: 'anthropic/claude-3-haiku',
      name: 'Claude 3 Haiku',
      provider: 'Anthropic',
      capabilities: ['image-uploads', 'reasoning'],
      enabled: false,
    },
    {
      id: 'google/gemini-pro',
      name: 'Gemini Pro',
      provider: 'Google',
      capabilities: ['image-uploads'],
      enabled: false,
    },
    {
      id: 'google/gemini-1.5-pro',
      name: 'Gemini 1.5 Pro',
      provider: 'Google',
      capabilities: ['image-uploads', 'pdf-uploads', 'reasoning'],
      enabled: false,
    },
    {
      id: 'meta/llama-3-70b-instruct',
      name: 'Llama 3 70B',
      provider: 'Meta',
      capabilities: ['reasoning'],
      enabled: false,
    },
  ]);

  function toggleModelStatus(modelId: string, status: boolean) {
    const model = models.value.find((m) => m.id === modelId);
    if (model) {
      model.enabled = status;
    }
  }

  return {
    models,
    toggleModelStatus,
  };
});

const modelsStore = useModelsStore();
const searchTerm = ref('');
const selectedCapabilities = ref<string[]>([]);
const selectedProviders = ref<string[]>([]);

// Get unique capabilities and providers for filtering
const capabilities = computed(() => {
  const allCapabilities = new Set<string>();
  modelsStore.models.forEach((model) => {
    model.capabilities.forEach((cap) => allCapabilities.add(cap));
  });
  return Array.from(allCapabilities);
});

const providers = computed(() => {
  const allProviders = new Set<string>();
  modelsStore.models.forEach((model) => {
    allProviders.add(model.provider);
  });
  return Array.from(allProviders);
});

// Filter models based on search term and selected filters
const filteredModels = computed(() => {
  return modelsStore.models.filter((model) => {
    // Filter by search term
    const matchesSearch =
      searchTerm.value === '' ||
      model.name.toLowerCase().includes(searchTerm.value.toLowerCase()) ||
      model.provider.toLowerCase().includes(searchTerm.value.toLowerCase());

    // Filter by capabilities
    const matchesCapabilities =
      selectedCapabilities.value.length === 0 ||
      selectedCapabilities.value.some((cap) => model.capabilities.includes(cap));

    // Filter by providers
    const matchesProviders = selectedProviders.value.length === 0 || selectedProviders.value.includes(model.provider);

    return matchesSearch && matchesCapabilities && matchesProviders;
  });
});

function toggleCapabilityFilter(capability: string) {
  const index = selectedCapabilities.value.indexOf(capability);
  if (index === -1) {
    selectedCapabilities.value.push(capability);
  } else {
    selectedCapabilities.value.splice(index, 1);
  }
}

function toggleProviderFilter(provider: string) {
  const index = selectedProviders.value.indexOf(provider);
  if (index === -1) {
    selectedProviders.value.push(provider);
  } else {
    selectedProviders.value.splice(index, 1);
  }
}

function clearFilters() {
  searchTerm.value = '';
  selectedCapabilities.value = [];
  selectedProviders.value = [];
}
</script>

<template>
  <div class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-bold">Models</h1>
      <p class="text-muted-foreground">Configure which models are available in the model selector.</p>
    </div>

    <div class="flex flex-col gap-6 md:flex-row">
      <!-- Filters sidebar -->
      <div class="w-full space-y-6 md:w-64">
        <div>
          <Label for="search">Search Models</Label>
          <Input id="search" v-model="searchTerm" placeholder="Search by name or provider..." class="mt-2" />
        </div>

        <div>
          <h3 class="mb-3 font-medium">Capabilities</h3>
          <div class="space-y-2">
            <div v-for="capability in capabilities" :key="capability" class="flex items-center gap-2">
              <Button
                variant="outline"
                :class="{
                  'bg-primary/10': selectedCapabilities.includes(capability),
                }"
                size="sm"
                @click="toggleCapabilityFilter(capability)"
              >
                {{
                  capability === 'pdf-uploads'
                    ? 'PDF Uploads'
                    : capability === 'image-uploads'
                      ? 'Image Uploads'
                      : capability === 'reasoning'
                        ? 'Has Reasoning Capabilities'
                        : capability
                }}
              </Button>
            </div>
          </div>
        </div>

        <div>
          <h3 class="mb-3 font-medium">Providers</h3>
          <div class="space-y-2">
            <div v-for="provider in providers" :key="provider" class="flex items-center gap-2">
              <Button
                variant="outline"
                :class="{
                  'bg-primary/10': selectedProviders.includes(provider),
                }"
                size="sm"
                @click="toggleProviderFilter(provider)"
              >
                {{ provider }}
              </Button>
            </div>
          </div>
        </div>

        <Button variant="ghost" size="sm" @click="clearFilters"> Clear all filters </Button>
      </div>

      <!-- Models list -->
      <div class="flex-1">
        <div v-if="filteredModels.length === 0" class="py-12 text-center">
          <p class="text-muted-foreground">No models match your filters.</p>
        </div>

        <div v-else class="grid gap-4">
          <Card v-for="model in filteredModels" :key="model.id" class="overflow-hidden">
            <CardContent class="p-4">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-medium">{{ model.name }}</h3>
                  <p class="text-muted-foreground text-sm">{{ model.provider }}</p>
                  <div class="mt-2 flex gap-2">
                    <span
                      v-for="capability in model.capabilities"
                      :key="capability"
                      class="bg-secondary rounded-full px-2 py-0.5 text-xs"
                    >
                      {{ capability }}
                    </span>
                  </div>
                </div>

                <div class="flex items-center">
                  <Switch
                    :checked="model.enabled"
                    @update:checked="(status: boolean) => modelsStore.toggleModelStatus(model.id, status)"
                  />
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  </div>
</template>
