use poem::Request;
use poem_openapi::auth::Bearer;
use poem_openapi::SecurityScheme;
use common::Claims;
use crate::services::auth_service::AuthService;
use std::sync::Arc;

#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    key_name = "Authorization",
    checker = "jwt_checker"
)]
pub struct JwtAuth(pub Claims);

async fn jwt_checker(req: &Request, bearer: Bearer) -> Option<Claims> {
    // Retrieve AuthService registered in Poem's app data
    let auth_service = req.data::<Arc<AuthService>>()?;
    auth_service.verify_token(&bearer.token).ok()
}
