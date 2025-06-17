import { defineSchema, defineTable } from 'convex/server';
import { v } from 'convex/values';

export default defineSchema({
  messages: defineTable({
    role: v.string(),
    parts: v.array(
      v.union(
        v.object({ text: v.string(), type: v.literal('text') }),
        v.object({
          type: v.literal('file'),
          data: v.string(), // base64 encoded file data
          filename: v.string(),
          mimeType: v.string(),
          size: v.number(),
        }),
      ),
    ),
    attachmentIds: v.array(v.any()),
    attachments: v.array(v.any()),

    threadId: v.string(),
    userId: v.string(),

    model: v.optional(v.string()),
    modelParams: v.optional(
      v.object({
        includeSearch: v.boolean(),
        reasoningEffort: v.string(),
      }),
    ),
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
    timeToFirstToken: v.optional(v.float64()),
    tokens: v.optional(v.float64()),
    tokensPerSecond: v.optional(v.float64()),
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
    .index('by_featured', ['featured']),
});
