import type { FunctionArgs, FunctionReference, FunctionReturnType } from 'convex/server';
import { ConvexClient } from 'convex/browser';
import { ref } from 'vue';

let convex: ConvexClient | null = null;

export function initConvex(url: string) {
  if (convex) {
    console.warn('called initConvex while already initialized');
    return;
  }

  try {
    convex = new ConvexClient(url);
  } catch (error) {
    console.error('initConvex failed:', error);
  }
}

export function useConvex() {
  if (!convex) throw new Error('called useConvex before client initialized with initConvex');

  return convex;
}

type Query = FunctionReference<'query'>;

type QueryResult<Q extends Query> = FunctionReturnType<Q>;

export function useQuery<Q extends Query>(query: Q, args?: FunctionArgs<Q>) {
  const client = useConvex();

  const data = ref<QueryResult<Q> | null>(null);
  const error = ref<Error | null>(null);

  function onResult(result: QueryResult<Q> | null) {
    data.value = result;
    error.value = null;
  }

  function onError(err: Error) {
    data.value = null;
    error.value = err;
  }

  client.onUpdate(query, args, onResult, onError);

  return { data, error };
}
