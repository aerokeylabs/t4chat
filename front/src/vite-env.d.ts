/// <reference types="vite/client" />

import type { Processor } from 'unified';

declare global {
  interface Window {
    // biome-ignore lint/suspicious/noExplicitAny: just gotta any here
    markdownProcessor: Processor<any, any, any, any, any>;
  }
}
