import { v } from 'convex/values';
import { mutation, query } from './_generated/server';
import { getIdentity } from './utils';

const customizationSettingsValidator = v.object({
  userName: v.optional(v.string()),
  userOccupation: v.optional(v.string()),
  userTraits: v.optional(v.array(v.string())),
  hidePersonalInfo: v.optional(v.boolean()),
  mainFont: v.optional(v.string()),
  codeFont: v.optional(v.string()),
});

const DEFAULT_SETTINGS = {
  userName: '',
  userOccupation: '',
  userTraits: [],
  hidePersonalInfo: false,
  mainFont: 'Inter',
  codeFont: 'Fira Code',
  disableHorizontalLines: false,
  favoriteModels: [],
  hasMigrated: false,
  latestTOSDate: 0,
  statsForNerds: false,
  streamerMode: false,
  theme: 'system',
};

export const getSettings = query({
  handler: async (ctx) => {
    const identity = await getIdentity(ctx);
    const userId = identity.tokenIdentifier;

    const existingSettings = await ctx.db
      .query('settings')
      .withIndex('by_user', (q) => q.eq('userId', userId))
      .first();

    if (existingSettings == null) return DEFAULT_SETTINGS;

    return {
      ...DEFAULT_SETTINGS,
      ...existingSettings,
    };
  },
});

export const updateSettings = mutation({
  args: { settings: customizationSettingsValidator },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);
    const userId = identity.tokenIdentifier;

    const existingSettings = await ctx.db
      .query('settings')
      .withIndex('by_user', (q) => q.eq('userId', userId))
      .first();

    if (existingSettings) {
      return ctx.db.patch(existingSettings._id, {
        userName: args.settings.userName,
        userOccupation: args.settings.userOccupation,
        userTraits: args.settings.userTraits,
        hidePersonalInfo: args.settings.hidePersonalInfo,
        mainFont: args.settings.mainFont,
        codeFont: args.settings.codeFont,
      });
    } else {
      return ctx.db.insert('settings', {
        userId,
        ...args.settings,
        ...DEFAULT_SETTINGS,
      });
    }
  },
});
