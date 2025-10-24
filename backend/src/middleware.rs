use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::ServiceExt;

use crate::{auth::verify_jwt, config::Config, database::Database};

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for certain paths
    let path = request.uri().path();
    if path.starts_with("/health") || 
       path.starts_with("/api/auth") ||
       path.starts_with("/api/creators") ||
       path.starts_with("/api/campaigns") ||
       path.starts_with("/api/events") ||
       path.starts_with("/api/posts") ||
       path.starts_with("/api/products") ||
       path.starts_with("/api/articles") ||
       path.starts_with("/api/podcasts") ||
       (path.starts_with("/api/") && request.method() == "OPTIONS") {
        return Ok(next.run(request).await);
    }

    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // Load config to get JWT secret
    let config = Config::from_env().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Verify JWT token
    let claims = verify_jwt(token, &config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add user ID to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub mod auth {
    use axum::{
        extract::{FromRequestParts, State},
        http::{request::Parts, StatusCode},
    };

    use crate::{auth::Claims, database::Database};

    #[axum::async_trait]
    impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
    {
        type Rejection = StatusCode;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
            parts
                .extensions
                .get::<Claims>()
                .cloned()
                .ok_or(StatusCode::UNAUTHORIZED)
        }
    }
}
