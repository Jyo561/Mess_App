use argon2::{
    password_hash::{phc::PasswordHash, PasswordHasher, PasswordVerifier},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use common::{Claims, LoginRequest, LoginResponse, SignupRequest};
use std::sync::Arc;
use crate::repositories::user_repository::{UserRepository, UserRecord};

// Pre-computed valid Argon2id hash to normalize response times when a user doesn't exist
const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$eHh4eHh4eHh4eHh4eHh4eA$Y2Fubm90YmVndWVzc2VkeHh4eHh4eHh4eHh4eHh4eHg";

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    ValidationError(String),
    Internal(String),
}

pub struct AuthService {
    user_repo: Arc<UserRepository>,
    jwt_secret: Vec<u8>,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        let secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET environment variable must be set")
            .into_bytes();

        Self {
            user_repo,
            jwt_secret: secret,
        }
    }

    /// Authenticates credentials and issues a JWT token
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AuthError> {
        // 1. Fetch user from repository
        let user_record = self.user_repo.find_by_ic(&req.ic_no).await.map_err(|e| {
            tracing::error!("Failed to fetch user {}: {:?}", req.ic_no, e);
            AuthError::Internal("Database query failed".into())
        })?;

        // 2. Perform constant-time verification against timing attacks
        let hash_to_check = user_record
            .as_ref()
            .map(|u| u.password_hash.as_str())
            .unwrap_or(DUMMY_HASH);

        let parsed_hash = PasswordHash::new(hash_to_check);
        let password_valid = match parsed_hash {
            Ok(h) => Argon2::default().verify_password(req.password.as_bytes(), &h).is_ok(),
            Err(_) => false,
        };

        // If user was not found or password didn't match, return generic failure
        if user_record.is_none() || !password_valid {
            return Err(AuthError::InvalidCredentials);
        }

        let user = user_record.unwrap();

        // 3. Issue Token
        let token = self.issue_token(&user)?;

        Ok(LoginResponse {
            token,
            user_id: user.id,
            name: user.name,
            designation: user.designation,
            ci: user.ci,
            is_admin: user.is_admin,
        })
    }

    /// Issue a signed JWT token
    fn issue_token(&self, user: &UserRecord) -> Result<String, AuthError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(12))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.clone(),
            name: user.name.clone(),
            designation: user.designation.clone(),
            ci: user.ci.clone(),
            is_admin: user.is_admin,
            exp: expiration,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.jwt_secret),
        )
        .map_err(|e| {
            tracing::error!("JWT encoding error: {:?}", e);
            AuthError::Internal("Token generation failed".into())
        })
    }

    /// Verifies incoming Bearer token
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        decode::<Claims>(token, &DecodingKey::from_secret(&self.jwt_secret), &validation)
            .map(|data| data.claims)
    }

    /// Helper for admin/seed password hashing
    pub fn hash_password(&self, raw_password: &str) -> Result<String, String> {
        Argon2::default()
            .hash_password(raw_password.as_bytes())
            .map(|h| h.to_string())
            .map_err(|e| format!("Argon2 hash failed: {}", e))
    }

    pub async fn signup(&self, req: SignupRequest) -> Result<LoginResponse, AuthError> {
        let ic_str = req.ic_no.to_string();

        // 1. Basic validation
        if req.name.trim().is_empty() {
            return Err(AuthError::ValidationError("Name cannot be empty".into()));
        }
        if req.password.len() < 6 {
            return Err(AuthError::ValidationError("Password must be at least 6 characters".into()));
        }
        if req.ic_no <= 0 {
            return Err(AuthError::ValidationError("Invalid IC Number".into()));
        }

        // 2. Check if IC No already exists
        let existing = self.user_repo.find_by_ic(&ic_str).await.map_err(|e| {
            tracing::error!("Error checking existing user: {:?}", e);
            AuthError::Internal("Database error".into())
        })?;

        if existing.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        // 3. Hash password using Argon2
        let password_hash = self.hash_password(&req.password).map_err(|e| {
            tracing::error!("Failed to hash password during signup: {:?}", e);
            AuthError::Internal("Encryption error".into())
        })?;

        // 4. Build record (is_admin defaults to false for public signups)
        let new_user = UserRecord {
            id: ic_str.clone(),
            ic_no: req.ic_no,
            name: req.name.trim().to_string(),
            email: req.email.trim().to_string(),
            password_hash,
            designation: req.designation.trim().to_string(),
            ci: req.ci.trim().to_string(),
            is_admin: false,
        };

        self.user_repo.create(&new_user).await.map_err(|e| {
            tracing::error!("Failed to save new user {}: {:?}", ic_str, e);
            AuthError::Internal("Database insert failed".into())
        })?;

        // 5. Issue session JWT directly
        let token = self.issue_token(&new_user)?;

        Ok(LoginResponse {
            token,
            user_id: new_user.id,
            name: new_user.name,
            designation: new_user.designation,
            ci: new_user.ci,
            is_admin: new_user.is_admin,
        })
    }
}
