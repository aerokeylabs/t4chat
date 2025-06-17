<script setup lang="ts">
import { getMarkdownProcessor } from '@/lib/shiki';
import { useMutationObserver } from '@vueuse/core';
import { createApp, ref, useTemplateRef, watch } from 'vue';
import CodeblockHeader from './CodeblockHeader.vue';

const props = defineProps<{
  source: string;
}>();

const html = ref('');

watch(
  props,
  async (newProps) => {
    const processor = await getMarkdownProcessor();
    html.value = (await processor.process(newProps.source)).toString();
  },
  { immediate: true },
);

const component = useTemplateRef('component');

function addHeaderIfMissing(node: HTMLElement) {
  if (node.classList.contains('patched')) return;
  node.classList.add('patched');

  const language = node.getAttribute('data-language') || 'plaintext';

  if (node.querySelector('.codeblock-header')) return;

  const app = createApp(CodeblockHeader, {
    language,
    onCopy: () => {
      console.info('Copying code block to clipboard');
    },
    onWrap: () => {
      console.info('Toggling code block wrapping');
      node.classList.toggle('wrapped');
    },
  });

  const container = document.createElement('div');
  node.insertBefore(container, node.firstChild);
  app.mount(container);
}

useMutationObserver(
  component,
  (mutations) => {
    document.querySelectorAll('.codeblock').forEach((node) => {
      addHeaderIfMissing(node as HTMLElement);
    });

    if (mutations.length === 0) {
      console.info('No mutations detected');
    }
  },
  { childList: true, subtree: true },
);
</script>

<template>
  <div ref="component" v-html="html" class="prose"></div>
</template>
