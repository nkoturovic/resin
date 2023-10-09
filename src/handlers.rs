use crate::{models, SharedState};
use ormlite::model::*;

use axum::response::{ErrorResponse, Result};
use axum::{extract::State, response::Html};

use models::User;

pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn db_test_handler(State(state): State<SharedState>) -> Result<Html<String>> {
    let mut conn = state.db_pool.acquire().await.unwrap();

    let result = User::select()
        .fetch_all(conn.as_mut())
        .await;

    let users_str = match result {
        Ok(users) => {
            users
                .iter()
                .map(|ent| format!("<li>{:?}</li>", ent))
                .reduce(|cur: String, nxt: String| cur + &nxt)
                .unwrap_or(String::from("<p>No registered users yet</p>"))
        }

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
