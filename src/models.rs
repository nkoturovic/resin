use garde::Validate;
use ormlite::model::*;
use ormlite::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;
use crate::AppState;

struct UserContext {
    check_required: bool,
}

impl Default for UserContext {
    fn default() -> Self {
        UserContext {
            check_required: false,
        }
    }
}

fn maybe_required<T>(opt_value: &Option<T>, context: &AppState) -> garde::Result {
    if context.check_required {
        if opt_value.is_none() {
            return Err(garde::Error::new("value is required"));
        }
    }

    Ok(())
}

#[derive(Debug, Model, Serialize, Deserialize, Validate)]
#[ormlite(table="users", insertable=InsertUser)]
#[garde(context(AppState as ctx))]
pub struct User {
    #[garde(skip)]
    pub id: Option<Uuid>,

    #[garde(skip)]
    pub username: Option<String>,

    #[garde(skip)]
    pub email: Option<String>,

    #[garde(custom(maybe_required))]
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
    pub created_at: Option<DateTime<Utc>>,

    #[garde(skip)]
    pub updated_at: Option<DateTime<Utc>>,
}
