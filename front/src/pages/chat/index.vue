<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { useUser } from '@clerk/vue';
import { CodeIcon, GraduationCapIcon, NewspaperIcon, SparklesIcon } from 'lucide-vue-next';
import { computed, ref, type FunctionalComponent } from 'vue';

const { user } = useUser();

const defaultPrompts = [
  'How does AI work?',
  'Are black holes real?',
  'How many Rs are in the word "strawberry"?',
  'What is the meaning of life?',
];

const rawCategories: [string, [FunctionalComponent, string[]]][] = [
  [
    'Create',
    [
      SparklesIcon,
      [
        'Write a short story about a robot discovering emotions',
        'Help me outline a sci-fi novel set in a post-apocalyptic world',
        'Create a character profile for a complex villain with sympathetic motives',
        'Give me 5 creative writing prompts for flash fiction',
      ],
    ],
  ],
  [
    'Explore',
    [
      NewspaperIcon,
      [
        'Good books for fans of Rick Rubin',
        'Countries ranked by number of corgis',
        'Most successful companies in the world',
        'How much does Claude cost?',
      ],
    ],
  ],
  [
    'Code',
    [
      CodeIcon,
      [
        'Write code to invert a binary search tree in Python',
        "What's the difference between Promise.all and Promise.allSettled?",
        "Explain React's useEffect cleanup function",
        'Best practices for error handling in async/await',
      ],
    ],
  ],
  [
    'Learn',
    [
      GraduationCapIcon,
      [
        "Beginner's guide to TypeScript",
        'Explain the CAP theorem in distributed systems',
        'Why is AI so expensive?',
        'Are black holes real?',
      ],
    ],
  ],
];

const categories = new Map(rawCategories);

const selectedCategory = ref<string | null>(null);

function onCategorySelect(category: string) {
  selectedCategory.value = selectedCategory.value === category ? null : category;
}

const prompts = computed(() => {
  if (selectedCategory.value == null) return defaultPrompts;
  return categories.get(selectedCategory.value)?.[1] ?? defaultPrompts;
});
</script>

<template>
  <section class="new-chat-page">
    <h1 class="mb-2 text-3xl font-semibold">How can I help you, {{ user?.firstName ?? 'someone' }}?</h1>

    <div class="flex gap-2">
      <Button
        v-for="([category, [icon]], i) in categories"
        :key="i"
        variant="outline"
        @click="onCategorySelect(category)"
        :class="{ 'bg-secondary/50': selectedCategory === category }"
      >
        <component :is="icon" class="h-4 w-4" />
        {{ category }}
      </Button>
    </div>

    <div class="example-prompts">
      <div v-for="(prompt, i) in prompts" :key="i" class="w-full">
        <Button variant="ghost" class="w-full justify-start">
          {{ prompt }}
        </Button>
      </div>
    </div>
  </section>
</template>

<style>
.new-chat-page {
  display: flex;
  flex-direction: column;
  gap: calc(var(--spacing) * 6);

  > .example-prompts {
    display: flex;
    flex-direction: column;
    gap: calc(var(--spacing) * 2);

    > div {
      width: 100%;
      padding-bottom: calc(var(--spacing) * 2);

      &:not(:last-child) {
        border-bottom: 1px solid var(--border);
      }
    }
  }
}
</style>
