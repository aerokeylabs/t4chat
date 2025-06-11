import type { Id } from '@/convex/_generated/dataModel';

export type ModelParams = {
  includeSearch: boolean;
  reasoningEffort: string;
};

export type Part = {
  text: string;
  type: 'text';
};

export type ProviderMetadata = {
  google: Google;
};

export type Google = {
  groundingMetadata: null;
  safetyRatings: null;
};

type BaseMessage = {
  _creationTime: number;
  _id: Id<'messages'>;
  attachmentIds: unknown[];
  attachments: unknown[];
  created_at: number;
  messageId: string;
  parts: Part[];
  status: string;
  threadId: string;
  updated_at: number;
  userId: string;
};

export type UserMessage = BaseMessage & {
  role: 'user';
};

export type AssistantMessage = BaseMessage & {
  role: 'assistant';
  model: string;
  modelParams: ModelParams;
  providerMetadata: ProviderMetadata;
  resumableStreamId: string;
  timeToFirstToken: number;
  tokens: number;
  tokensPerSecond: number;
};

export type Message = UserMessage | AssistantMessage;
