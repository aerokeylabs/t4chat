import { ConvexError, v } from 'convex/values';
import { mutation, query } from './_generated/server';
import { getIdentity, validateKey } from './utils';

export const getById = query({
  args: { id: v.id('messages') },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);

    const message = await ctx.db.get(args.id);
    if (message?.userId !== identity.tokenIdentifier) return null;

    return message;
  },
});

// api only route
export const apiGetById = query({
  args: { apiKey: v.string(), id: v.id('messages') },
  handler: async (ctx, args) => {
    validateKey(args.apiKey);

    const message = await ctx.db.get(args.id);

    return message == null ? null : message;
  },
});

export const getByThreadId = query({
  args: { threadId: v.id('threads') },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);

    const thread = await ctx.db.get(args.threadId);
    if (thread?.userId !== identity.tokenIdentifier) return null;

    const messages = await ctx.db
      .query('messages')
      .withIndex('by_thread', (q) => q.eq('threadId', thread._id))
      .order('asc')
      .collect();

    return { messages };
  },
});

// api only route
export const apiGetByThreadId = query({
  args: { apiKey: v.string(), threadId: v.id('threads') },
  handler: async (ctx, args) => {
    validateKey(args.apiKey);

    const thread = await ctx.db.get(args.threadId);
    if (thread == null) return null;

    const messages = await ctx.db
      .query('messages')
      .withIndex('by_thread', (q) => q.eq('threadId', thread._id))
      .order('asc')
      .collect();

    return { messages };
  },
});

function createTextPart(text: string) {
  return { type: 'text', text } as const;
}

// api only route
export const apiAppendText = mutation({
  args: { apiKey: v.string(), messageId: v.id('messages'), text: v.string() },
  handler: async (ctx, args) => {
    validateKey(args.apiKey);

    const message = await ctx.db.get(args.messageId);
    if (message == null) return null;

    const parts = message.parts;

    // if no parts, add a text part with args.text
    // if last part is text, append args.text to it
    // if last part is not text, add a new text part with args.text
    const lastPart = parts.length === 0 ? null : parts[parts.length - 1];

    if (lastPart == null) {
      parts.push(createTextPart(args.text));
    } else if (lastPart.type === 'text') {
      lastPart.text += args.text;
    } else {
      parts.push(createTextPart(args.text));
    }

    await ctx.db.patch(message._id, {
      parts,
    });

    return { _id: message._id };
  },
});

// api only route
export const apiAppendReasoning = mutation({
  args: {
    apiKey: v.string(),
    messageId: v.id('messages'),
    reasoning: v.string(),
  },
  handler: async (ctx, args) => {
    validateKey(args.apiKey);

    const message = await ctx.db.get(args.messageId);
    if (message == null) return null;

    if (message.role !== 'assistant') {
      throw new ConvexError('Only assistant messages can have reasoning');
    }

    const reasoning = (message.reasoning ?? '') + args.reasoning;

    await ctx.db.patch(message._id, {
      reasoning,
    });

    return { _id: message._id };
  },
});

// api only route
export const apiAppendAnnotations = mutation({
  args: {
    apiKey: v.string(),
    messageId: v.id('messages'),
    annotations: v.array(
      v.object({
        title: v.string(),
        url: v.string(),
        content: v.string(),
      }),
    ),
  },
  handler: async (ctx, args) => {
    validateKey(args.apiKey);

    const message = await ctx.db.get(args.messageId);
    if (message == null) return null;

    if (message.role !== 'assistant') {
      throw new ConvexError('Only assistant messages can have annotations');
    }

    const existing = message.annotations ?? [];
    const newAnnotations = args.annotations.filter(
      (a) =>
        !existing.some(
          (existingAnnotation) =>
            existingAnnotation.title === a.title &&
            existingAnnotation.url === a.url &&
            existingAnnotation.content === a.content,
        ),
    );

    const annotations = [...existing, ...newAnnotations];

    await ctx.db.patch(message._id, {
      annotations,
    });

    return { _id: message._id };
  },
});

// api only route
export const apiComplete = mutation({
  args: {
    apiKey: v.string(),
    messageId: v.id('messages'),
    model: v.string(),
    modelParams: v.optional(
      v.object({
        reasoningEffort: v.string(),
        includeSearch: v.boolean(),
      }),
    ),
    promptTokenCount: v.number(),
    tokenCount: v.number(),
    durationMs: v.number(),
    tokensPerSecond: v.number(),
    timeToFirstTokenMs: v.number(),
  },
  handler: async (
    ctx,
    {
      apiKey,
      messageId,
      model,
      modelParams,
      promptTokenCount,
      tokenCount,
      durationMs,
      tokensPerSecond,
      timeToFirstTokenMs,
    },
  ) => {
    validateKey(apiKey);

    const message = await ctx.db.get(messageId);
    if (message == null) return null;

    if (message.role === 'assistant') {
      await ctx.db.patch(message._id, {
        status: 'complete',
        model,
        modelParams: modelParams != null ? { ...modelParams } : undefined,
        promptTokenCount,
        tokenCount,
        durationMs,
        tokensPerSecond,
        timeToFirstTokenMs,
      });
    } else {
      throw new ConvexError('Only assistant messages can be completed');
    }

    return { _id: message._id };
  },
});

// api only route
export const apiCancel = mutation({
  args: {
    apiKey: v.string(),
    messageId: v.id('messages'),
  },
  handler: async (ctx, args) => {
    validateKey(args.apiKey);

    const message = await ctx.db.get(args.messageId);
    if (message == null) return null;

    if (message.role === 'assistant') {
      await ctx.db.patch(message._id, {
        status: 'cancelled',
      });
    } else {
      throw new ConvexError('Only assistant messages can be cancelled');
    }

    return { _id: message._id };
  },
});
