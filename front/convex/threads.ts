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
    message: v.string(),
    model: v.string(),
    modelParams: modelParamsValidator,
    files: v.optional(
      v.array(
        v.object({
          name: v.string(),
          size: v.number(),
          type: v.string(),
          data: v.string(), // base64 encoded data
        }),
      ),
    ),
  },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);

    const userId = identity.tokenIdentifier;

    const { message, model, modelParams, files } = args;

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
      branchParent: null,
      userId,
    });

    // Prepare message parts
    const parts: any[] = [];

    // Add text message if not empty
    if (message.trim() !== '') {
      parts.push({ text: message, type: 'text' });
    }

    // Add file attachments if any
    if (files && files.length > 0) {
      for (const file of files) {
        // Use type assertion to assure TypeScript that this matches the messagePartValidator union type
        const filePart = {
          type: 'file' as const,
          data: file.data,
          filename: file.name,
          mimeType: file.type,
          size: file.size,
        };
        parts.push(filePart as any); // Type assertion to avoid TypeScript error
      }
    }

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
    messageParts: v.array(messagePartValidator),
    model: v.string(),
    modelParams: modelParamsValidator,
    files: v.optional(
      v.array(
        v.object({
          name: v.string(),
          size: v.number(),
          type: v.string(),
          data: v.string(), // base64 encoded data
        }),
      ),
    ),
  },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);

    const thread = await ctx.db.get(args.threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    const threadId = thread._id;

    const userId = (await ctx.auth.getUserIdentity())?.tokenIdentifier ?? 'null';

    // Prepare message parts
    let messageParts = [...args.messageParts];

    // Add file attachments if any
    if (args.files && args.files.length > 0) {
      for (const file of args.files) {
        // Use type assertion to assure TypeScript that this matches the messagePartValidator union type
        const filePart = {
          type: 'file' as const,
          data: file.data,
          filename: file.name,
          mimeType: file.type,
          size: file.size,
        };
        messageParts.push(filePart as any); // Type assertion to avoid TypeScript error
      }
    }

    await ctx.db.insert('messages', {
      parts: messageParts,
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
