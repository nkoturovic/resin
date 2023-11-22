use crate::{models, AppState};
use axum::extract::rejection::{ExtensionRejection, JsonRejection, JsonSyntaxError};
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::StatusCode;
use axum_garde::WithValidation;
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

pub async fn db_test_handler(State(state): State<AppState>) -> Result<Html<String>> {
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
    WithValidation(_valid_user): WithValidation<Json<User>>,
) -> impl IntoResponse {
    tracing::debug!("Does it work?");
    // let mut conn = state.db_pool.acquire().await.unwrap();

    return (
            StatusCode::BAD_REQUEST,
            Json(format!("{{\"msg\": \"email is required\"}}")),
    );


    // let mut user = valid_user.into_inner();
    // user.id = Some(Uuid::new_v4());

    // // if user.email.is_none() {
    // //     return (
    // //         StatusCode::BAD_REQUEST,
    // //         Json(format!("{{\"msg\": \"email is required\"}}")),
    // //     );
    // // }

    // match user.insert(&mut conn).await {
    //     Ok(u) => (
    //         StatusCode::CREATED,
    //         Json(format!("{{\"user\": \"{:#?}\"}}", Json(u))),
    //     ),
    //     Err(msg) => (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(format!("{{\"msg\": \"{}\"}}", msg.to_string())),
    //     ),
    // }
}

// pub async fn update_user_handler(
//     State(state): State<AppState>,
//     opt_valid_user: Option<WithValidation<Json<User>>>,
// ) -> impl IntoResponse {
//     tracing::debug!("Does it work?");
//     let mut conn = state.db_pool.acquire().await.unwrap();
//     return "Does it work";
//     // let mut user = valid_user.into_inner();
//     // user.id = Some(Uuid::new_v4());
//
//     // if user.email.is_none() {
//     //     return (
//     //         StatusCode::BAD_REQUEST,
//     //         Json(format!("{{\"msg\": \"email is required\"}}")),
//     //     );
//     // }
//
//     // match user.insert(&mut conn).await {
//     //     Ok(u) => (
//     //         StatusCode::CREATED,
//     //         Json(format!("{{\"user\": \"{:#?}\"}}", Json(u))),
//     //     ),
//     //     Err(msg) => (
//     //         StatusCode::INTERNAL_SERVER_ERROR,
//     //         Json(format!("{{\"msg\": \"{}\"}}", msg.to_string())),
//     //     ),
//     // }
// }
