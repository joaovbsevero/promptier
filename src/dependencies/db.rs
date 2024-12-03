use mongodb::{Client, Database};

use crate::config::Config;

pub(crate) async fn setup(config: &Config) -> Database {
    let client = Client::with_uri_str(&config.db_uri)
        .await
        .expect(&format!("Invalid connection URI: {}", config.db_uri));
    client.database(&config.db_name)
}
