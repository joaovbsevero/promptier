use poem_openapi::Object;

#[derive(Object)]
pub(crate) struct Error {
    pub(crate) message: String,
    pub(crate) r#type: String,

    #[oai(default, skip_serializing_if_is_none)]
    pub(crate) location: Option<Vec<String>>,
}
