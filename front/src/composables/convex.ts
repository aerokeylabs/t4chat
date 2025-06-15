import { ConvexClient } from 'convex/browser';
import type { FunctionArgs, FunctionReference, FunctionReturnType } from 'convex/server';
import { inject, ref, watch, type Ref } from 'vue';

export function useConvex(): ConvexClient {
  const client = inject<ConvexClient>('convexClient');
  if (client == null) throw new Error('failed to inject convex');
  return client;
}

export function useConvexAuthenticated() {
  const isAuthenticated = inject<Ref<boolean>>('isConvexAuthenticated');
  if (isAuthenticated == null) throw new Error('failed to inject convex');
  return isAuthenticated;
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
    try {
      unsubscribe = client.onUpdate(query, args, onResult, onError);
    } catch (err) {
      console.error('Subscription failed:', err);
      error.value = err as Error;
      data.value = null;
    }
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
