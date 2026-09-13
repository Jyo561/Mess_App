use libsql::{Connection, Error as LibSqlError};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UserRecord {
    pub id: String,            // IC No as String
    pub ic_no: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub designation: String,
    pub ci: String,
    pub is_admin: bool,
}

pub struct UserRepository {
    db: Arc<Connection>,
}

impl UserRepository {
    pub fn new(db: Arc<Connection>) -> Self {
        Self { db }
    }

    pub async fn create(&self, user: &UserRecord) -> Result<(), LibSqlError> {
        let mut stmt = self
            .db
            .prepare(
                "INSERT INTO users (id, ic_no, name, email, password_hash, designation, ci, is_admin)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            )
            .await?;

        stmt.execute([
            user.id.as_str(),
            &user.ic_no.to_string(),
            user.name.as_str(),
            user.email.as_str(),
            user.password_hash.as_str(),
            user.designation.as_str(),
            user.ci.as_str(),
            &if user.is_admin { "1" } else { "0" },
        ])
        .await?;

        Ok(())
    }

    pub async fn find_by_ic(&self, ic_input: &str) -> Result<Option<UserRecord>, LibSqlError> {
        let trimmed = ic_input.trim();

        // Query checks either the text ID or matching numeric IC No
        let mut stmt = self
            .db
            .prepare(
                "SELECT id, ic_no, name, email, password_hash, designation, ci, is_admin 
                 FROM users 
                 WHERE id = ?1 OR CAST(ic_no AS TEXT) = ?1",
            )
            .await?;

        let mut rows = stmt.query([trimmed]).await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(UserRecord {
                id: row.get(0)?,
                ic_no: row.get(1)?,
                name: row.get(2)?,
                email: row.get(3)?,
                password_hash: row.get(4)?,
                designation: row.get(5)?,
                ci: row.get(6)?,
                is_admin: row.get(7)?,
            }))
        } else {
            Ok(None)
        }
    }
}
