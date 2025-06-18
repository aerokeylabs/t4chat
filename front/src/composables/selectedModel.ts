import { useReactiveQuery } from '@/composables/convex';
import { api } from '@/convex/_generated/api';
import { useLocalStorage } from '@vueuse/core';
import { defineStore } from 'pinia';
import { computed } from 'vue';

export type ReasoningEffort = null | 'low' | 'medium' | 'high';

export const useSelectedModel = defineStore('selectedModel', () => {
  const slug = useLocalStorage<string>('selected-model', 'google/gemini-2.5-flash-lite-preview-06-17');
  const args = computed(() => ({ slug: slug.value ?? '' }));
  const { data: model } = useReactiveQuery(api.models.getBySlug, args);

  const searchEnabled = useLocalStorage<boolean>('search-enabled', false);
  const reasoningEffort = useLocalStorage<ReasoningEffort>('reasoning-effort', null);

  return { slug, model, searchEnabled, reasoningEffort };
});
