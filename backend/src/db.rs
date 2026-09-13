use libsql::{Builder, Connection};
use std::env;

pub async fn get_db_conn() -> Connection {
    let url = env::var("TURSO_URL").expect("TURSO_URL must be set");
    let token = env::var("TURSO_TOKEN").expect("TURSO_TOKEN must be set");
    
    let db = Builder::new_remote(url, token).build().await.unwrap();
    db.connect().unwrap()
}
