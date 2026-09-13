mod db;
// mod auth;
use poem::{
    listener::TcpListener,
    middleware::Cors,
    EndpointExt,
    Route,
    Server,
};
use poem_openapi::OpenApiService;
use std::sync::Arc;

mod services;
mod controllers;
mod repositories;
mod security;


use crate::controllers::{attendance_controller::AttendanceController, auth_controller::AuthController};
use crate::repositories::{user_repository::UserRepository, attendance_repository::AttendanceRepository};
use crate::services::{auth_service::AuthService, attendance_service::AttendanceService};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    dotenvy::dotenv().ok();
    // tracing_subscriber::fmt::init();

    // 1. Database
    let conn = db::get_db_conn().await;
    let shared_db = Arc::new(conn);

    // 2. Instantiate Repositories -> Services -> Controllers
    //Authentication
    let user_repo = Arc::new(UserRepository::new(shared_db.clone()));
    let auth_service = Arc::new(AuthService::new(user_repo.clone()));
    let auth_controller = AuthController::new(auth_service.clone());

    //Attendance Feature
    let attendance_repo = Arc::new(AttendanceRepository::new(shared_db.clone()));
    let attendance_service = Arc::new(AttendanceService::new(attendance_repo.clone()));
    let attendance_controller = AttendanceController::new(attendance_service.clone());

    let combined_api = (auth_controller, attendance_controller);

    let api_service =
        OpenApiService::new(combined_api, "Enclave API", "1.0")
            .server("http://localhost:3000");

    let ui = api_service.swagger_ui();

    let cors = Cors::new()
        .allow_origin("http://localhost:8080")
        .allow_methods(["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(["Content-Type", "Authorization"]);

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .with(cors);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await

}
