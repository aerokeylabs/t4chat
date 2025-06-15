use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletion {
  pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatChoice {
  pub delta: ChatDelta,
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
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum ChatDelta {
  Text { content: String },
  Finished { finish_reason: String },
  Refusal { refusal: String },
}
