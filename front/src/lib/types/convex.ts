import type { Id } from '@/convex/_generated/dataModel';

export type ModelParams = {
  includeSearch: boolean;
  reasoningEffort: string;
};

export type TextPart = {
  type: 'text';
  text: string;
};

export type AttachmentPart = {
  type: 'attachment';
  id: Id<'attachments'>;
};

export type Part = TextPart | AttachmentPart;

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
  attachmentIds: string[];
  attachments: unknown[];
  parts: Part[];
  threadId: string;
  updated_at: number;
  userId: string;
};

export type UserMessage = BaseMessage & {
  role: 'user';
};

export type Annotation = {
  title: string;
  url: string;
  content: string;
};

type BaseAssistantMessage = BaseMessage & {
  role: 'assistant';
  status: 'pending' | 'complete' | 'cancelled' | 'error';
  model: string;
  modelParams: ModelParams;
  providerMetadata: ProviderMetadata;

  reasoning?: string;
  annotations?: Annotation[];
};

type CompletedAssistantMessage = {
  status: 'complete';
  promptTokenCount: number;
  tokenCount: number;
  durationMs: number;
  tokensPerSecond: number;
  timeToFirstTokenMs: number;
};

type PendingAssistantMessage = {
  status: 'pending';
};

export type AssistantMessage = BaseAssistantMessage & (CompletedAssistantMessage | PendingAssistantMessage);

export type Message = UserMessage | AssistantMessage;

export type Model = {
  _id: Id<'models'>;
  _creationTime: number;
  featured?: boolean;
  id: string;
  slug: string;
  name: string;
  description: string;
  image: boolean;
  reasoning: boolean;
  speed: number;
};
