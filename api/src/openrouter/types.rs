use serde::{Deserialize, Serialize};

// region: list models

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListModelsResponse {
  pub data: Vec<Model>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Model {
  /// Unique model identifier used in API requests
  /// (e.g., `google/gemini-2.5-pro-preview`)
  pub id: String,
  /// Permanent slug for the model that never changes
  #[serde(rename = "canonical_slug")]
  pub slug: String,
  pub hugging_face_id: Option<String>,
  /// Human-readable display name for the model
  pub name: String,
  /// Unix timestamp of when the model was added to OpenRouter
  pub created: i64,
  /// Detailed description of the model’s capabilities and characteristics
  pub description: String,
  /// Maximum context window size in tokens
  pub context_length: i64,
  /// Object describing the model’s technical capabilities
  pub architecture: Architecture,
  /// Lowest price structure for using this model
  pub pricing: Pricing,
  /// Configuration details for the primary provider
  pub top_provider: TopProvider,
  /// Array of supported API parameters for this model
  pub supported_parameters: Vec<Parameter>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Architecture {
  pub modality: ModalitySummary,
  pub input_modalities: Vec<Modality>,
  pub output_modalities: Vec<Modality>,
  pub tokenizer: String,
  pub instruct_type: Option<String>,
}

#[derive(Deserialize, PartialEq, Eq)]
pub enum Modality {
  #[serde(rename = "file")]
  File,
  #[serde(rename = "image")]
  Image,
  #[serde(rename = "text")]
  Text,
}

#[derive(Deserialize, PartialEq, Eq)]
pub enum ModalitySummary {
  #[serde(rename = "text+image->text")]
  TextImageToText,
  #[serde(rename = "text->text")]
  TextToText,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pricing {
  /// Cost per input token
  pub prompt: String,
  /// Cost per output token
  pub completion: String,
  /// Fixed cost per API request
  pub request: Option<String>,
  /// Cost per image input
  pub image: Option<String>,
  /// Cost per web search operation
  pub web_search: Option<String>,
  /// Cost for internal reasoning tokens
  pub internal_reasoning: Option<String>,
  /// Cost per cached input token read
  pub input_cache_read: Option<String>,
  /// Cost per cached input token write
  pub input_cache_write: Option<String>,
}

#[derive(Deserialize, PartialEq, Eq)]
pub enum Parameter {
  /// Repetition reduction
  #[serde(rename = "frequency_penalty")]
  FrequencyPenalty,
  /// Include reasoning in response
  #[serde(rename = "include_reasoning")]
  IncludeReasoning,
  #[serde(rename = "logit_bias")]
  LogitBias,
  #[serde(rename = "logprobs")]
  Logprobs,
  /// Response length limiting
  #[serde(rename = "max_tokens")]
  MaxTokens,
  #[serde(rename = "min_p")]
  MinP,
  /// Topic diversity
  #[serde(rename = "presence_penalty")]
  PresencePenalty,
  /// Internal reasoning mode
  #[serde(rename = "reasoning")]
  Reasoning,
  #[serde(rename = "repetition_penalty")]
  RepetitionPenalty,
  /// Output format specification
  #[serde(rename = "response_format")]
  ResponseFormat,
  /// Deterministic outputs
  #[serde(rename = "seed")]
  Seed,
  /// Custom stop sequences
  #[serde(rename = "stop")]
  Stop,
  /// JSON schema enforcement
  #[serde(rename = "structured_outputs")]
  StructuredOutputs,
  /// Randomness control
  #[serde(rename = "temperature")]
  Temperature,
  /// Tool selection control
  #[serde(rename = "tool_choice")]
  ToolChoice,
  /// Function calling capabilities
  #[serde(rename = "tools")]
  Tools,
  #[serde(rename = "top_a")]
  TopA,
  #[serde(rename = "top_k")]
  TopK,
  #[serde(rename = "top_logprobs")]
  TopLogprobs,
  /// Nucleus sampling
  #[serde(rename = "top_p")]
  TopP,
  #[serde(rename = "web_search_options")]
  WebSearchOptions,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TopProvider {
  /// Provider-specific context limit
  pub context_length: Option<i64>,
  /// Maximum tokens in response
  pub max_completion_tokens: Option<i64>,
  /// Whether content moderation is applied
  pub is_moderated: bool,
}

// endregion

// region: completions

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
  #[serde(rename = "user")]
  User,
  #[serde(rename = "system")]
  System,
  #[serde(rename = "assistant")]
  Assistant,
  #[serde(rename = "function")]
  Function,
  #[serde(rename = "tool")]
  Tool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageUrl {
  pub url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct File {
  pub filename: String,
  /// base64 encoded file content
  pub file_data: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ContentPart {
  #[serde(rename = "text")]
  Text { text: String },
  #[serde(rename = "image_url")]
  Image { image_url: ImageUrl },
  #[serde(rename = "file")]
  File { file: File },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageRequest {
  pub role: Role,
  pub content: ContentPart,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageResponse {
  pub role: Role,
  pub content: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
  #[serde(rename = "low")]
  Low,
  #[serde(rename = "medium")]
  Medium,
  #[serde(rename = "high")]
  High,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ReasoningRequest {
  pub effort: ReasoningEffort,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CompletionRequest {
  pub model: String,
  pub messages: Vec<MessageRequest>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub reasoning: Option<ReasoningRequest>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_tokens: Option<u32>,

  pub stream: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompletionChoice {
  pub message: MessageResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompletionResponse {
  pub id: String,
  pub choices: Vec<CompletionChoice>,
}

// endregion

// region: streaming completions

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletion {
  pub choices: Vec<ChatChoice>,
  pub usage: Option<ChatUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatChoice {
  pub delta: ChatDelta,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum ChatDelta {
  Text {
    content: String,
    reasoning: Option<String>,
    annotations: Option<Vec<Annotation>>,
  },
  Finished {
    finish_reason: String,
  },
  Refusal {
    refusal: String,
  },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
  Stop,
  Length,
  ContentFilter,
  ToolUse,
  FunctionCall,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatUsage {
  pub prompt_tokens: u32,
  pub completion_tokens: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Annotation {
  #[serde(rename = "url_citation")]
  UrlCitation { url_citation: UrlCitation },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlCitation {
  pub title: String,
  pub url: String,
  pub content: String,
}

// endregion
