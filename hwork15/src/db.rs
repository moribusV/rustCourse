use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::SqlitePool;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        let db = Self { pool };
        db.create_tables().await?;
        Ok(db)
    }

    async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_message_by_username(&self, username: &str, content: &str) -> Result<()> {
        // let user_id = self.get_user_id(username).await?;
        self.save_message(username, content).await
    }

    async fn save_message(&self, username: &str, content: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO messages (username, content)
            VALUES (?, ?)
            "#,
        )
        .bind(username)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<()> {
        let password_hash = hash_password(password)?;
        sqlx::query(
            r#"
            INSERT INTO users (username, password_hash) VALUES (?, ?)
            "#,
        )
        .bind(username)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn authenticate_user(&self, username: &str, password: &str) -> Result<i64> {
        let stored_hash: String = sqlx::query_scalar(
            r#"
            SELECT password_hash FROM users WHERE username = ?
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        if verify_password(password, &stored_hash)? {
            let user_id: i64 = sqlx::query_scalar(
                r#"
                SELECT id FROM users WHERE username = ?
                "#,
            )
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
            Ok(user_id)
        } else {
            Err(sqlx::Error::RowNotFound.into())
        }
    }
}

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
