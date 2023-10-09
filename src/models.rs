use garde::Validate;
use ormlite::model::*;
use ormlite::types::chrono::{DateTime, Utc};

// use uuid::Uuid; // Do I want to use UUID??

#[derive(Debug, Model, Validate)]
#[ormlite(table="users", insertable=InsertUser)]
pub struct User {
    #[garde(skip)]
    pub id: i64,

    #[garde(skip)]
    pub username: Option<String>,

    #[garde(email)]
    pub email: Option<String>,

    #[garde(skip)]
    pub password: Option<String>,

    #[garde(skip)]
    pub first_name: Option<String>,

    #[garde(skip)]
    pub last_name: Option<String>,

    #[garde(skip)]
    pub date_of_birth: Option<String>,

    #[garde(skip)]
    pub country: Option<String>,

    #[garde(skip)]
    pub language: Option<String>,

    #[garde(skip)]
    pub created_at: DateTime<Utc>,

    #[garde(skip)]
    pub updated_at: DateTime<Utc>,
}
