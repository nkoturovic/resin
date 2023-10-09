use crate::{models, SharedState};
use ormlite::model::*;

use axum::response::{ErrorResponse, Result};
use axum::{extract::State, response::Html};

use models::{PersonEntity, PersonModel};

pub async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn db_test_handler(State(state): State<SharedState>) -> Result<Html<String>> {
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
