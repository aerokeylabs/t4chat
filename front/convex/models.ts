import { ConvexError, v } from 'convex/values';
import type { ModelResponse } from '../src/lib/types/generated';
import { internal } from './_generated/api';
import { internalAction, internalMutation } from './_generated/server';
import { getApiKey, getApiUrl } from './utils';

export const update = internalMutation({
  args: {
    models: v.array(
      v.object({
        id: v.string(),
        slug: v.string(),
        name: v.string(),
        description: v.string(),
        image: v.boolean(),
        reasoning: v.boolean(),
      }),
    ),
  },
  async handler(ctx, { models }) {
    for (const { id, slug, name, description, image, reasoning } of models) {
      const model = await ctx.db
        .query('models')
        .withIndex('by_slug', (q) => q.eq('slug', slug))
        .first();

      if (model != null) {
        await ctx.db.patch(model._id, {
          name,
          description,
          image,
          reasoning,
        });
      } else {
        await ctx.db.insert('models', {
          id,
          slug,
          name,
          description,
          image,
          reasoning,
          speed: 0,
        });
      }
    }
  },
});

// remove models that are not in the slugs array
export const removeOld = internalMutation({
  args: v.object({
    slugs: v.array(v.string()),
  }),
  async handler(ctx, { slugs }) {
    const allModels = await ctx.db.query('models').collect();

    const modelsToRemove = allModels.filter((model) => !slugs.includes(model.slug));

    for (const model of modelsToRemove) {
      await ctx.db.delete(model._id);
    }
  },
});

export const updateModels = internalAction(async (ctx) => {
  let apiKey = getApiKey();
  let apiUrl = getApiUrl();

  let url = new URL('/models', apiUrl);

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${apiKey}`,
    },
  });

  if (!response.ok) throw new ConvexError(`Failed to fetch models: ${response.status} ${response.statusText}`);

  const models = (await response.json()) as ModelResponse[];

  console.info(`Fetched ${models.length} models from API.`);

  if (models.length === 0) {
    console.warn('No models found, skipping removal of old models.');
    return;
  }

  await ctx.runMutation(internal.models.update, { models });

  const slugs = models.map((model) => model.slug);
  await ctx.runMutation(internal.models.removeOld, { slugs });
});
