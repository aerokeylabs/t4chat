import { v } from 'convex/values';
import { mutation, query } from './_generated/server';

export const getThreads = query({
  handler: async (ctx) => {
    const threads = await ctx.db.query('threads').order('desc').take(100);
    return { threads };
  },
});

export const createThread = mutation({
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
      title: '',
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
