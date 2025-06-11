import { v } from 'convex/values';
import { query } from './_generated/server';

export const getMessagesByThread = query({
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
