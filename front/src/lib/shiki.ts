import rehypeShikiFromHighlighter from '@shikijs/rehype/core';
import rehypeStringify from 'rehype-stringify';
import remarkBreaks from 'remark-breaks';
import remarkGfm from 'remark-gfm';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import { createHighlighterCore, type HighlighterCore, type ShikiTransformerContext } from 'shiki/core';
import { createOnigurumaEngine } from 'shiki/engine/oniguruma';
import { unified } from 'unified';
import type { Element } from 'hast';

let highlighterPromise: Promise<HighlighterCore> | null = null;
let highlighterSingleton: HighlighterCore | null = null;

export async function createHighlighter(): Promise<HighlighterCore> {
  if (highlighterSingleton != null) {
    return Promise.resolve(highlighterSingleton);
  }

  if (highlighterPromise != null) return highlighterPromise;

  highlighterPromise = createHighlighterCore({
    themes: [import('@shikijs/themes/min-light'), import('@shikijs/themes/min-dark')],
    langs: [
      import('@shikijs/langs/angular-html'),
      import('@shikijs/langs/angular-ts'),
      import('@shikijs/langs/astro'),
      import('@shikijs/langs/bash'),
      import('@shikijs/langs/blade'),
      import('@shikijs/langs/c'),
      import('@shikijs/langs/cpp'),
      import('@shikijs/langs/coffee'),
      import('@shikijs/langs/coffeescript'),
      import('@shikijs/langs/cpp'),
      import('@shikijs/langs/css'),
      import('@shikijs/langs/glsl'),
      import('@shikijs/langs/gql'),
      import('@shikijs/langs/graphql'),
      import('@shikijs/langs/haml'),
      import('@shikijs/langs/handlebars'),
      import('@shikijs/langs/hbs'),
      import('@shikijs/langs/html'),
      import('@shikijs/langs/html-derivative'),
      import('@shikijs/langs/http'),
      import('@shikijs/langs/imba'),
      import('@shikijs/langs/jade'),
      import('@shikijs/langs/java'),
      import('@shikijs/langs/javascript'),
      import('@shikijs/langs/jinja'),
      import('@shikijs/langs/jison'),
      import('@shikijs/langs/jl'),
      import('@shikijs/langs/js'),
      import('@shikijs/langs/json'),
      import('@shikijs/langs/json5'),
      import('@shikijs/langs/jsonc'),
      import('@shikijs/langs/jsonl'),
      import('@shikijs/langs/jsx'),
      import('@shikijs/langs/julia'),
      import('@shikijs/langs/less'),
      import('@shikijs/langs/lit'),
      import('@shikijs/langs/markdown'),
      import('@shikijs/langs/marko'),
      import('@shikijs/langs/md'),
      import('@shikijs/langs/mdc'),
      import('@shikijs/langs/mdx'),
      import('@shikijs/langs/php'),
      import('@shikijs/langs/postcss'),
      import('@shikijs/langs/pug'),
      import('@shikijs/langs/py'),
      import('@shikijs/langs/python'),
      import('@shikijs/langs/r'),
      import('@shikijs/langs/regex'),
      import('@shikijs/langs/regexp'),
      import('@shikijs/langs/sass'),
      import('@shikijs/langs/scss'),
      import('@shikijs/langs/sh'),
      import('@shikijs/langs/shell'),
      import('@shikijs/langs/shellscript'),
      import('@shikijs/langs/sql'),
      import('@shikijs/langs/styl'),
      import('@shikijs/langs/stylus'),
      import('@shikijs/langs/svelte'),
      import('@shikijs/langs/ts'),
      import('@shikijs/langs/ts-tags'),
      import('@shikijs/langs/tsx'),
      import('@shikijs/langs/typescript'),
      import('@shikijs/langs/vue'),
      import('@shikijs/langs/vue-html'),
      import('@shikijs/langs/wasm'),
      import('@shikijs/langs/wgsl'),
      import('@shikijs/langs/wit'),
      import('@shikijs/langs/xml'),
      import('@shikijs/langs/yaml'),
      import('@shikijs/langs/yml'),
      import('@shikijs/langs/zsh'),
    ],
    engine: createOnigurumaEngine(() => import('shiki/wasm')),
  });

  console.time('init shiki highlighter');

  const highlighter = await highlighterPromise;

  console.timeEnd('init shiki highlighter');

  highlighterSingleton = highlighter;
  highlighterPromise = null;

  return Promise.resolve(highlighterSingleton);
}

function transform(this: ShikiTransformerContext, hast: Element): void | Element {
  return {
    type: 'element',
    tagName: 'div',
    properties: {
      className: ['codeblock'],
      'data-language': this.options.lang || 'plaintext',
    },
    children: [hast],
  };
}

async function createProcessor() {
  const highlighter = await createHighlighter();

  const processor = unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkBreaks)
    .use(remarkRehype)
    .use(rehypeShikiFromHighlighter, highlighter as any, {
      inline: 'tailing-curly-colon',
      themes: {
        light: 'min-light',
        dark: 'min-dark',
      },
      transformers: [
        {
          pre: transform,
        },
      ],
    })
    .use(rehypeStringify, {});

  // warm up shiki
  processor.process('```js\nconsole.log("Hello, world!");\n```');

  return processor;
}

type MarkdownProcessor = Awaited<ReturnType<typeof createProcessor>>;

let markdownProcessorPromise: Promise<MarkdownProcessor> | null = null;
let markdownProcessorSingleton: MarkdownProcessor | null = null;

export async function getMarkdownProcessor(): Promise<MarkdownProcessor> {
  if (markdownProcessorSingleton != null) {
    return Promise.resolve(markdownProcessorSingleton);
  }

  if (markdownProcessorPromise != null) return markdownProcessorPromise;

  markdownProcessorPromise = createProcessor();

  const processor = await markdownProcessorPromise;

  markdownProcessorSingleton = processor;
  markdownProcessorPromise = null;

  return Promise.resolve(markdownProcessorSingleton);
}
