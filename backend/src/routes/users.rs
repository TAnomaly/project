use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, put},
    Router,
};
use uuid::Uuid;

use crate::{
    auth::Claims,
    database::Database,
    models::User,
};

pub fn user_routes() -> Router<Database> {
    Router::new()
        .route("/me", get(get_current_user))
        .route("/:id", get(get_user_by_id))
        .route("/:id", put(update_user))
}

async fn get_current_user(
    State(db): State<Database>,
    claims: Claims,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(claims.sub.parse::<Uuid>().unwrap())
    .fetch_one(&db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

async fn get_user_by_id(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&db.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

async fn update_user(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    claims: Claims,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<User>, StatusCode> {
    // Only allow users to update their own profile
    if claims.sub.parse::<Uuid>().unwrap() != id {
        return Err(StatusCode::FORBIDDEN);
    }

    let display_name = payload.get("display_name").and_then(|v| v.as_str());
    let bio = payload.get("bio").and_then(|v| v.as_str());
    let is_creator = payload.get("is_creator").and_then(|v| v.as_bool());

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET display_name = COALESCE($2, display_name),
            bio = COALESCE($3, bio),
            is_creator = COALESCE($4, is_creator),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(display_name)
    .bind(bio)
    .bind(is_creator)
    .fetch_one(&db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
