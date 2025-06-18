import type { Id } from '@/convex/_generated/dataModel';
import { useEventBus } from '@vueuse/core';

const eventBus = useEventBus<Id<'messages'>>('retryEventBus');

export function useRetryEventBus() {
  return eventBus;
}
