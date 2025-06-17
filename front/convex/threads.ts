import { v } from 'convex/values';
import { mutation, query } from './_generated/server';
import { getIdentity, messagePartValidator, modelParamsValidator, validateKey } from './utils';

export const getById = query({
  args: { id: v.id('threads') },
  handler: async (ctx, { id }) => {
    const identity = await getIdentity(ctx);

    const thread = await ctx.db.get(id);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    return thread;
  },
});

// api only route
export const apiGetById = query({
  args: { apiKey: v.string(), id: v.id('threads') },
  handler: async (ctx, { apiKey, id }) => {
    validateKey(apiKey);

    const thread = await ctx.db.get(id);

    return thread == null ? null : thread;
  },
});

export const getThreads = query({
  args: {
    query: v.optional(v.string()),
  },
  handler: async (ctx, { query }) => {
    const identity = await getIdentity(ctx);

    if (query == null || query.trim() === '') {
      const threads = await ctx.db
        .query('threads')
        .withIndex('by_user', (q) => q.eq('userId', identity.tokenIdentifier))
        .take(256);

      threads.sort((a, b) => b.lastMessageAt - a.lastMessageAt);

      return { threads };
    }

    const threads = await ctx.db
      .query('threads')
      .withSearchIndex('by_title', (q) =>
        q.search('title', query.trim()).eq('visibility', 'visible').eq('userId', identity.tokenIdentifier),
      )
      .take(256);

    threads.sort((a, b) => b.lastMessageAt - a.lastMessageAt);

    return { threads };
  },
});

export const create = mutation({
  args: {
    message: v.string(),
    model: v.string(),
    modelParams: modelParamsValidator,
  },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);

    const userId = identity.tokenIdentifier;

    const { message, model, modelParams } = args;

    const threadId = await ctx.db.insert('threads', {
      generationStatus: 'pending',
      updatedAt: Date.now(),
      createdAt: Date.now(),
      lastMessageAt: Date.now(),
      userSetTitle: false,
      title: undefined,
      visibility: 'visible',
      pinned: false,
      model,
      branchParent: null,
      userId,
    });

    await ctx.db.insert('messages', {
      parts: [{ text: message, type: 'text' }],
      role: 'user',
      attachmentIds: [],
      attachments: [],
      threadId,
      userId,
    });

    const assistantMessageId = await ctx.db.insert('messages', {
      parts: [],
      role: 'assistant',
      attachmentIds: [],
      attachments: [],
      threadId,
      userId,
      status: 'pending',
      model,
      modelParams,
    });

    return { threadId, assistantMessageId };
  },
});

export const createMessage = mutation({
  args: {
    threadId: v.id('threads'),
    messageParts: v.array(messagePartValidator),
    model: v.string(),
    modelParams: modelParamsValidator,
  },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);

    const thread = await ctx.db.get(args.threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    const threadId = thread._id;

    const userId = (await ctx.auth.getUserIdentity())?.tokenIdentifier ?? 'null';

    await ctx.db.insert('messages', {
      parts: args.messageParts,
      role: 'user',
      attachmentIds: [],
      attachments: [],
      threadId,
      userId,
    });

    const assistantMessageId = await ctx.db.insert('messages', {
      parts: [],
      role: 'assistant',
      attachmentIds: [],
      attachments: [],
      threadId,
      userId,
      status: 'pending',
      model: args.model,
      modelParams: args.modelParams,
    });

    await ctx.db.patch(threadId, {
      lastMessageAt: Date.now(),
      updatedAt: Date.now(),
      generationStatus: 'pending',
    });

    return { assistantMessageId };
  },
});

export const updateTitle = mutation({
  args: {
    threadId: v.id('threads'),
    title: v.string(),
  },
  handler: async (ctx, { threadId, title }) => {
    const identity = await getIdentity(ctx);
    const thread = await ctx.db.get(threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    await ctx.db.patch(thread._id, {
      title,
      userSetTitle: true,
    });

    return { _id: thread._id };
  },
});

// api only route
export const apiSetTitle = mutation({
  args: {
    apiKey: v.string(),
    threadId: v.id('threads'),
    title: v.string(),
  },
  handler: async (ctx, { apiKey, threadId, title }) => {
    validateKey(apiKey);
    const thread = await ctx.db.get(threadId);
    if (thread == null) return null;

    await ctx.db.patch(thread._id, {
      title,
      userSetTitle: false,
    });

    return { _id: thread._id };
  },
});
