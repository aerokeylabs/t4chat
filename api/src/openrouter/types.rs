use serde::Deserialize;

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
