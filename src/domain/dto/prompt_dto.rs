use poem_openapi::{payload::Json, ApiResponse};

use super::Error;
use crate::domain::entities::Prompt;

#[derive(ApiResponse)]
pub(crate) enum CreatePromptResponse {
    #[oai(status = 201, header(name = "Content-Location", ty = "String"))]
    Ok(Json<Prompt>),

    #[oai(status = 409)]
    PromptAlreadyExists(Json<Error>),

    #[oai(status = 500)]
    InternalServerError(Json<Error>),
}

#[derive(ApiResponse)]
pub(crate) enum ReadPromptsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Prompt>>),

    #[oai(status = 500)]
    InternalServerError(Json<Error>),
}

#[derive(ApiResponse)]
pub(crate) enum ReadPromptResponse {
    #[oai(status = 200)]
    Ok(Json<Prompt>),

    #[oai(status = 404)]
    PromptNotFound(Json<Error>),

    #[oai(status = 422)]
    InvalidObjectId(Json<Error>),

    #[oai(status = 500)]
    InternalServerError(Json<Error>),
}

#[derive(ApiResponse)]
pub(crate) enum DeletePromptResponse {
    #[oai(status = 204)]
    Ok,

    #[oai(status = 404)]
    PromptNotFound(Json<Error>),

    #[oai(status = 422)]
    InvalidObjectId(Json<Error>),

    #[oai(status = 500)]
    InternalServerError(Json<Error>),
}
