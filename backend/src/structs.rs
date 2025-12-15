use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Offer,   
    Request, 
}

impl std::fmt::Display for PostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostType::Offer => write!(f, "offer"),
            PostType::Request => write!(f, "request"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub description: String,
    pub categories: Vec<String>,
    pub user_id: i32,
    pub post_type: PostType,
    pub pin_code: Option<String>,
    pub user_name: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPost {
    pub description: String,
    pub categories: Vec<String>,
    pub post_type: PostType,
    pub pin_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPostForm {
    pub description: String,
    pub categories: String, // JSON string that will be parsed to Vec<String>
    pub post_type: PostType,
    pub pin_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    pub success: bool,
    pub id: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub pin_code: Option<String>,
    pub name: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
    pub pin_code: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub pin_code: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfilePictureUpdate {
    pub profile_picture: String,
}
