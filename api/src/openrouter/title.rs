use std::sync::Arc;

use tokio::sync::Mutex;

use crate::openrouter::OpenrouterClient;
use crate::openrouter::completions::get_completions;
use crate::openrouter::types::{ContentPart, MessageRequest, Role};
use crate::prelude::*;

const TITLE_GENERATION_SYSTEM_MESSAGE: &str = "You are an AI assistant that creates short, descriptive titles. Your only task is to generate a concise title (max 50 characters) based on the user's messages. You must always return a title, even if the conversation seems unclear. Do not add any explanation, just provide the title text. Never return an empty response.";
const TITLE_GENERATION_MODEL: &str = "anthropic/claude-3-haiku";

pub async fn generate_title_from_content(
  openrouter: Arc<Mutex<OpenrouterClient>>,
  thread_id: String,
  thread_messages: Vec<MessageRequest>,
  custom_key: Option<String>,
) -> anyhow::Result<String> {
  debug!("generating title for thread: {}", thread_id);

  let system_message = MessageRequest {
    role: Role::System,
    content: vec![ContentPart::Text {
      text: TITLE_GENERATION_SYSTEM_MESSAGE.to_string(),
    }],
  };

  let mut messages = vec![system_message];
  messages.extend(thread_messages);

  messages.push(MessageRequest {
    role: Role::User,
    content: vec![ContentPart::Text {
      text: "What is the title of this conversation?".to_string(),
    }],
  });

  let completions = get_completions(openrouter, TITLE_GENERATION_MODEL, messages, custom_key, Some(50), None).await?;

  let title = completions
    .choices
    .into_iter()
    .map(|choice| choice.message.content.to_string())
    .collect::<Vec<_>>()
    .join("");

  let final_title = title
    .trim()
    .trim_matches(|c| c == '"' || c == '\'' || c == '`')
    .to_string();

  if final_title.is_empty() {
    bail!("Generated title is empty for thread: {}", thread_id);
  }

  Ok(final_title)
}
