use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::Row;

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Campaign {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub goal_amount: f64,
    pub current_amount: Option<f64>,
    pub status: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CampaignQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub fn campaign_routes() -> Router<Database> {
    Router::new()
        .route("/", get(get_campaigns))
        .route("/", post(create_campaign))
        .route("/:slug", get(get_campaign_by_slug))
}

async fn get_campaigns(
    State(db): State<Database>,
    Query(params): Query<CampaignQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(12);
    let offset = (page - 1) * limit;

    // Use simple SQL query with snake_case table and column names
    let query = "SELECT id, title, description, goal_amount, current_amount, status, slug, created_at, updated_at FROM campaigns ORDER BY created_at DESC LIMIT $1 OFFSET $2";
    
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

async fn create_campaign(
    State(db): State<Database>,
    claims: crate::auth::Claims,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Extract values from payload
    let title = payload.get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("New Campaign");
    
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("Campaign description");
    
    let goal_amount = payload.get("goal_amount")
        .and_then(|v| v.as_f64())
        .unwrap_or(1000.0);
    
    // Generate a unique slug from title
    let slug = title
        .to_lowercase()
        .replace(" ", "-")
        .replace("'", "")
        .replace("\"", "")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    
    // Store campaign in database
    let campaign_id = uuid::Uuid::new_v4();
    let result = sqlx::query(
        "INSERT INTO campaigns (id, title, description, goal_amount, slug, status, creator_id, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())"
    )
    .bind(campaign_id)
    .bind(title)
    .bind(description)
    .bind(goal_amount)
    .bind(&slug)
    .bind("DRAFT")
    .bind(&claims.sub)
    .execute(&db.pool)
    .await;
    
    match result {
        Ok(_) => {
            let response = serde_json::json!({
                "success": true,
                "data": {
                    "id": campaign_id,
                    "slug": slug,
                    "title": title,
                    "description": description,
                    "goal_amount": goal_amount,
                    "current_amount": 0.0,
                    "status": "DRAFT"
                }
            });
            Ok(Json(response))
        }
        Err(e) => {
            eprintln!("Error creating campaign: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_campaign_by_slug(
    State(db): State<Database>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Query campaign from database by slug
    let campaign = sqlx::query(
        "SELECT id, title, description, goal_amount, current_amount, status, slug, created_at, updated_at 
         FROM campaigns WHERE slug = $1"
    )
    .bind(&slug)
    .fetch_one(&db.pool)
    .await;
    
    match campaign {
        Ok(row) => {
            let id: Uuid = row.get("id");
            let title: String = row.get("title");
            let description: String = row.get("description");
            let goal_amount: f64 = row.get("goal_amount");
            let current_amount: Option<f64> = row.get("current_amount");
            let status: String = row.get("status");
            let slug: String = row.get("slug");
            let created_at: DateTime<Utc> = row.get("created_at");
            
            let response = serde_json::json!({
                "success": true,
                "data": {
                    "id": id,
                    "slug": slug,
                    "title": title,
                    "description": description,
                    "goal_amount": goal_amount,
                    "current_amount": current_amount.unwrap_or(0.0),
                    "status": status,
                    "created_at": created_at
                }
            });
            Ok(Json(response))
        }
        Err(e) => {
            eprintln!("Error fetching campaign: {:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
