<script setup lang="ts">
import { getMarkdownProcessor } from '@/lib/shiki';
import { useMutationObserver } from '@vueuse/core';
import { createApp, ref, useTemplateRef, watch } from 'vue';
import CodeblockHeader from './CodeblockHeader.vue';
import { useCodeblockWrap } from '@/composables/codeblockWrap';

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

const codeblockWrap = useCodeblockWrap();
const wrapChangeWatchers: ((value: boolean) => void)[] = [];

watch(
  codeblockWrap,
  (newValue) => {
    wrapChangeWatchers.forEach((watcher) => watcher(newValue));
  },
  { immediate: true },
);

const component = useTemplateRef('component');

function addHeaderIfMissing(node: HTMLElement) {
  if (node.classList.contains('patched')) return;
  node.classList.add('patched');

  const language = node.getAttribute('data-language') || 'plaintext';

  if (node.querySelector('.codeblock-header')) return;

  let watcher: (value: boolean) => void;

  const app = createApp(CodeblockHeader, {
    parent: node,
    language,
    onCopy() {
      console.info('Copying code block content');
      const code = node.querySelector('code');
      if (code) {
        navigator.clipboard.writeText(code.textContent || '');
      } else {
        console.warn('No code element found to copy');
      }
    },
    onWrap(newValue: boolean) {
      codeblockWrap.value = newValue;
    },
    onUnmounted() {
      wrapChangeWatchers.splice(wrapChangeWatchers.indexOf(watcher), 1);
    },
  });

  const container = document.createElement('div');
  node.insertBefore(container, node.firstChild);
  const instance = app.mount(container) as InstanceType<typeof CodeblockHeader>;
  instance.setWrap(codeblockWrap.value);

  watcher = (value: boolean) => {
    instance.setWrap(value);
  };

  wrapChangeWatchers.push(watcher);
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
