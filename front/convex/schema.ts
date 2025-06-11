import { defineSchema, defineTable } from 'convex/server';
import { v } from 'convex/values';

export default defineSchema({
  messages: defineTable({
    attachmentIds: v.array(v.any()),
    attachments: v.array(v.any()),
    created_at: v.float64(),
    messageId: v.string(),
    model: v.string(),
    modelParams: v.optional(
      v.object({
        includeSearch: v.boolean(),
        reasoningEffort: v.string(),
      }),
    ),
    parts: v.array(v.object({ text: v.string(), type: v.string() })),
    providerMetadata: v.optional(
      v.object({
        google: v.object({
          groundingMetadata: v.null(),
          safetyRatings: v.null(),
        }),
      }),
    ),
    resumableStreamId: v.optional(v.string()),
    role: v.string(),
    status: v.string(),
    threadId: v.string(),
    timeToFirstToken: v.optional(v.float64()),
    tokens: v.optional(v.float64()),
    tokensPerSecond: v.optional(v.float64()),
    updated_at: v.float64(),
    userId: v.string(),
  })
    .index('by_thread', ['threadId'])
    .index('by_user', ['userId']),
  settings: defineTable({
    codeFont: v.string(),
    disableHorizontalLines: v.boolean(),
    favoriteModels: v.array(v.any()),
    hasMigrated: v.boolean(),
    latestTOSDate: v.float64(),
    mainFont: v.string(),
    statsForNerds: v.boolean(),
    streamerMode: v.boolean(),
    theme: v.string(),
    userId: v.string(),
  }).index('by_user', ['userId']),
  threads: defineTable({
    branchParent: v.null(),
    createdAt: v.float64(),
    generationStatus: v.string(),
    lastMessageAt: v.float64(),
    model: v.string(),
    pinned: v.boolean(),
    threadId: v.string(),
    title: v.string(),
    updatedAt: v.float64(),
    userId: v.string(),
    userSetTitle: v.boolean(),
    visibility: v.string(),
  }).index('by_user', ['userId']),
});
