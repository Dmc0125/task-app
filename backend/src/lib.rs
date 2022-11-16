use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn establish_db_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(&env_err_msg("Could not find DATABASE_URL"));
    let db = Database::connect(database_url)
        .await
        .unwrap_or_else(|_| panic!("[DATABASE]: Could not connect to database"));

    db
}

pub fn env_err_msg(msg: &str) -> String {
    format!("[ENV]: {}", msg)
}
