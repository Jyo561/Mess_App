use libsql::{Builder, Database, Error as LibSqlError};
use std::env;

pub async fn get_db() -> Result<Database, LibSqlError> {
    let url = env::var("TURSO_URL").expect("TURSO_URL must be set");
    let token = env::var("TURSO_TOKEN").expect("TURSO_TOKEN must be set");

    Builder::new_remote(url, token).build().await
}
