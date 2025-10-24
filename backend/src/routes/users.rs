use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    auth::Claims,
    database::Database,
    models::User,
};

pub fn user_routes() -> Router<Database> {
    Router::new()
        .route("/me", get(get_current_user))
        .route("/me/campaigns", get(get_user_campaigns))
        .route("/become-creator", post(become_creator))
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
    .bind(&claims.sub)
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

async fn get_user_campaigns(
    State(db): State<Database>,
    claims: Claims,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get campaigns created by the current user
    let campaigns = sqlx::query(
        "SELECT id, title, description, goal_amount, current_amount, status, slug, created_at, updated_at 
         FROM campaigns WHERE creator_id = $1 ORDER BY created_at DESC"
    )
    .bind(&claims.sub)
    .fetch_all(&db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let campaign_list: Vec<serde_json::Value> = campaigns
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<uuid::Uuid, _>("id"),
                "title": row.get::<String, _>("title"),
                "description": row.get::<String, _>("description"),
                "goal_amount": row.get::<f64, _>("goal_amount"),
                "current_amount": row.get::<Option<f64>, _>("current_amount").unwrap_or(0.0),
                "status": row.get::<String, _>("status"),
                "slug": row.get::<String, _>("slug"),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
            })
        })
        .collect();
    
    let response = serde_json::json!({
        "success": true,
        "data": campaign_list
    });
    
    Ok(Json(response))
}

async fn become_creator(
    State(db): State<Database>,
    claims: Claims,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user_id = &claims.sub;
    
    println!("🔄 User {} is trying to become a creator", user_id);
    
    // Check if user exists first
    let user_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user_id)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Error checking if user exists: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    if !user_exists {
        eprintln!("User {} not found", user_id);
        return Err(StatusCode::NOT_FOUND);
    }
    
    // Update user to be a creator
    let result = sqlx::query(
        "UPDATE users SET is_creator = true WHERE id = $1"
    )
    .bind(user_id)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Error updating user to creator: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    if result.rows_affected() == 0 {
        eprintln!("No rows affected when updating user {} to creator", user_id);
        return Err(StatusCode::NOT_FOUND);
    }
    
    println!("✅ User {} successfully became a creator", user_id);
    
    let response = serde_json::json!({
        "success": true,
        "message": "Successfully became a creator"
    });
    
    Ok(Json(response))
}