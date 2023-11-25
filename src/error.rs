use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(err) => {
                let payload = serde_json::json!({
                    "code" : "validation",
                    "msg" : "Validation errors",
                    "errors": err.field_errors(),
                });
                (StatusCode::BAD_REQUEST, Json(payload))
            }
            ServerError::AxumJsonRejection(err) => {
                let payload = serde_json::json!({
                    "code" : "json",
                    "msg" : err.to_string(),
                });
                (err.status(), Json(payload))
            }
        }
        .into_response()
    }
}
