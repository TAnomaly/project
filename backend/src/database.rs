use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::time::Duration;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        // Create tables if they don't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                github_id BIGINT UNIQUE,
                username VARCHAR(255) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE,
                display_name VARCHAR(255),
                avatar_url TEXT,
                bio TEXT,
                is_creator BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                title VARCHAR(255) NOT NULL,
                content TEXT,
                media_url TEXT,
                media_type VARCHAR(50),
                is_premium BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS products (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                name VARCHAR(255) NOT NULL,
                description TEXT,
                price DOUBLE PRECISION NOT NULL,
                currency VARCHAR(3) DEFAULT 'USD',
                image_url TEXT,
                is_digital BOOLEAN DEFAULT FALSE,
                download_url TEXT,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS subscriptions (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                creator_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                stripe_subscription_id VARCHAR(255),
                status VARCHAR(50) NOT NULL,
                current_period_start TIMESTAMP,
                current_period_end TIMESTAMP,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS purchases (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
                stripe_payment_intent_id VARCHAR(255),
                amount DECIMAL(10,2) NOT NULL,
                currency VARCHAR(3) DEFAULT 'USD',
                status VARCHAR(50) NOT NULL,
                created_at TIMESTAMP DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_user_id ON products(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_subscriptions_creator_id ON subscriptions(creator_id)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            pool: self.pool.clone(),
        }
    }
}

impl Database {
    pub async fn get_campaigns(&self, limit: i64, offset: i64) -> Result<Vec<crate::routes::campaigns::Campaign>, sqlx::Error> {
        sqlx::query_as::<_, crate::routes::campaigns::Campaign>(
            "SELECT id, title, description, \"goalAmount\", \"currentAmount\", status, \"createdAt\", \"updatedAt\", \"creatorId\" FROM \"Campaign\" ORDER BY \"createdAt\" DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_events(&self, limit: i64, offset: i64, upcoming: bool) -> Result<Vec<crate::routes::events::Event>, sqlx::Error> {
        let query = if upcoming {
            "SELECT id, title, description, status, \"startTime\" as start_time, \"endTime\" as end_time, location, \"createdAt\" as created_at, \"updatedAt\" as updated_at, \"hostId\" as host_id FROM \"Event\" WHERE \"startTime\" > NOW() ORDER BY \"startTime\" ASC LIMIT $1 OFFSET $2"
        } else {
            "SELECT id, title, description, status, \"startTime\" as start_time, \"endTime\" as end_time, location, \"createdAt\" as created_at, \"updatedAt\" as updated_at, \"hostId\" as host_id FROM \"Event\" ORDER BY \"startTime\" DESC LIMIT $1 OFFSET $2"
        };
        
        sqlx::query_as::<_, crate::routes::events::Event>(query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
    }
}
