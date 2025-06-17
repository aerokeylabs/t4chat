<script setup lang="ts">
import CodeblockHeader from '@/components/CodeblockHeader.vue';
import { useCodeblockWrap } from '@/composables/codeblockWrap';
import { getMarkdownProcessor } from '@/lib/shiki';
import { useMutationObserver } from '@vueuse/core';
import { createApp, ref, useTemplateRef, watch } from 'vue';

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

type CodeblockHeaderInstance = InstanceType<typeof CodeblockHeader>;

const codeblockWrap = useCodeblockWrap();
const headerInstances = ref<CodeblockHeaderInstance[]>([]);

watch(
  codeblockWrap,
  (newValue) => {
    headerInstances.value.forEach((instance) => {
      instance.setWrap(newValue);
    });
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
  });

  const container = document.createElement('div');
  node.insertBefore(container, node.firstChild);
  const instance = app.mount(container) as CodeblockHeaderInstance;
  instance.setWrap(codeblockWrap.value);
  headerInstances.value.push(instance);
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
