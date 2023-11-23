mod error;
mod handlers;
mod models;
mod router_extensions;
mod validation;

use axum::response::Result;
use axum::{
    routing::{get, post},
    Router,
};

use ormlite::postgres::Postgres;
use ormlite::Pool;
use std::net::SocketAddr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::handlers::{create_user_handler, get_users_handler, hello_handler, print_user_handler};
use crate::router_extensions::ResinRouterExtenions;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "resin=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        db_pool: ormlite::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://kotur:qweqwe123@localhost:3306")
            .await?,
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/user", get(get_users_handler))
        .route("/user", post(create_user_handler))
        .route("/print_user", post(print_user_handler))
        .add_tracing_layer()
        .with_state(state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
