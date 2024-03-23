use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// User Domain Entity
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub email: String,
    pub display_name: Option<String>,
    pub image: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub token: String,
}

// DTOs
#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub email: String,
    pub role: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestLoginUser {
    pub email: String,
    pub password: String,
}
