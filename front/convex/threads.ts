import { v } from 'convex/values';
import { mutation, query } from './_generated/server';

export const getById = query({
  args: { id: v.id('threads') },
  handler: async (ctx, args) => {
    const thread = await ctx.db.get(args.id);
    return thread == null ? null : thread;
  },
});

export const getThreads = query({
  handler: async (ctx) => {
    const userId = (await ctx.auth.getUserIdentity())?.tokenIdentifier ?? 'null';

    console.info('getThreads called for userId:', userId);

    const threads = await ctx.db.query('threads').order('desc').take(100);
    return { threads };
  },
});

export const create = mutation({
  args: {
    message: v.string(),
    model: v.string(),
    modelParams: v.object({
      includeSearch: v.boolean(),
      reasoningEffort: v.string(),
    }),
  },
  handler: async (ctx, args) => {
    const userId = (await ctx.auth.getUserIdentity())?.tokenIdentifier ?? 'null';

    const threadId = await ctx.db.insert('threads', {
      generationStatus: 'pending',
      updatedAt: Date.now(),
      createdAt: Date.now(),
      lastMessageAt: Date.now(),
      userSetTitle: false,
      title: undefined,
      visibility: 'visible',
      pinned: false,
      model: args.model,
      branchParent: null,
      userId,
    });

    await ctx.db.insert('messages', {
      parts: [{ text: args.message, type: 'text' }],
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

    return { threadId, assistantMessageId };
  },
});

export const createMessage = mutation({
  args: {
    threadId: v.id('threads'),
    messageParts: v.array(
      v.object({
        type: v.union(v.literal('text')),
        text: v.string(),
      }),
    ),
    model: v.string(),
    modelParams: v.object({
      includeSearch: v.boolean(),
      reasoningEffort: v.string(),
    }),
  },
  handler: async (ctx, args) => {
    const thread = await ctx.db.get(args.threadId);
    if (thread == null) return null;

    // if (thread.userId !== (await ctx.auth.getUserIdentity())?.tokenIdentifier) {
    //   throw new Error('You do not have permission to create a message in this thread');
    // }

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

    await ctx.db.patch(args.threadId, {
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
  handler: async (ctx, args) => {
    const thread = await ctx.db.get(args.threadId);

    if (thread == null) return null;

    // if (thread.userId !== (await ctx.auth.getUserIdentity())?.tokenIdentifier) {
    //   throw new Error('You do not have permission to update this thread');
    // }

    await ctx.db.patch(args.threadId, {
      title: args.title,
      userSetTitle: true,
    });

    return { _id: thread._id };
  },
});

export const apiSetTitle = mutation({
  args: {
    threadId: v.id('threads'),
    title: v.string(),
  },
  handler: async (ctx, args) => {
    const thread = await ctx.db.get(args.threadId);

    if (thread == null) return null;

    await ctx.db.patch(args.threadId, {
      title: args.title,
      userSetTitle: false,
    });

    return { _id: thread._id };
  },
});
