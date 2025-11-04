use crate::{
    database::DbPool,
    models::user::*,
    utils::{errors::AppError, password},
};
use chrono::Utc;
use uuid::Uuid;

pub async fn list_users(db: &DbPool) -> Result<Vec<User>, AppError> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(db)
    .await?;

    Ok(users)
}

pub async fn get_user(db: &DbPool, id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound("User not found".to_string()))?;

    Ok(user)
}

pub async fn create_user(db: &DbPool, payload: CreateUser) -> Result<User, AppError> {
    // Check if email already exists
    let existing = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(db)
    .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }

    // Hash password
    let password_hash = password::hash_password(&payload.password)?;

    // Create user
    let role = payload.role.unwrap_or(UserRole::User);
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, role, company, created_at, updated_at, mfa_enabled)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(role.as_str())
    .bind(&payload.company)
    .bind(Utc::now())
    .bind(Utc::now())
    .bind(false)
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn update_user(
    db: &DbPool,
    id: Uuid,
    payload: UpdateUser,
) -> Result<User, AppError> {
    // Get existing user
    let mut user = get_user(db, id).await?;

    // Update fields
    if let Some(email) = payload.email {
        user.email = email;
    }
    if let Some(company) = payload.company {
        user.company = Some(company);
    }
    if let Some(timezone) = payload.timezone {
        user.timezone = Some(timezone);
    }
    if let Some(avatar_url) = payload.avatar_url {
        user.avatar_url = Some(avatar_url);
    }

    // Save to database
    let user = sqlx::query_as::<_, User>(
        "UPDATE users
         SET email = $1, company = $2, timezone = $3, avatar_url = $4, updated_at = $5
         WHERE id = $6
         RETURNING *"
    )
    .bind(&user.email)
    .bind(&user.company)
    .bind(&user.timezone)
    .bind(&user.avatar_url)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn delete_user(db: &DbPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(())
}
