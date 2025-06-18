import { defineSchema, defineTable } from 'convex/server';
import { v } from 'convex/values';

export const messagePartValidator = v.union(
  v.object({ text: v.string(), type: v.literal('text') }),
  v.object({ type: v.literal('attachment'), id: v.id('attachments') }),
);

export const modelParamsValidator = v.object({
  includeSearch: v.boolean(),
  reasoningEffort: v.string(),
});

export default defineSchema({
  messages: defineTable({
    role: v.string(),
    parts: v.array(messagePartValidator),
    attachmentIds: v.array(v.any()),
    attachments: v.array(v.any()),

    threadId: v.string(),
    userId: v.string(),

    model: v.optional(v.string()),
    modelParams: v.optional(modelParamsValidator),
    providerMetadata: v.optional(
      v.object({
        google: v.object({
          groundingMetadata: v.null(),
          safetyRatings: v.null(),
        }),
      }),
    ),
    resumableStreamId: v.optional(v.string()),
    status: v.optional(
      v.union(v.literal('pending'), v.literal('complete'), v.literal('cancelled'), v.literal('error')),
    ),

    reasoning: v.optional(v.string()),
    annotations: v.optional(
      v.array(
        v.object({
          title: v.string(),
          url: v.string(),
          content: v.string(),
        }),
      ),
    ),

    promptTokenCount: v.optional(v.number()),
    tokenCount: v.optional(v.number()),
    durationMs: v.optional(v.number()),
    tokensPerSecond: v.optional(v.number()),
    timeToFirstTokenMs: v.optional(v.number()),
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
    // Customization fields
    userName: v.optional(v.string()),
    userOccupation: v.optional(v.string()),
    userTraits: v.optional(v.array(v.string())),
    hidePersonalInfo: v.optional(v.boolean()),
  }).index('by_user', ['userId']),

  threads: defineTable({
    branchParent: v.null(),
    createdAt: v.float64(),
    generationStatus: v.string(),
    lastMessageAt: v.float64(),
    model: v.string(),
    pinned: v.boolean(),
    title: v.optional(v.string()),
    updatedAt: v.float64(),
    userId: v.string(),
    userSetTitle: v.boolean(),
    visibility: v.string(),
  })
    .searchIndex('by_title', {
      searchField: 'title',
      filterFields: ['userId', 'visibility'],
    })
    .index('by_user', ['userId', 'visibility']),

  models: defineTable({
    id: v.string(),
    slug: v.string(),
    name: v.string(),
    description: v.string(),
    image: v.boolean(),
    reasoning: v.boolean(),
    speed: v.float64(),

    featured: v.optional(v.boolean()),
  })
    .searchIndex('by_name', {
      searchField: 'name',
    })
    .index('by_slug', ['slug'])
    .index('by_openrouter_id', ['id'])
    .index('by_featured', ['featured']),

  attachments: defineTable({
    name: v.string(),
    mimeType: v.string(),
    size: v.number(),
    storageId: v.optional(v.id('_storage')),
  }),
});
