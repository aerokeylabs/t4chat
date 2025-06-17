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

export const getSettings = query({
  handler: async (ctx) => {
    const identity = await getIdentity(ctx);
    const userId = identity.subject;

    const existingSettings = await ctx.db
      .query('settings')
      .withIndex('by_user', (q) => q.eq('userId', userId))
      .first();

    if (!existingSettings) {
      return {
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
    }

    return {
      userName: existingSettings.userName || '',
      userOccupation: existingSettings.userOccupation || '',
      userTraits: existingSettings.userTraits || [],
      hidePersonalInfo: existingSettings.hidePersonalInfo || false,
      mainFont: existingSettings.mainFont || 'Inter',
      codeFont: existingSettings.codeFont || 'Fira Code',
      disableHorizontalLines: existingSettings.disableHorizontalLines,
      favoriteModels: existingSettings.favoriteModels,
      hasMigrated: existingSettings.hasMigrated,
      latestTOSDate: existingSettings.latestTOSDate,
      statsForNerds: existingSettings.statsForNerds,
      streamerMode: existingSettings.streamerMode,
      theme: existingSettings.theme,
    };
  },
});

export const updateSettings = mutation({
  args: { settings: customizationSettingsValidator },
  handler: async (ctx, args) => {
    const identity = await getIdentity(ctx);
    const userId = identity.subject;

    const existingSettings = await ctx.db
      .query('settings')
      .withIndex('by_user', (q) => q.eq('userId', userId))
      .first();

    if (existingSettings) {
      if (args.settings.hidePersonalInfo !== undefined && 
          args.settings.hidePersonalInfo !== existingSettings.hidePersonalInfo) {
        console.log('Convex: Updating hidePersonalInfo from', existingSettings.hidePersonalInfo, 'to', args.settings.hidePersonalInfo);
      }
      
      return ctx.db.patch(existingSettings._id, {
        ...(args.settings.userName !== undefined && { userName: args.settings.userName }),
        ...(args.settings.userOccupation !== undefined && { userOccupation: args.settings.userOccupation }),
        ...(args.settings.userTraits !== undefined && { userTraits: args.settings.userTraits }),
        ...(args.settings.hidePersonalInfo !== undefined && { hidePersonalInfo: args.settings.hidePersonalInfo }),
        ...(args.settings.mainFont !== undefined && { mainFont: args.settings.mainFont }),
        ...(args.settings.codeFont !== undefined && { codeFont: args.settings.codeFont }),
      });
    } else {
      return ctx.db.insert('settings', {
        userId,
        userName: args.settings.userName || '',
        userOccupation: args.settings.userOccupation || '',
        userTraits: args.settings.userTraits || [],
        hidePersonalInfo: args.settings.hidePersonalInfo || false,
        mainFont: args.settings.mainFont || 'Inter',
        codeFont: args.settings.codeFont || 'Fira Code',
        disableHorizontalLines: false,
        favoriteModels: [],
        hasMigrated: false,
        latestTOSDate: 0,
        statsForNerds: false,
        streamerMode: false,
        theme: 'system',
      });
    }
  },
});
