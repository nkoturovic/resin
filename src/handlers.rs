use crate::{
    error::ServerError,
    models,
    validation::{ValidatedJson, ValidationOpts},
    AppState,
};
use axum::http::StatusCode;
use axum_extra::extract::WithRejection;
use ormlite::model::*;

use axum::response::{ErrorResponse, Result};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Json,
};
use models::User;
use uuid::Uuid;

pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn get_users_handler(State(state): State<AppState>) -> Result<Html<String>> {
    let mut conn = state.db_pool.acquire().await.unwrap();

    let result = User::select().fetch_all(conn.as_mut()).await;

    let users_str = match result {
        Ok(users) => users
            .iter()
            .map(|ent| format!("<li>{:?}</li>", ent))
            .reduce(|cur: String, nxt: String| cur + &nxt)
            .unwrap_or(String::from("<p>No registered users yet</p>")),

        Err(err) => {
            return Err(ErrorResponse::from(err.to_string()));
        }
    };

    Ok(Html(format!(
        "<h1>Hello, Db!</h1>
        <ul>{}</ul>",
        users_str
    )))
}

pub async fn print_user_handler(
    WithRejection(Json(user), _): WithRejection<Json<User>, ServerError>,
) -> impl IntoResponse {
    Json(user)
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    ValidatedJson(mut user): ValidatedJson<User, { ValidationOpts::SKIP_REQUIRED }>,
) -> impl IntoResponse {
    let mut conn = state.db_pool.acquire().await.unwrap();

    let opt_user_same_email_result = User::select()
        .where_bind("email = ?", user.email.clone().unwrap())
        .fetch_optional(&state.db_pool)
        .await;

    match opt_user_same_email_result {
        Ok(opt) => {
            if let Some(u) = opt {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "msg": format!("email '{}' already taken", u.email.unwrap())
                    })),
                );
            }
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "msg": err.to_string()
                })),
            )
        }
    }

    user.id = Some(Uuid::new_v4());

    match user.insert(&mut conn).await {
        Ok(u) => (StatusCode::CREATED, Json(serde_json::json!({ "user": u }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "msg": e.to_string()
            })),
        ),
    }
}
