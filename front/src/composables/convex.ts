import { ConvexClient } from 'convex/browser';
import type { FunctionArgs, FunctionReference, FunctionReturnType } from 'convex/server';
import { ref, watch, type Ref } from 'vue';

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
type QueryArgs<Q extends Query> = FunctionArgs<Q>;
type QueryResult<Q extends Query> = FunctionReturnType<Q>;

export function useQuery<Q extends Query>(query: Q, args?: QueryArgs<Q>) {
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

export function useReactiveQuery<Q extends Query>(query: Q, maybeArgs?: Ref<QueryArgs<Q>>) {
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

  let unsubscribe: () => void;

  function subscribe(args?: QueryArgs<Q>) {
    if (unsubscribe) unsubscribe();
    unsubscribe = client.onUpdate(query, args, onResult, onError);
  }

  if (maybeArgs) {
    watch(maybeArgs, subscribe);
  }

  subscribe(maybeArgs?.value);

  return { data, error };
}

type Mutation = FunctionReference<'mutation'>;
type MutationArgs<M extends Mutation> = FunctionArgs<M>;
export type MutationResult<M extends Mutation> = FunctionReturnType<M>;

export function useMutation<M extends Mutation>(mutation: M) {
  const client = useConvex();

  return async function execute(args: MutationArgs<M>): Promise<MutationResult<M>> {
    try {
      return await client.mutation(mutation, args);
    } catch (err) {
      console.error('Mutation failed:', err);
      throw err;
    }
  };
}
