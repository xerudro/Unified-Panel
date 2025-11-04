use crate::{
    config::Config,
    database::DbPool,
    models::{auth::*, user::{User, UserRole}},
    utils::{errors::AppError, jwt, password},
};
use chrono::Utc;
use uuid::Uuid;

pub async fn login(
    db: &DbPool,
    config: &Config,
    payload: LoginRequest,
) -> Result<(String, User), AppError> {
    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    if !password::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Check MFA if enabled
    if user.mfa_enabled {
        if let Some(totp_code) = payload.totp_code {
            let secret = user.mfa_secret.as_ref()
                .ok_or(AppError::Unauthorized("MFA secret not found".to_string()))?;

            if !verify_totp(&totp_code, secret)? {
                return Err(AppError::Unauthorized("Invalid TOTP code".to_string()));
            }
        } else {
            return Err(AppError::Unauthorized("TOTP code required".to_string()));
        }
    }

    // Log login attempt
    sqlx::query(
        "INSERT INTO login_attempts (email, ip_address, success, attempted_at)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(&payload.email)
    .bind("0.0.0.0") // TODO: Get real IP from request
    .bind(true)
    .bind(Utc::now())
    .execute(db)
    .await?;

    // Generate JWT token
    let token = jwt::generate_token(&user, config)?;

    Ok((token, user))
}

pub async fn register(
    db: &DbPool,
    payload: RegisterRequest,
) -> Result<User, AppError> {
    // Check if user already exists
    let existing = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(db)
    .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = password::hash_password(&payload.password)?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, role, company, created_at, updated_at, mfa_enabled)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(UserRole::User.as_str())
    .bind(&payload.company)
    .bind(Utc::now())
    .bind(Utc::now())
    .bind(false)
    .fetch_one(db)
    .await?;

    Ok(user)
}

fn verify_totp(code: &str, secret: &str) -> Result<bool, AppError> {
    use totp_rs::{TOTP, Algorithm, Secret};

    let secret = Secret::Encoded(secret.to_string()).to_bytes()
        .map_err(|_| AppError::InternalError("Invalid TOTP secret".to_string()))?;

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
    ).map_err(|_| AppError::InternalError("Failed to create TOTP".to_string()))?;

    Ok(totp.check_current(code)
        .map_err(|_| AppError::Unauthorized("Invalid TOTP code".to_string()))?)
}
