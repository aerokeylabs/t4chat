import { useLocalStorage } from '@vueuse/core';

const wrap = useLocalStorage('codeblock-wrap', false);

export function useCodeblockWrap() {
  return wrap;
}
