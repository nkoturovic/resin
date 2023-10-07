mod models; // import models module
mod resin_extensions;

// use crate::types::{PersonEntity, PersonModel};
// use ormlite::model::*;

use axum::{
    response::Html,
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;

use crate::resin_extensions::ResinRouterExtenions;



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

    // build our application with a route
   let app = Router::new()
        .route("/", get(handler))
        .add_tracing_layer();
 
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
//    let pool = ormlite::postgres::PgPoolOptions::new()
//        .max_connections(5)
//        .connect("postgres://kotur:qweqwe123@localhost:3306")
//        .await?;
//
//     let mut conn = pool.acquire().await.unwrap();
//     // Query builder syntax closely follows SQL syntax, translated into chained function calls.
//     let people_models = PersonModel::select()
//         .where_("age > ?")
//         .bind(50)
//         .fetch_all(conn.as_mut())
//         .await?;
//
//     let people_ents: Vec<PersonEntity> = people_models.into_iter().map(|m| m.into()).collect();
//     println!("{:?}", people_ents);
//
//     // You can insert the model directly.
//     let mut john_model = PersonModel {
//         id: people_ents.last().map_or(0, |p| p.id + 1),
//         name: "John".to_string(),
//         age: 50,
//     }
//     .insert(&mut conn)
//     .await?;
//
//     let mut john_entity = PersonEntity::from(john_model);
//     println!("{:?}", john_entity);
//
//     // After modifying the object, you can update all its fields.
//     john_entity.age += 1;
//
//     john_model = john_entity.into();
//     john_model.update_all_fields(conn.as_mut()).await?;
//
//     Ok(())
// }
