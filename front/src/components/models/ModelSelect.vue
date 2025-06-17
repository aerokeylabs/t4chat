<script setup lang="ts">
import IconInput from '@/components/IconInput.vue';
import { Button } from '@/components/ui/button';
import { useQuery, useReactiveQuery } from '@/composables/convex';
import { useSelectedModel } from '@/composables/selectedModel';
import { api } from '@/convex/_generated/api';
import { debouncedRef } from '@vueuse/core';
import { SearchIcon } from 'lucide-vue-next';
import { computed, ref } from 'vue';
import ModelListItem from './ModelListItem.vue';

const selected = useSelectedModel();
function selectModel(slug: string) {
  selected.slug = slug;
}

const { data: featured } = useQuery(api.models.getFeatured);

const query = ref('');
const debouncedQuery = debouncedRef(query, 300);

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
    <IconInput v-model="query" type="text" placeholder="Search models...">
      <SearchIcon />
    </IconInput>

    <div class="model-select-inner">
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
    </div>
  </div>
</template>

<style>
.model-select {
  display: flex;
  flex-direction: column;
  gap: calc(var(--spacing) * 2);

  height: 570px;
  width: 420px;

  .model-select-inner {
    min-height: 0;
    height: 100%;
    overflow-y: auto;

    display: flex;
    flex-direction: column;
  }
}
</style>
