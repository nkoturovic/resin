use crate::{
    models,
    validation::{ValidatedJson, ValidationOpts},
    AppState,
};
use axum::http::StatusCode;
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

pub async fn create_user_handler(
    State(state): State<AppState>,
    ValidatedJson(mut user): ValidatedJson<User, { ValidationOpts::SKIP_REQUIRED }>,
) -> impl IntoResponse {
    let mut conn = state.db_pool.acquire().await.unwrap();
    user.id = Some(Uuid::new_v4());

    // TODO(nkoturovic): You need to have multiple structs
    // with different defaults to avoid this (can boilermates solve it??)
    // The issue could be different field macros that are needed, actually
    // that is the main thing that is messing the things currently

    // if user.email.is_none() {
    //     return (
    //         StatusCode::BAD_REQUEST,
    //         Json(format!("{{\"msg\": \"email is required\"}}")),
    //     );
    // }

    match user.insert(&mut conn).await {
        Ok(u) => (
            StatusCode::CREATED,
            Json(format!("{{\"user\": \"{:#?}\"}}", Json(u))),
        ),
        Err(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("{{\"msg\": \"{}\"}}", msg.to_string())),
        ),
    }
}
