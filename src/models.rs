use ormlite::model::*;
use ormlite::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Model, Serialize, Deserialize)]
#[ormlite(table="users", insertable=InsertUser)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub country: Option<String>,
    pub language: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
