use common::{LoginRequest, LoginResponse, SignupRequest};
use poem_openapi::{payload::Json, ApiResponse, OpenApi};
use std::sync::Arc;
use crate::services::auth_service::{AuthError, AuthService};

#[derive(ApiResponse)]
pub enum LoginResult {
    #[oai(status = 200)]
    Ok(Json<LoginResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<String>),
    #[oai(status = 500)]
    InternalError(Json<String>),
}

#[derive(ApiResponse)]
pub enum SignupResult {
    #[oai(status = 201)]
    Created(Json<LoginResponse>),
    #[oai(status = 400)]
    BadRequest(Json<String>),
    #[oai(status = 409)]
    Conflict(Json<String>),
    #[oai(status = 500)]
    InternalError(Json<String>),
}

pub struct AuthController {
    auth_service: Arc<AuthService>,
}

impl AuthController {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }
}

#[OpenApi]
impl AuthController {
    #[oai(path = "/auth/login", method = "post")]
    async fn login(&self, body: Json<LoginRequest>) -> LoginResult {
        match self.auth_service.login(body.0).await {
            Ok(response) => LoginResult::Ok(Json(response)),
            Err(AuthError::InvalidCredentials) => {
                LoginResult::Unauthorized(Json("Invalid IC number or password.".into()))
            }
            Err(_) => {
                LoginResult::InternalError(Json("An error occurred. Please try again later.".into()))
            }
        }
    }

    #[oai(path = "/auth/signup", method = "post")]
    async fn signup(&self, body: Json<SignupRequest>) -> SignupResult {
        match self.auth_service.signup(body.0).await {
            Ok(response) => SignupResult::Created(Json(response)),
            Err(AuthError::ValidationError(msg)) => SignupResult::BadRequest(Json(msg)),
            Err(AuthError::UserAlreadyExists) => {
                SignupResult::Conflict(Json("An account with this IC Number already exists.".into()))
            }
            Err(_) => {
                SignupResult::InternalError(Json("Registration failed. Please try again later.".into()))
            }
        }
    }
}
