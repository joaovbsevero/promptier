use crate::domain::dto::{
    CreatePromptResponse, DeletePromptResponse, ReadPromptResponse, ReadPromptsResponse,
};
use crate::{controllers::prompt_controllers, domain::entities::Prompt};

use mongodb::Database;
use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};

use super::APITags;

pub(crate) struct PromptsAPI;

#[OpenApi]
impl PromptsAPI {
    /// Create new prompt
    #[oai(method = "post", path = "/", tag = APITags::Prompts)]
    async fn create_prompt(
        &self,
        Data(database): Data<&Database>,
        Json(prompt): Json<Prompt>,
    ) -> CreatePromptResponse {
        prompt_controllers::create_prompt(database, prompt).await
    }

    /// Read all prompts
    ///
    /// Use the query arguments to filter the returned prompts.
    #[oai(method = "get", path = "/", tag = APITags::Prompts)]
    async fn read_prompts(
        &self,
        Data(database): Data<&Database>,
        Query(name): Query<Option<String>>,
        Query(model): Query<Option<String>>,
        Query(temperature): Query<Option<f64>>,
    ) -> ReadPromptsResponse {
        prompt_controllers::read_prompts(database, name, model, temperature).await
    }

    /// Read one prompt
    #[oai(method = "get", path = "/:id", tag = APITags::Prompts)]
    async fn read_prompt(
        &self,
        Data(database): Data<&Database>,
        Path(id): Path<String>,
    ) -> ReadPromptResponse {
        prompt_controllers::read_prompt(database, id).await
    }

    /// Delete a prompt
    #[oai(method = "delete", path = "/:id", tag = APITags::Prompts)]
    async fn delete_prompt(
        &self,
        Data(database): Data<&Database>,
        Path(id): Path<String>,
    ) -> DeletePromptResponse {
        prompt_controllers::delete_prompt(database, id).await
    }
}
