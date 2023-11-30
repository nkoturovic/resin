use std::collections::HashMap;

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, rejection::JsonRejection},
    Json, http,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

use crate::error::ServerError;

pub struct ValidationOpts;
impl ValidationOpts {
    pub const SKIP_REQUIRED: u8 = 0x1;
}

/// Type alias for [`http::Request`] whose body type defaults to [`Body`], the most common body
/// type used with axum.
pub type Request<T = Body> = http::Request<T>;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T, const OPTS: u8 = 0x0>(pub T);

#[async_trait]
impl<T, S, const OPTS: u8> FromRequest<S> for ValidatedJson<T, OPTS>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
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
