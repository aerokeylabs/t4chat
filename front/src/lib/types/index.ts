import type { ModelId } from '../models';

export * from './generated';

export type Message = {
  id: string;
  content: string;
  role: 'user' | 'assistant';
};

// model list

export type Model = {
  id: ModelId;
  hugging_face_id: null | string;
  name: string;
  created: number;
  description: string;
  context_length: number;
  architecture: Architecture;
  pricing: Pricing;
  top_provider: TopProvider;
  supported_parameters: Parameter[];
};

export type Architecture = {
  modality: ModalityString;
  input_modalities: Modality[];
  output_modalities: Modality[];
  tokenizer: Tokenizer;
  instruct_type: null | string;
};

export type Modality = 'file' | 'image' | 'text';

export type ModalityString = 'text+image->text' | 'text->text';

export type Tokenizer =
  | 'Gemini'
  | 'Other'
  | 'Qwen'
  | 'DeepSeek'
  | 'Claude'
  | 'GPT'
  | 'Llama3'
  | 'Mistral'
  | 'Qwen3'
  | 'Grok'
  | 'Llama4'
  | 'Cohere'
  | 'Nova'
  | 'Yi'
  | 'Llama2'
  | 'Router';

export type Pricing = {
  prompt: string;
  completion: string;
  request?: string;
  image?: string;
  web_search?: string;
  internal_reasoning?: string;
  input_cache_read?: string;
  input_cache_write?: string;
};

export type Parameter =
  | 'tools'
  | 'tool_choice'
  | 'max_tokens'
  | 'temperature'
  | 'top_p'
  | 'reasoning'
  | 'include_reasoning'
  | 'structured_outputs'
  | 'response_format'
  | 'stop'
  | 'frequency_penalty'
  | 'presence_penalty'
  | 'seed'
  | 'top_k'
  | 'repetition_penalty'
  | 'logit_bias'
  | 'logprobs'
  | 'top_logprobs'
  | 'min_p'
  | 'web_search_options'
  | 'top_a';

export type TopProvider = {
  context_length: number | null;
  max_completion_tokens: number | null;
  is_moderated: boolean;
};
