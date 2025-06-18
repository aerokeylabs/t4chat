import { ConvexError, v } from 'convex/values';
import type { ModelResponse } from '../src/lib/types/generated';
import { internal } from './_generated/api';
import { internalAction, internalMutation, query } from './_generated/server';
import { getApiKey, getApiUrl } from './utils';

export const getBySlug = query({
  args: {
    slug: v.string(),
  },
  async handler(ctx, { slug }) {
    const model = await ctx.db
      .query('models')
      .withIndex('by_slug', (q) => q.eq('slug', slug))
      .first();

    return model ?? null;
  },
});

export const getByOpenrouterId = query({
  args: {
    id: v.string(),
  },
  async handler(ctx, { id }) {
    // remove :online from end of id if it exists
    const trimmedId = id.endsWith(':online') ? id.slice(0, -7) : id;

    const model = await ctx.db
      .query('models')
      .withIndex('by_openrouter_id', (q) => q.eq('id', trimmedId))
      .first();
    return model ?? null;
  },
});

export const search = query({
  args: {
    query: v.string(),
  },
  async handler(ctx, { query }) {
    const models = await ctx.db
      .query('models')
      .withSearchIndex('by_name', (q) => q.search('name', query))
      .collect();

    return { models };
  },
});

export const getFeatured = query({
  async handler(ctx) {
    const models = await ctx.db
      .query('models')
      .withIndex('by_featured', (q) => q.eq('featured', true))
      .collect();

    return { models };
  },
});

// regex to match the first sentence in first capture group
const sentenceRegex = /^.*?[.!?](?:\s|$)/g;
const markdownLinkRegex = /\[(.*?)\]\(.*?\)/g;

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

      let trimmedDescription: string;

      // trim description to first sentence
      if (description.length > 0) {
        const match = description.match(sentenceRegex);
        if (match && match[0]) {
          trimmedDescription = match[0];
        } else {
          trimmedDescription = description;
        }
      }

      // replace [text](url) with just text
      trimmedDescription = trimmedDescription.replace(markdownLinkRegex, '$1').trim();

      if (model != null) {
        await ctx.db.patch(model._id, {
          name,
          description: trimmedDescription,
          image,
          reasoning,
        });
      } else {
        await ctx.db.insert('models', {
          id,
          slug,
          name,
          description: trimmedDescription,
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

export const updateModelsFromApi = internalAction(async (ctx) => {
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
