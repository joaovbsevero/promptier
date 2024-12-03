mod config;
mod controllers;
mod dependencies;
mod domain;
mod middlewares;
mod routes;
mod utils;

use poem::{handler, listener::TcpListener, middleware::AddData, EndpointExt, Route};
use poem_openapi::OpenApiService;

use config::Config;
use routes::PromptsAPI;
use tokio::{fs::File, io::AsyncReadExt};

#[handler]
async fn favicon_handler() -> Vec<u8> {
    let favicon = File::open("resources/icon/books.avif").await;
    match favicon {
        Ok(mut file) => {
            let mut buffer = vec![];
            match file.read_to_end(&mut buffer).await {
                Ok(_) => buffer,
                Err(_) => vec![],
            }
        }
        Err(_) => return vec![],
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load .env vars
    let config = Config::build();

    // Init dependencies
    dependencies::log::setup(&config);
    let db = dependencies::db::setup(&config).await;

    // Setup service
    let api_service = OpenApiService::new(PromptsAPI, "Prompt Library 📚", config.version);
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/favicon.ico", favicon_handler)
        .nest("/", api_service)
        .nest("/docs", ui)
        .around(middlewares::log)
        .with(AddData::new(db));

    // Start server
    let address = format!("0.0.0.0:{}", config.port);
    poem::Server::new(TcpListener::bind(address)).run(app).await
}
