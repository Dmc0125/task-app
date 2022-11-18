use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub mod entities;

pub async fn establish_db_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = get_env_var("DATABASE_URL");
    Database::connect(database_url).await
}

pub fn get_env_var<S: Into<String>>(key: S) -> String {
    let k: String = key.into();
    env::var(&k).expect(&format!("[ENV]: Could not find {}", &k))
}
