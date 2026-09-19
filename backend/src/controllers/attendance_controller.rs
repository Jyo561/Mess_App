use poem_openapi::{payload::Json, ApiResponse, OpenApi};
//use poem_openapi_derive::OpenApi;
use crate::security::JwtAuth;
use crate::services::{meal_service, attendance_service::{AttendanceError, AttendanceService}};
use common::{MealType, AttendanceEntry};
use std::sync::Arc;

#[derive(ApiResponse)]
pub enum EntryResponse {
    #[oai(status = 200)]
    Success(Json<String>),
    #[oai(status = 400)]
    BadRequest(Json<String>),
    #[oai(status = 403)]
    Forbidden(Json<String>),
    #[oai(status = 500)]
    InternalError(Json<String>),
}

#[derive(ApiResponse)]
pub enum MealsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MealType>>),
}

#[derive(ApiResponse)]
pub enum HistoryResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<AttendanceEntry>>),
    #[oai(status = 500)]
    InternalError(Json<String>),
}

pub struct AttendanceController {
    service: Arc<AttendanceService>,
}

impl AttendanceController {
    pub fn new(service: Arc<AttendanceService>) -> Self {
        Self { service }
    }
}

#[OpenApi]
impl AttendanceController {
    /// Get available meals for today (authenticated)
    #[oai(path = "/meals", method = "get")]
    async fn get_meals(&self, _auth: JwtAuth) -> MealsResponse {
        let meals = self.service.get_available_meals_for_today();
        MealsResponse::Ok(Json(meals))
    }

    /// Submit attendance response for a meal
    #[oai(path = "/entry", method = "post")]
    async fn submit_entry(&self, auth: JwtAuth, body: Json<AttendanceEntry>) -> EntryResponse {
        let requester_id = &auth.0.sub;
        let is_admin = auth.0.is_admin;

        match self.service.submit_entry(requester_id, is_admin, body.0).await {
            Ok(_) => EntryResponse::Success(Json("Attendance recorded successfully".into())),
            Err(AttendanceError::RuleViolation(msg)) => EntryResponse::BadRequest(Json(msg)),
            Err(AttendanceError::Forbidden(msg)) => EntryResponse::Forbidden(Json(msg)),
            Err(_) => EntryResponse::InternalError(Json("Could not save attendance".into())),
        }
    }

    /// Get recent attendance history for the logged-in member
    #[oai(path = "/history", method = "get")]
    async fn get_history(&self, auth: JwtAuth) -> HistoryResponse {
        let requester_id = &auth.0.sub;

        match self.service.get_history(requester_id).await {
            Ok(history) => HistoryResponse::Ok(Json(history)),
            Err(_) => HistoryResponse::InternalError(Json("Failed to retrieve history".into())),
        }
    }
}
