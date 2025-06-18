import { ConvexError, v } from 'convex/values';
import { mutation, query } from './_generated/server';
import { getIdentity, validateKey } from './utils';
import { messagePartValidator, modelParamsValidator } from './schema';
import { Id } from './_generated/dataModel';

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

function sortThreads(threads: { lastMessageAt: number }[]) {
  threads.sort((a, b) => b.lastMessageAt - a.lastMessageAt);
}

export const getThreads = query({
  args: {
    query: v.optional(v.string()),
  },
  handler: async (ctx, { query }) => {
    const identity = await getIdentity(ctx);

    if (query == null || query.trim() === '') {
      const threads = await ctx.db
        .query('threads')
        .withIndex('by_user', (q) => q.eq('userId', identity.tokenIdentifier).eq('visibility', 'visible'))
        .take(256);

      sortThreads(threads);

      return { threads };
    }

    const threads = await ctx.db
      .query('threads')
      .withSearchIndex('by_title', (q) =>
        q.search('title', query.trim()).eq('visibility', 'visible').eq('userId', identity.tokenIdentifier),
      )
      .take(256);

    sortThreads(threads);

    return { threads };
  },
});

export const create = mutation({
  args: {
    parts: v.array(messagePartValidator),
    model: v.string(),
    modelParams: modelParamsValidator,
  },
  handler: async (ctx, { parts, model, modelParams }) => {
    const identity = await getIdentity(ctx);
    const userId = identity.tokenIdentifier;

    const threadId = await ctx.db.insert('threads', {
      generationStatus: 'pending',
      updatedAt: Date.now(),
      createdAt: Date.now(),
      lastMessageAt: Date.now(),
      userSetTitle: false,
      title: undefined,
      visibility: 'hidden',
      pinned: false,
      model,
      branchParent: undefined,
      userId,
    });

    await ctx.db.insert('messages', {
      parts,
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
    parts: v.array(messagePartValidator),
    model: v.string(),
    modelParams: modelParamsValidator,
  },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);
    const userId = identity.tokenIdentifier;

    const { parts, model, modelParams } = args;

    const thread = await ctx.db.get(args.threadId);
    if (thread?.userId !== userId) return null;

    const threadId = thread._id;

    await ctx.db.insert('messages', {
      parts,
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

    await ctx.db.patch(threadId, {
      lastMessageAt: Date.now(),
      updatedAt: Date.now(),
      generationStatus: 'pending',
    });

    return { assistantMessageId };
  },
});

export const deleteThreadById = mutation({
  args: {
    threadId: v.id('threads'),
  },
  handler: async (ctx, { threadId }) => {
    const identity = await getIdentity(ctx);
    const thread = await ctx.db.get(threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    const messages = await ctx.db
      .query('messages')
      .withIndex('by_thread', (q) => q.eq('threadId', threadId))
      .collect();

    for (const message of messages) {
      await ctx.db.delete(message._id);
    }

    await ctx.db.delete(threadId);

    return { threadId };
  },
});

export const pinThreadById = mutation({
  args: {
    threadId: v.id('threads'),
  },
  handler: async (ctx, { threadId }) => {
    const identity = await getIdentity(ctx);
    const thread = await ctx.db.get(threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    await ctx.db.patch(threadId, {
      pinned: true,
    });

    return { threadId };
  },
});

export const unpinThreadById = mutation({
  args: {
    threadId: v.id('threads'),
  },
  handler: async (ctx, { threadId }) => {
    const identity = await getIdentity(ctx);
    const thread = await ctx.db.get(threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    await ctx.db.patch(threadId, {
      pinned: false,
    });

    return { threadId };
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
      visibility: 'visible',
    });

    return { _id: thread._id };
  },
});

export const apiGetMessagesUntil = query({
  args: {
    apiKey: v.string(),
    threadId: v.id('threads'),
    untilId: v.id('messages'),
  },
  async handler(ctx, { apiKey, threadId, untilId }) {
    validateKey(apiKey);

    const thread = await ctx.db.get(threadId);
    if (thread == null) return null;

    const until = await ctx.db.get(untilId);
    if (until?.threadId !== threadId) return null;

    const messages = await ctx.db
      .query('messages')
      .withIndex('by_thread', (q) => q.eq('threadId', threadId))
      .collect();

    messages.sort((a, b) => a._creationTime - b._creationTime);

    return messages.filter((message) => message._creationTime <= until._creationTime);
  },
});

// Get message counts for multiple threads
export const getMessageCounts = query({
  args: {
    threadIds: v.array(v.id('threads')),
  },
  handler: async (ctx, { threadIds }) => {
    const identity = await getIdentity(ctx);
    const counts: Record<string, number> = {};

    // Get all messages for the requested threads
    const allMessages = await Promise.all(
      threadIds.map(async (threadId) => {
        const thread = await ctx.db.get(threadId);
        // Only count messages for threads that belong to the current user
        if (thread?.userId === identity.tokenIdentifier) {
          const messages = await ctx.db
            .query('messages')
            .withIndex('by_thread', (q) => q.eq('threadId', threadId))
            .collect();
          return { threadId, count: messages.length };
        }
        return { threadId, count: 0 };
      }),
    );

    // Convert to a record for easier lookup
    allMessages.forEach(({ threadId, count }) => {
      counts[threadId] = count;
    });

    return counts;
  },
});

export const forkThread = mutation({
  args: {
    messageId: v.id('messages'),
  },
  async handler(ctx, { messageId }) {
    const identity = await getIdentity(ctx);
    const userId = identity.tokenIdentifier;

    const message = await ctx.db.get(messageId);
    if (message?.userId !== userId) {
      throw new ConvexError('Message not found');
    }

    const threadId = message.threadId as Id<'threads'>;
    const thread = await ctx.db.get(threadId);

    if (thread?.userId !== userId) {
      throw new ConvexError('Thread not found');
    }

    // Create a new thread with the same properties as the original
    const newThreadId = await ctx.db.insert('threads', {
      generationStatus: 'pending',
      updatedAt: Date.now(),
      createdAt: Date.now(),
      lastMessageAt: Date.now(),
      userSetTitle: thread.userSetTitle,
      title: thread.title,
      visibility: thread.visibility,
      pinned: false,
      model: thread.model,
      branchParent: thread._id,
      userId: thread.userId,
    });

    // insert messages up to the fork point
    const messages = await ctx.db
      .query('messages')
      .withIndex('by_thread', (q) => q.eq('threadId', threadId))
      .order('asc')
      .collect();
    const forkIndex = messages.findIndex((msg) => msg._id === messageId);
    const messagesToInsert = messages.slice(0, forkIndex + 1);

    for (const msg of messagesToInsert) {
      await ctx.db.insert('messages', {
        role: msg.role,
        parts: msg.parts,
        attachmentIds: msg.attachmentIds,
        attachments: msg.attachments,
        userId: msg.userId,
        model: msg.model,
        modelParams: msg.modelParams,
        providerMetadata: msg.providerMetadata,
        resumableStreamId: msg.resumableStreamId,
        status: msg.status,
        reasoning: msg.reasoning,
        annotations: msg.annotations,
        promptTokenCount: msg.promptTokenCount,
        tokenCount: msg.tokenCount,
        durationMs: msg.durationMs,
        tokensPerSecond: msg.tokensPerSecond,
        timeToFirstTokenMs: msg.timeToFirstTokenMs,
        threadId: newThreadId,
      });
    }

    return { threadId: newThreadId };
  },
});
