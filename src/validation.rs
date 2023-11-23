use std::collections::HashMap;

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

pub struct ValidationOpts;
impl ValidationOpts {
    pub const SKIP_REQUIRED: u8 = 0x1;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T, const OPTS: u8 = 0x0>(pub T);

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
impl<T, S, B, const OPTS: u8> FromRequest<S, B> for ValidatedJson<T, OPTS>
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
        if (ValidationOpts::SKIP_REQUIRED & OPTS) != 0 {
            match value.validate() {
                Ok(()) => Ok(ValidatedJson(value)),
                Err(errs) => {
                    let field_erros = errs.field_errors();
                    let filtered: HashMap<_, _> = field_erros
                        .iter()
                        .map(|(fname, ferrs)| {
                            (
                                fname,
                                ferrs
                                    .iter()
                                    .filter(|e| e.code != "required")
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect();

                    if filtered.is_empty() {
                        Ok(ValidatedJson(value))
                    } else {
                        let mut result_errs = ValidationErrors::new();
                        for (fname, ferrs) in filtered {
                            ferrs
                                .iter()
                                .for_each(|err| result_errs.add(fname, (*err).clone()))
                        }
                        if !result_errs.is_empty() {
                            return Err(ServerError::ValidationError(result_errs));
                        }
                        Ok(ValidatedJson(value))
                    }
                }
            }
        } else {
            value.validate()?;
            Ok(ValidatedJson(value))
        }
    }
}
