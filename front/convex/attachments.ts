import { v } from 'convex/values';
import { mutation, query } from './_generated/server';
import { getIdentity } from './utils';

export const generateUploadUrl = mutation({
  args: {
    name: v.string(),
    mimeType: v.string(),
    size: v.number(),
  },
  handler: async (ctx, { name, mimeType, size }) => {
    await getIdentity(ctx);

    const id = await ctx.db.insert('attachments', {
      name,
      mimeType,
      size,
    });

    const uploadUrl = await ctx.storage.generateUploadUrl();

    return {
      id,
      uploadUrl,
    };
  },
});

export const completeUpload = mutation({
  args: {
    id: v.id('attachments'),
    storageId: v.id('_storage'),
  },
  handler: async (ctx, { id, storageId }) => {
    await getIdentity(ctx);

    const attachment = await ctx.db.get(id);
    if (attachment == null) return null;

    await ctx.db.patch(attachment._id, {
      storageId,
    });

    return attachment;
  },
});

export const getUrl = query({
  args: {
    id: v.id('attachments'),
  },
  handler: async (ctx, { id }) => {
    const attachment = await ctx.db.get(id);
    if (attachment?.storageId == null) return null;

    const url = (await ctx.storage.getUrl(attachment.storageId)) ?? null;

    return { url, ...attachment };
  },
});

// api only route
export const apiGetById = query({
  args: {
    apiKey: v.string(),
    id: v.id('attachments'),
  },
  handler: async (ctx, { apiKey, id }) => {
    validateKey(apiKey);
    const attachment = await ctx.db.get(id);
    if (attachment?.storageId == null) return null;

    const url = (await ctx.storage.getUrl(attachment.storageId)) ?? null;

    return { url, ...attachment };
  },
});
