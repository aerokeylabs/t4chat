import { useReactiveQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import { useLocalStorage } from '@vueuse/core';
import { defineStore } from 'pinia';
import { computed } from 'vue';

export const useSelectedModel = defineStore('selectedModel', () => {
  const slug = useLocalStorage<string | null>('selected-model', null);
  const args = computed(() => ({ slug: slug.value ?? '' }));
  const { data: model } = useReactiveQuery(api.models.getBySlug, args);

  const searchEnabled = useLocalStorage<boolean>('search-enabled', false);

  return { slug, model, searchEnabled };
});
