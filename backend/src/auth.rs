use crate::cloudinary::{CloudinaryConfig, CloudinaryService};
use crate::error::AppError;
use crate::structs::{AuthResponse, LoginRequest, NewUser, ProfilePictureUpdate, UserProfile};
use axum::{Form, Json, extract::{Path, State}};
use bcrypt::{DEFAULT_COST, hash, verify};
use http::StatusCode;
use sqlx::PgPool;
use tower_sessions::Session;

pub async fn register(
    State(pool): State<PgPool>,
    session: Session,
    Form(new_user): Form<NewUser>,
) -> Result<Json<AuthResponse>, AppError> {
    if !new_user.email.contains('@') || new_user.email.is_empty() {
        return Ok(Json(AuthResponse {
            success: false,
            message: "Invalid email format".to_string(),
            user_id: None,
        }));
    }

    if new_user.password.len() < 6 {
        return Ok(Json(AuthResponse {
            success: false,
            message: "Password must be at least 6 characters long".to_string(),
            user_id: None,
        }));
    }

    if let Some(ref name) = new_user.name {
        if name.trim().is_empty() {
            return Ok(Json(AuthResponse {
                success: false,
                message: "Name cannot be empty".to_string(),
                user_id: None,
            }));
        }
    }

    let existing_user = sqlx::query!("SELECT id FROM users WHERE email = $1", new_user.email)
        .fetch_optional(&pool)
        .await?;

    if existing_user.is_some() {
        return Ok(Json(AuthResponse {
            success: false,
            message: "Email already registered".to_string(),
            user_id: None,
        }));
    }

    let password_hash = hash(new_user.password.as_bytes(), DEFAULT_COST).map_err(|_| {
        AppError::HttpError(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to hash password"),
        )
    })?;

    let profile_picture_url = if let Some(profile_picture_data) = &new_user.profile_picture {
        let cloudinary_config = CloudinaryConfig::from_env()
            .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e))?;
        let cloudinary = CloudinaryService::new(cloudinary_config);

        let temp_public_id = format!("profile_pictures/temp_{}", uuid::Uuid::new_v4());
        let image_url = cloudinary
            .upload_image(profile_picture_data, Some(temp_public_id))
            .await
            .map_err(|e| AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, e))?;
        Some(image_url)
    } else {
        None
    };

    let user = sqlx::query!(
        "INSERT INTO users (email, password_hash, name, pin_code, profile_picture) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        new_user.email,
        password_hash,
        new_user.name,
        new_user.pin_code,
        profile_picture_url
    )
    .fetch_one(&pool)
    .await?;

    session.insert("user_id", user.id).await.map_err(|_| {
        AppError::HttpError(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to set session"),
        )
    })?;

    Ok(Json(AuthResponse {
        success: true,
        message: "Registration successful".to_string(),
        user_id: Some(user.id),
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Form(login_request): Form<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        login_request.email
    )
    .fetch_optional(&pool)
    .await?;

    match user {
        Option::Some(user_record) => {
            let is_valid = verify(
                login_request.password.as_bytes(),
                &user_record.password_hash,
            )
            .map_err(|_| {
                AppError::HttpError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    anyhow::anyhow!("Failed to verify password"),
                )
            })?;

            if is_valid {
                session
                    .insert("user_id", user_record.id)
                    .await
                    .map_err(|_| {
                        AppError::HttpError(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            anyhow::anyhow!("Failed to set session"),
                        )
                    })?;

                Ok(Json(AuthResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    user_id: Some(user_record.id),
                }))
            } else {
                Ok(Json(AuthResponse {
                    success: false,
                    message: "Invalid credentials".to_string(),
                    user_id: None,
                }))
            }
        }
        Option::None => Ok(Json(AuthResponse {
            success: false,
            message: "Invalid credentials".to_string(),
            user_id: None,
        })),
    }
}

pub async fn logout(session: Session) -> Result<Json<AuthResponse>, AppError> {
    session.flush().await.map_err(|_| {
        AppError::HttpError(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow::anyhow!("Failed to clear session"),
        )
    })?;

    Ok(Json(AuthResponse {
        success: true,
        message: "Logged out successfully".to_string(),
        user_id: None,
    }))
}

pub async fn check_auth(session: Session) -> Result<Json<AuthResponse>, AppError> {
    match session.get::<i32>("user_id").await {
        Ok(Some(user_id)) => Ok(Json(AuthResponse {
            success: true,
            message: "Authenticated".to_string(),
            user_id: Some(user_id),
        })),
        _ => Ok(Json(AuthResponse {
            success: false,
            message: "Not authenticated".to_string(),
            user_id: None,
        })),
    }
}

pub async fn get_my_user_id(session: Session) -> Result<Json<i32>, AppError> {
    match session.get::<i32>("user_id").await {
        Ok(Some(user_id)) => Ok(Json(user_id)),
        _ => Err(AppError::HttpError(
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Authentication required"),
        )),
    }
}

pub async fn get_my_profile(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<UserProfile>, AppError> {
    let user_id = get_my_user_id(session).await?.0;

    let user = sqlx::query!(
        "SELECT id, email, name, pin_code, profile_picture FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        name: user.name,
        pin_code: user.pin_code,
        profile_picture: user.profile_picture,
    }))
}

pub async fn get_user_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserProfile>, AppError> {
    let user = sqlx::query!(
        "SELECT id, email, name, pin_code, profile_picture FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        name: user.name,
        pin_code: user.pin_code,
        profile_picture: user.profile_picture,
    }))
}

pub async fn update_profile_picture(
    State(pool): State<PgPool>,
    session: Session,
    Form(update): Form<ProfilePictureUpdate>,
) -> Result<Json<UserProfile>, AppError> {
    let user_id = get_my_user_id(session).await?.0;

    let cloudinary_config = CloudinaryConfig::from_env()
        .map_err(|e| AppError::HttpError(http::StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let cloudinary = CloudinaryService::new(cloudinary_config);

    let public_id = format!("profile_pictures/user_{}", user_id);

    let image_url = cloudinary
        .upload_image(&update.profile_picture, Some(public_id))
        .await
        .map_err(|e| {
            tracing::error!("Cloudinary upload failed: {}", e);
            AppError::HttpError(http::StatusCode::INTERNAL_SERVER_ERROR, e)
        })?;

    let user = sqlx::query!(
        "UPDATE users SET profile_picture = $1 WHERE id = $2 RETURNING id, email, name, pin_code, profile_picture",
        image_url,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        name: user.name,
        pin_code: user.pin_code,
        profile_picture: user.profile_picture,
    }))
}
