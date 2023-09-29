use crate::models::{PersonEntity, PersonModel};
use ormlite::model::*;

mod models; // import models module

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start by making a database connection.
    let pool = ormlite::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://kotur:qweqwe123@localhost:3306")
        .await?;

    let mut conn = pool.acquire().await.unwrap();

    // Query builder syntax closely follows SQL syntax, translated into chained function calls.
    let people_models = PersonModel::select()
        .where_("age > ?")
        .bind(50)
        .fetch_all(conn.as_mut())
        .await?;

    let people_ents: Vec<PersonEntity> = people_models.into_iter().map(|m| m.into()).collect();
    println!("{:?}", people_ents);

    // You can insert the model directly.
    let mut john_model = PersonModel {
        id: people_ents.last().map_or(0, |p| p.id + 1),
        name: "John".to_string(),
        age: 50,
    }
    .insert(&mut conn)
    .await?;

    let mut john_entity = PersonEntity::from(john_model);
    println!("{:?}", john_entity);

    // After modifying the object, you can update all its fields.
    john_entity.age += 1;

    john_model = john_entity.into();
    john_model.update_all_fields(conn.as_mut()).await?;

    Ok(())
}
