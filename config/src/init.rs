use std::{env, sync::Arc};

use sea_orm::Database;

fn init_env() {
    dotenv::from_path(".env").ok();
}

pub async fn init_mysq_pool() -> sea_orm::DatabaseConnection {
    // 初始化env环境.
    init_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("database_url: {}", database_url);

    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    println!("db: {:?}", db);

    db
}