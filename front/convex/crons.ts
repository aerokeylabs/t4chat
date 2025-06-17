import { cronJobs } from 'convex/server';
import { internal } from './_generated/api';

const crons = cronJobs();

crons.daily('update models from api', { hourUTC: 0, minuteUTC: 0 }, internal.models.updateModelsFromApi);

export default crons;
