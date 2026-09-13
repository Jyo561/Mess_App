use common::{AttendanceEntry, MealType};
use chrono::NaiveDate;
use libsql::{Connection, Error as LibSqlError};
use std::sync::Arc;

pub struct AttendanceRepository {
    db: Arc<Connection>,
}

impl AttendanceRepository {
    pub fn new(db: Arc<Connection>) -> Self {
        Self { db }
    }

    /// Inserts or updates an attendance entry for a user on a specific meal & date
    pub async fn save_entry(&self, entry: &AttendanceEntry) -> Result<(), LibSqlError> {
        let meal_str = match entry.meal {
            MealType::Breakfast => "Breakfast",
            MealType::Lunch => "Lunch",
            MealType::Dinner => "Dinner",
        };

        let date_str = entry.date.format("%Y-%m-%d").to_string();

        let mut stmt = self
            .db
            .prepare(
                "INSERT INTO entries (user_id, meal_type, date, response, remarks)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
            )
            .await?;

        stmt.execute([
            entry.user_id.as_str(),
            meal_str,
            date_str.as_str(),
            &if entry.response { "1" } else { "0" },
            entry.remarks.as_str(),
        ])
        .await?;

        Ok(())
    }

    /// Fetches the attendance history for a specific user (ordered newest first)
    pub async fn get_user_history(&self, user_id: &str, limit: u32) -> Result<Vec<AttendanceEntry>, LibSqlError> {
        let mut stmt = self
            .db
            .prepare(
                "SELECT id, user_id, meal_type, date, response, remarks 
                 FROM entries 
                 WHERE user_id = ?1 
                 ORDER BY date DESC, id DESC 
                 LIMIT ?2",
            )
            .await?;

        let mut rows = stmt.query([user_id, &limit.to_string()]).await?;
        let mut entries = Vec::new();

        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let user_id: String = row.get(1)?;
            let meal_str: String = row.get(2)?;
            let date_raw: String = row.get(3)?;
            let response_int: i32 = row.get(4)?;
            let remarks: String = row.get(5).unwrap_or_default();

            let meal = match meal_str.as_str() {
                "Breakfast" => MealType::Breakfast,
                "Lunch" => MealType::Lunch,
                _ => MealType::Dinner,
            };

            let date = NaiveDate::parse_from_str(&date_raw, "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Utc::now().date_naive());

            entries.push(AttendanceEntry {
                id: Some(id),
                user_id,
                meal,
                date,
                response: response_int == 1,
                remarks,
            });
        }

        Ok(entries)
    }
}
