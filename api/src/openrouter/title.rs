use std::sync::Arc;

use tokio::sync::Mutex;

use crate::openrouter::OpenrouterClient;
use crate::openrouter::completions::get_completions;
use crate::openrouter::types::{Message, Role};
use crate::prelude::*;

const TITLE_GENERATION_SYSTEM_MESSAGE: &str = "You are a helpful assistant that generates concise, descriptive titles based on the user's messages. Create a short, one-line title (maximum 50 characters) that summarizes the main topic or question. Respond with only the title, without any additional text or formatting. Do not include quotes, backticks, or any other characters around the title. The title should be clear and relevant to the content provided.";
const TITLE_GENERATION_MODEL: &str = "anthropic/claude-3-haiku";

pub async fn generate_title_from_content(
  openrouter: Arc<Mutex<OpenrouterClient>>,
  thread_id: String,
  thread_messages: Vec<Message>,
  custom_key: Option<String>,
) -> anyhow::Result<String> {
  debug!("generating title for thread: {}", thread_id);

  let system_message = Message {
    role: Role::System,
    content: TITLE_GENERATION_SYSTEM_MESSAGE.to_string(),
  };

  let mut messages = vec![system_message];
  messages.extend(thread_messages);

  let completions = get_completions(openrouter, TITLE_GENERATION_MODEL, messages, custom_key, Some(50)).await?;

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
