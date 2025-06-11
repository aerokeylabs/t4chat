import { query } from './_generated/server';

export const getThreads = query({
  handler: async (ctx) => {
    const threads = await ctx.db.query('threads').order('desc').take(100);
    return { threads };
  },
});
