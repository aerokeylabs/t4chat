<script setup lang="ts">
import IconInput from '@/components/IconInput.vue';
import { Button } from '@/components/ui/button';
import { useQuery, useReactiveQuery } from '@/composables/convex';
import { useSelectedModel } from '@/composables/selectedModel';
import { api } from '@/convex/_generated/api';
import { displayModelName } from '@/lib/utils';
import { debouncedRef } from '@vueuse/core';
import { SearchIcon } from 'lucide-vue-next';
import { computed, ref } from 'vue';

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

const displayedModels = computed(() => (hasQuery.value ? searchResults.value : featured.value)?.models ?? []);
</script>

<template>
  <div class="model-select">
    <IconInput v-model="query" type="text" placeholder="Search models...">
      <SearchIcon />
    </IconInput>

    <div class="model-select-inner bg-sidebar">
      <Button v-if="!hasQuery && selected.model != null" variant="ghost" class="bg-secondary w-full justify-start">
        {{ displayModelName(selected.model.name) }}
      </Button>

      <template v-for="m in displayedModels" :key="m.id">
        <Button
          variant="ghost"
          class="w-full justify-start"
          :class="{ 'bg-secondary': selected.slug === m.slug }"
          @click="selectModel(m.slug)"
        >
          {{ displayModelName(m.name) }}
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

  height: 480px;
  width: 360px;

  .model-select-inner {
    min-height: 0;
    height: 100%;
    overflow-y: auto;

    display: flex;
    flex-direction: column;
  }
}
</style>
