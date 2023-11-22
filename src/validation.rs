use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

pub const VALID_OPTS_CHECK_ALL: char = 'A';
pub const VALID_OPTS_SKIP_REQUIRED: char = 'S';

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidJson<T, const O: char>(pub T);

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
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

#[async_trait]
impl<T, S, B, const O: char> FromRequest<S, B> for ValidJson<T, O>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        if O == VALID_OPTS_SKIP_REQUIRED {
            match value.validate() {
                Ok(()) => (),
                Err(es) => {
                    ler filtered: Vec<_> = es
                        .field_errors()
                        .iter()
                        .filter(|err| String::from(*err.0) != String::from("required"))
                        .collect();

                    if filtered.is_empty() {
                        return Ok(ValidJson(value));
                    } else {
                        let mut errs = ValidationErrors::new();
                        for fers in filtered {
                            for suberr in **fers.1 {
                                errs.add(fers.0, suberr)
                            }
                        }
                        return Err(ServerError::ValidationError(errs));
                    }
                }
            }
        }
        value.validate()?;
        Ok(ValidJson(value))
    }
}
