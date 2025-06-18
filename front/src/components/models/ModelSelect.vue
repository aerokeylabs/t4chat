<script setup lang="ts">
import IconInput from '@/components/input/IconInput.vue';
import ModelListItem from '@/components/models/ModelListItem.vue';
import { Button } from '@/components/ui/button';
import { useQuery, useReactiveQuery } from '@/composables/convex';
import { useSelectedModel } from '@/composables/selectedModel';
import { api } from '@/convex/_generated/api';
import { debouncedRef } from '@vueuse/core';
import { SearchIcon } from 'lucide-vue-next';
import { computed, ref } from 'vue';

const selected = useSelectedModel();
function selectModel(slug: string) {
  selected.slug = slug;
}

const { data: featured } = useQuery(api.models.getFeatured);

const query = ref('');
const debouncedQuery = debouncedRef(query, 150);

const args = computed(() => ({ query: debouncedQuery.value }));
const hasQuery = computed(() => debouncedQuery.value.trim() !== '');
const { data: searchResults } = useReactiveQuery(api.models.search, args, hasQuery);

const displayedModels = computed(() => {
  const models = (hasQuery.value ? searchResults.value : featured.value)?.models ?? [];
  const uniqueSlugs = new Set();

  if (!hasQuery.value && selected.model?.slug) {
    uniqueSlugs.add(selected.model.slug);
  }

  return models.filter((model) => {
    if (uniqueSlugs.has(model.slug)) return false;
    uniqueSlugs.add(model.slug);
    return true;
  });
});
</script>

<template>
  <div class="model-select">
    <div class="model-select-inner">
      <div class="model-list custom-scrollbar">
          <Button
            v-if="!hasQuery && selected.model != null"
            variant="ghost"
            size="xl"
            class="bg-secondary w-full justify-between"
          >
            <ModelListItem :model="selected.model" />
          </Button>

          <template v-for="model in displayedModels" :key="model.id">
            <Button
              variant="ghost"
              size="xl"
              class="w-full justify-between"
              :class="{ 'bg-secondary': selected.slug === model.slug }"
              @click="selectModel(model.slug)"
            >
              <ModelListItem :model />
            </Button>
          </template>

          <div v-if="displayedModels.length === 0" class="flex w-full">
            <span class="text-muted-foreground w-full p-4 text-center">
              {{ hasQuery ? 'No models found' : 'No models available' }}
            </span>
          </div>
      </div>
      
      <div class="search-container">
        <IconInput v-model="query" type="text" placeholder="Search all OpenRouter models...">
          <SearchIcon />
        </IconInput>
      </div>
    </div>
  </div>
</template>

<style>
.model-select {
  width: 100%;
  max-width: 480px;

  .model-select-inner {
    display: flex;
    flex-direction: column;
    gap: calc(var(--spacing) * 2);
    max-height: 70vh;
    overflow: hidden;
  }

  .model-list {
    overflow-y: auto;
    flex: 1;
    min-height: 0;
  }

  .search-container {
    padding: 0 calc(var(--spacing) * 2) calc(var(--spacing) * 2);
    background: var(--background);
    position: sticky;
    bottom: 0;
    border-top: 1px solid var(--border);
    padding-top: calc(var(--spacing) * 2);
  }
}
</style>
