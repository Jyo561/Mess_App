use chrono::{Datelike, Utc, Weekday};
use common::{AttendanceEntry, MealType};
use std::sync::Arc;
use crate::repositories::attendance_repository::AttendanceRepository;

#[derive(Debug)]
pub enum AttendanceError {
    Forbidden(String),
    RuleViolation(String),
    Internal(String),
}

pub struct AttendanceService {
    repo: Arc<AttendanceRepository>,
}

impl AttendanceService {
    pub fn new(repo: Arc<AttendanceRepository>) -> Self {
        Self { repo }
    }

    /// Determines valid meals for today based on the weekend rule
    pub fn get_available_meals_for_today(&self) -> Vec<MealType> {
        let today = Utc::now().date_naive();
        let weekday = today.weekday();

        // Lunch is only available on Saturday and Sunday
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            vec![MealType::Breakfast, MealType::Lunch, MealType::Dinner]
        } else {
            vec![MealType::Breakfast, MealType::Dinner]
        }
    }

    /// Validates rules and writes attendance entry to DB
    pub async fn submit_entry(&self, requester_id: &str, is_admin: bool, mut entry: AttendanceEntry) -> Result<(), AttendanceError> {
        // 1. Authorization: Non-admins can only submit for their own IC
        if !is_admin && entry.user_id != requester_id {
            return Err(AttendanceError::Forbidden("Cannot submit for another member".into()));
        }

        // Force entry.user_id to match the verified token if not admin
        if !is_admin {
            entry.user_id = requester_id.to_string();
        }

        // 2. Business Rule: Lunch is only allowed on Weekends
        if entry.meal == MealType::Lunch {
            let weekday = entry.date.weekday();
            if weekday != Weekday::Sat && weekday != Weekday::Sun {
                return Err(AttendanceError::RuleViolation(
                    "Lunch is only served on weekends (Saturday & Sunday)".into(),
                ));
            }
        }

        // 3. Save to database
        self.repo.save_entry(&entry).await.map_err(|e| {
            tracing::error!("Database failed to save attendance: {:?}", e);
            AttendanceError::Internal("Failed to record entry".into())
        })?;

        Ok(())
    }

    /// Retrieve history for the logged-in member
    pub async fn get_history(&self, user_id: &str) -> Result<Vec<AttendanceEntry>, AttendanceError> {
        self.repo.get_user_history(user_id, 30).await.map_err(|e| {
            tracing::error!("Failed to fetch history for {}: {:?}", user_id, e);
            AttendanceError::Internal("Database read error".into())
        })
    }
}
