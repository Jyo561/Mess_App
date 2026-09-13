use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};
#[cfg(feature = "backend")] // Only for backend
use poem_openapi::{Object,Enum};


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "backend", derive(Enum))]
pub enum MealType { Breakfast, Lunch, Dinner }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(Object))]
pub struct AttendanceEntry {
    pub id: Option<i64>,
    pub user_id: String,
    pub meal: MealType,
    pub date: NaiveDate,
    pub response: bool, // Yes/No
    pub remarks: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub name: String,
    pub ic_no: i32,
    pub email: String,
    pub designation: String,
    pub ci: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Object))]
pub struct SignupRequest {
    pub ic_no: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub designation: String,
    pub ci: String,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Object))]
pub struct LoginRequest {
    pub ic_no: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Object))]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String, // IC No
    pub name: String,
    pub designation: String,
    pub ci: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // IC No
    pub name: String,
    pub designation: String,
    pub ci: String,
    pub is_admin: bool,
    pub exp: usize,        // UNIX timestamp in seconds
}
