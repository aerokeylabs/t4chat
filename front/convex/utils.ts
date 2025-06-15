import { GenericQueryCtx } from 'convex/server';
import { ConvexError, v } from 'convex/values';

export async function getIdentity(ctx: GenericQueryCtx<any>) {
  const identity = await ctx.auth.getUserIdentity();
  if (identity == null) throw new Error('unauthorized');
  return identity;
}

export function validateKey(key: string) {
  if (key !== process.env.API_KEY) {
    throw new ConvexError('invalid API key');
  }
}

export const modelParamsValidator = v.object({
  includeSearch: v.boolean(),
  reasoningEffort: v.string(),
});

export const messagePartValidator = v.object({
  type: v.union(v.literal('text')),
  text: v.string(),
});
