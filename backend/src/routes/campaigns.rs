use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Campaign {
    pub id: String,
    pub title: String,
    pub description: String,
    pub goal_amount: f64,
    pub current_amount: f64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub creator_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CampaignQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub fn campaign_routes() -> Router<Database> {
    Router::new()
        .route("/", get(get_campaigns))
}

async fn get_campaigns(
    State(db): State<Database>,
    Query(params): Query<CampaignQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(12);
    let offset = (page - 1) * limit;

    // Use simple SQL query with snake_case table and column names
    let query = "SELECT id, title, description, goal_amount, current_amount, status, created_at, updated_at, creator_id FROM campaigns ORDER BY created_at DESC LIMIT $1 OFFSET $2";
    
    match sqlx::query_as::<_, Campaign>(query)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&db.pool)
        .await
    {
        Ok(campaigns) => {
            // Frontend'in beklediği format
            let response = serde_json::json!({
                "success": true,
                "data": campaigns,
                "pagination": {
                    "page": page,
                    "limit": limit,
                    "total": campaigns.len(),
                    "pages": 1
                }
            });
            Ok(Json(response))
        },
        Err(e) => {
            tracing::error!("Failed to fetch campaigns: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
