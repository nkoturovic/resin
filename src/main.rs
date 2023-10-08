mod models; // import models module
mod resin_extensions;

// use crate::types::{PersonEntity, PersonModel};
use ormlite::model::*;

use axum::response::{ErrorResponse, Response, Result};
use axum::{extract::State, response::Html, routing::get, Router};

use ormlite::postgres::Postgres;
use ormlite::Pool;
use std::{error::Error, net::SocketAddr, sync::Arc};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    models::{PersonEntity, PersonModel},
    resin_extensions::ResinRouterExtenions,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

type SharedState = Arc<AppState>;

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

    let state = SharedState::from(AppState {
        db_pool: ormlite::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://kotur:qweqwe123@localhost:3306")
            .await?,
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/db-test", get(db_test_handler))
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

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn db_test_handler(State(state): State<SharedState>) -> Result<Html<String>> {
    let mut conn = state.db_pool.acquire().await.unwrap();

    let result = PersonModel::select()
        .where_("age > ?")
        .bind(50)
        .fetch_all(conn.as_mut())
        .await;

    let ents_str = match result {
        Ok(people_models) => {
            let people_ents: Vec<PersonEntity> =
                people_models.into_iter().map(|m| m.into()).collect();

            let ent_str = people_ents
                .iter()
                .map(|ent| format!("<li>{:?}</li>", ent))
                .reduce(|cur: String, nxt: String| cur + &nxt)
                .unwrap();

            tracing::debug!("{:?}", ent_str);
            ent_str
        }

        Err(err) => {
            return Err(ErrorResponse::from(err.to_string()));
        }
    };

    // You can insert the model directly.
    //let mut john_model = PersonModel {
    //    id: people_ents.last().map_or(0, |p| p.id + 1),
    //    name: "John".to_string(),
    //    age: 50,
    //}
    //.insert(&mut conn)
    //.await?;

    //let mut john_entity = PersonEntity::from(john_model);
    //println!("{:?}", john_entity);

    //// After modifying the object, you can update all its fields.
    //john_entity.age += 1;

    //john_model = john_entity.into();
    //john_model.update_all_fields(conn.as_mut()).await?;

    Ok(Html(format!(
        "<h1>Hello, Db!</h1>
        <ul>{}</ul>",
        ents_str
    )))
}
