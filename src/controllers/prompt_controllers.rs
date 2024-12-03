use std::str::FromStr;

use mongodb::{
    bson::{oid::ObjectId, Document},
    Database,
};

use poem_openapi::payload::Json;

use crate::domain::{
    dto::{
        CreatePromptResponse, DeletePromptResponse, Error, ReadPromptResponse, ReadPromptsResponse,
    },
    entities::Prompt,
};

pub(crate) async fn create_prompt(database: &Database, mut prompt: Prompt) -> CreatePromptResponse {
    // Ensure _id is None
    prompt._id = None;

    let collection = database.collection::<Prompt>("prompts");
    let result = collection.insert_one(prompt.clone()).await;
    match result {
        Ok(r) => {
            prompt._id = Some(r.inserted_id.as_object_id().unwrap().to_string());
            CreatePromptResponse::Ok(Json(prompt))
        }
        Err(e) => match e.kind.as_ref() {
            mongodb::error::ErrorKind::Write(_) => {
                CreatePromptResponse::PromptAlreadyExists(Json(Error {
                    message: String::from("Prompt already exists"),
                    r#type: String::from("prompt"),
                    location: Option::Some(vec!["body".to_string(), "name".to_string()]),
                }))
            }
            _ => CreatePromptResponse::InternalServerError(Json(Error {
                message: e.to_string(),
                r#type: String::from("InternalServerError"),
                location: Option::None,
            })),
        },
    }
}

pub(crate) async fn read_prompts(
    database: &Database,
    name: Option<String>,
    model: Option<String>,
    temperature: Option<f64>,
) -> ReadPromptsResponse {
    let collection = database.collection::<Document>("prompts");
    let mut filter = mongodb::bson::doc! {};
    if let Some(name) = name {
        filter.insert("name", name);
    };
    if let Some(model) = model {
        filter.insert("model", model);
    }
    if let Some(temperature) = temperature {
        filter.insert("temperature", temperature);
    }
    let result = collection.find(filter).await;
    match result {
        Ok(mut cursor) => {
            let mut prompts = vec![];
            while let Ok(has_more) = cursor.advance().await {
                if has_more {
                    let mut document = cursor.deserialize_current().unwrap();

                    // Convert id to string before serializing into prompt
                    let id = document
                        .get("_id")
                        .unwrap()
                        .as_object_id()
                        .unwrap()
                        .to_string();
                    document.insert("_id", id);

                    prompts.push(mongodb::bson::from_document(document).unwrap());
                } else {
                    break;
                }
            }
            ReadPromptsResponse::Ok(Json(prompts))
        }
        Err(e) => ReadPromptsResponse::InternalServerError(Json(Error {
            message: e.to_string(),
            r#type: String::from("InternalServerError"),
            location: Option::None,
        })),
    }
}

pub(crate) async fn read_prompt(database: &Database, id: String) -> ReadPromptResponse {
    let collection = database.collection::<Document>("prompts");
    let id = match ObjectId::from_str(id.as_str()) {
        Ok(object_id) => object_id,
        Err(_) => {
            return ReadPromptResponse::InvalidObjectId(Json(Error {
                message: String::from("id must be a 12-byte input or a 24-character hex string"),
                r#type: String::from("InvalidObjectId"),
                location: Some(vec!["id".to_string()]),
            }))
        }
    };

    let result = collection.find_one(mongodb::bson::doc! {"_id": id}).await;
    match result {
        Ok(doc) => match doc {
            Some(mut doc) => {
                // Convert id to string before serializing into prompt
                let id = doc.get("_id").unwrap().as_object_id().unwrap().to_string();
                doc.insert("_id", id);

                ReadPromptResponse::Ok(Json(mongodb::bson::from_document(doc).unwrap()))
            }
            None => ReadPromptResponse::PromptNotFound(Json(Error {
                message: format!("Prompt not found with ID {}", id),
                r#type: String::from("PromptNotFound"),
                location: Option::None,
            })),
        },
        Err(e) => ReadPromptResponse::InternalServerError(Json(Error {
            message: e.to_string(),
            r#type: String::from("InternalServerError"),
            location: Option::None,
        })),
    }
}

pub(crate) async fn delete_prompt(database: &Database, id: String) -> DeletePromptResponse {
    let collection = database.collection::<Document>("prompts");
    let id = match ObjectId::from_str(id.as_str()) {
        Ok(object_id) => object_id,
        Err(_) => {
            return DeletePromptResponse::InvalidObjectId(Json(Error {
                message: String::from("id must be a 12-byte input or a 24-character hex string"),
                r#type: String::from("InvalidObjectId"),
                location: Some(vec!["id".to_string()]),
            }))
        }
    };

    let result = collection.delete_one(mongodb::bson::doc! {"_id": id}).await;
    match result {
        Ok(delete_result) => match delete_result.deleted_count {
            0 => DeletePromptResponse::PromptNotFound(Json(Error {
                message: format!("Prompt not found with ID {}", id),
                r#type: String::from("PromptNotFound"),
                location: Option::None,
            })),
            _ => DeletePromptResponse::Ok,
        },
        Err(e) => DeletePromptResponse::InternalServerError(Json(Error {
            message: e.to_string(),
            r#type: String::from("InternalServerError"),
            location: Option::None,
        })),
    }
}
