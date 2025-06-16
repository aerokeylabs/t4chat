import { cronJobs } from 'convex/server';
import { internal } from './_generated/api';

const crons = cronJobs();

crons.daily('updateModels', { hourUTC: 0, minuteUTC: 0 }, internal.models.updateModels);

export default crons;
