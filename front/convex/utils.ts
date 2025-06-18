import { GenericQueryCtx } from 'convex/server';
import { ConvexError } from 'convex/values';

export async function getIdentity(ctx: GenericQueryCtx<any>) {
  const identity = await ctx.auth.getUserIdentity();
  if (identity == null) throw new ConvexError('unauthorized');
  return identity;
}

export function getApiUrl() {
  const apiUrl = process.env.API_URL;
  if (!apiUrl) throw new ConvexError('API URL is not set in environment variables');
  return apiUrl;
}

export function getApiKey() {
  const apiKey = process.env.API_KEY;
  if (!apiKey) throw new ConvexError('API key is not set in environment variables');
  return apiKey;
}

export function validateKey(key: string) {
  if (key !== getApiKey()) {
    throw new ConvexError('invalid API key');
  }
}
