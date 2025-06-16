use crate::openrouter::OpenrouterError;
use crate::openrouter::types::{Modality, Model, Parameter};
use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum GetModelsError {
  #[error("Failed to get models: {0}")]
  Openrouter(#[from] OpenrouterError),
}

into_response!(GetModelsError {
  Openrouter(_) => StatusCode::INTERNAL_SERVER_ERROR,
});

#[derive(Serialize, Type)]
pub struct ModelResponse {
  id: String,
  slug: String,
  name: String,
  description: String,

  image: bool,
  reasoning: bool,
}

fn model_to_response(
  Model {
    id,
    slug,
    name,
    description,
    architecture,
    supported_parameters,
    ..
  }: Model,
) -> ModelResponse {
  let image = architecture.input_modalities.contains(&Modality::Image);
  let reasoning = supported_parameters.contains(&Parameter::Reasoning);

  ModelResponse {
    id,
    slug,
    name,
    description,
    image,
    reasoning,
  }
}

#[tracing::instrument(name = "get models", skip(state), err)]
#[axum::debug_handler]
pub async fn get_models(state: State<AppState>) -> Result<Json<Vec<ModelResponse>>, GetModelsError> {
  let models = state.openrouter.lock().await.get_models().await?;

  Ok(Json(models.data.into_iter().map(model_to_response).collect()))
}
