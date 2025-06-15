import { v } from 'convex/values';
import { mutation, query } from './_generated/server';

export const getById = query({
  args: { id: v.id('messages') },
  handler: async (ctx, args) => {
    const message = await ctx.db.get(args.id);

    return message == null ? null : message;
  },
});

export const getByThreadId = query({
  args: { threadId: v.string() },
  handler: async (ctx, args) => {
    const messages = await ctx.db
      .query('messages')
      // .withIndex('by_user', (q) => q.eq('userId', ctx.auth.userId))
      .withIndex('by_thread', (q) => q.eq('threadId', args.threadId))
      .order('asc')
      .collect();

    return { messages };
  },
});

function createTextPart(text: string) {
  return { type: 'text', text } as const;
}

export const appendText = mutation({
  args: { messageId: v.id('messages'), text: v.string() },
  handler: async (ctx, args) => {
    const message = await ctx.db.get(args.messageId);

    if (message == null) return null;

    const existingParts = message.parts;

    // if no parts, add a text part with args.text
    // if last part is text, append args.text to it
    // if last part is not text, add a new text part with args.text
    const lastPart = existingParts.length === 0 ? null : existingParts[existingParts.length - 1];

    if (lastPart == null) {
      message.parts.push(createTextPart(args.text));
      await ctx.db.patch(message._id, {
        parts: message.parts,
      });
    } else {
      const newText = lastPart.text + args.text;

      const newParts = existingParts.slice(0, -1);
      newParts.push(createTextPart(newText));

      await ctx.db.patch(message._id, {
        parts: newParts,
      });
    }

    return { _id: message._id };
  },
});

export const complete = mutation({
  args: {
    messageId: v.id('messages'),
    model: v.string(),
    modelParams: v.optional(
      v.object({
        reasoningEffort: v.string(),
        includeSearch: v.boolean(),
      }),
    ),
  },
  handler: async (ctx, args) => {
    const message = await ctx.db.get(args.messageId);

    if (message == null) return null;

    if (message.role === 'assistant') {
      await ctx.db.patch(message._id, {
        status: 'complete',
        model: args.model,
        modelParams: args.modelParams != null ? { ...args.modelParams } : undefined,
      });
    } else {
      throw new Error('Only assistant messages can be completed');
    }

    return { _id: message._id };
  },
});

export const cancel = mutation({
  args: { messageId: v.id('messages') },
  handler: async (ctx, args) => {
    const message = await ctx.db.get(args.messageId);

    if (message == null) return null;

    if (message.role === 'assistant') {
      await ctx.db.patch(message._id, {
        status: 'cancelled',
      });
    } else {
      throw new Error('Only assistant messages can be cancelled');
    }

    return { _id: message._id };
  },
});
