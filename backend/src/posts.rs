use crate::auth::get_my_user_id;
use crate::error;
use crate::structs::{DeleteResponse, NewPost, NewPostForm, Post, PostType};
use axum::{
    Form, Json,
    extract::{Path, State},
};
use error::AppError;
use http::StatusCode;
use sqlx::PgPool;
use tower_sessions::Session;

pub async fn list_my_posts(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<Vec<Post>>, AppError> {
    let user_id = get_my_user_id(session).await?.0;

    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         WHERE p.user_id = $1 ORDER BY p.id DESC",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: match row.post_type.as_str() {
                "offer" => PostType::Offer,
                "request" => PostType::Request,
                _ => PostType::Request,
            },
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn list_user_posts(
    State(pool): State<PgPool>,
    Path(userid): Path<i32>,
) -> Result<Json<Vec<Post>>, AppError> {
    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         WHERE p.user_id = $1 ORDER BY p.id DESC",
        userid
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: match row.post_type.as_str() {
                "offer" => PostType::Offer,
                "request" => PostType::Request,
                _ => PostType::Request,
            },
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn list_offers(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<Vec<Post>>, AppError> {
    let user_id = get_my_user_id(session).await?.0;

    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         WHERE p.user_id = $1 AND p.post_type = 'offer' ORDER BY p.id",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: PostType::Offer,
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn list_requests(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<Vec<Post>>, AppError> {
    let user_id = get_my_user_id(session).await?.0;

    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         WHERE p.user_id = $1 AND p.post_type = 'request' ORDER BY p.id",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: PostType::Request,
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn list_community_posts(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<Vec<Post>>, AppError> {
    let _user_id = get_my_user_id(session).await?.0;

    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         ORDER BY p.id DESC"
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: match row.post_type.as_str() {
                "offer" => PostType::Offer,
                "request" => PostType::Request,
                _ => PostType::Request,
            },
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn list_community_offers(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<Vec<Post>>, AppError> {
    let _user_id = get_my_user_id(session).await?.0;

    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         WHERE p.post_type = 'offer' ORDER BY p.id DESC"
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: PostType::Offer,
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn list_community_requests(
    State(pool): State<PgPool>,
    session: Session,
) -> Result<Json<Vec<Post>>, AppError> {
    let _user_id = get_my_user_id(session).await?.0;

    let rows = sqlx::query!(
        "SELECT p.id, p.description, p.categories, p.user_id, p.post_type, p.pin_code, u.name as user_name, u.profile_picture 
         FROM posts p 
         LEFT JOIN users u ON p.user_id = u.id 
         WHERE p.post_type = 'request' ORDER BY p.id DESC"
    )
    .fetch_all(&pool)
    .await?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.id,
            description: row.description,
            categories: row.categories,
            user_id: row.user_id,
            post_type: PostType::Request,
            pin_code: row.pin_code,
            user_name: row.user_name,
            profile_picture: row.profile_picture,
        })
        .collect();

    Ok(Json(posts))
}

pub async fn create_post(
    State(pool): State<PgPool>,
    session: Session,
    Form(form_data): Form<NewPostForm>,
) -> Result<Json<Post>, AppError> {
    let user_id = get_my_user_id(session).await?.0;
    
    // Parse the categories JSON string
    let categories: Vec<String> = serde_json::from_str(&form_data.categories)
        .map_err(|e| AppError::HttpError(StatusCode::BAD_REQUEST, anyhow::anyhow!("Invalid categories format: {}", e)))?;
    
    let new_post = NewPost {
        description: form_data.description,
        categories,
        post_type: form_data.post_type,
        pin_code: form_data.pin_code,
    };
    
    let post_type_str = new_post.post_type.to_string();

    let row = sqlx::query!(
        "INSERT INTO posts (description, categories, user_id, post_type, pin_code) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id, description, categories, user_id, post_type, pin_code",
        new_post.description,
        &new_post.categories,
        user_id,
        post_type_str,
        new_post.pin_code
    )
    .fetch_one(&pool)
    .await?;

    let user = sqlx::query!("SELECT name, profile_picture FROM users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await
        .ok();

    let created_post = Post {
        id: row.id,
        description: row.description,
        categories: row.categories,
        user_id: row.user_id,
        post_type: new_post.post_type,
        pin_code: row.pin_code,
        user_name: user.as_ref().and_then(|u| u.name.clone()),
        profile_picture: user.as_ref().and_then(|u| u.profile_picture.clone()),
    };

    Ok(Json(created_post))
}

pub async fn delete_post(
    State(pool): State<PgPool>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<Json<DeleteResponse>, AppError> {
    let user_id = get_my_user_id(session).await?.0;

    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(Json(DeleteResponse {
            success: true,
            id,
            message: format!("Post with id {} deleted successfully.", id),
        }))
    } else {
        Err(AppError::HttpError(
            StatusCode::NOT_FOUND,
            anyhow::anyhow!("Post with id {} not found for deletion.", id),
        ))
    }
}

pub async fn update_post(
    State(pool): State<PgPool>,
    session: Session,
    Json(post): Json<Post>,
) -> Result<Json<Post>, AppError> {
    let user_id = get_my_user_id(session).await?.0;
    let post_type_str = post.post_type.to_string();

    let result = sqlx::query!(
        "UPDATE posts SET description = $1, categories = $2, post_type = $3, pin_code = $4 WHERE id = $5 AND user_id = $6", 
        post.description,
        &post.categories,
        post_type_str,
        post.pin_code,
        post.id,
        user_id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(Json(post))
    } else {
        Err(AppError::HttpError(
            StatusCode::NOT_FOUND,
            anyhow::anyhow!("Post with id {} not found for update.", post.id),
        ))
    }
}
