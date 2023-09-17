use crate::models::Person;
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
    let people = Person::select()
        .where_("age > ?")
        .bind(50)
        .fetch_all(conn.as_mut())
        .await?;
    println!("{:?}", people);

    // You can insert the model directly.
    let mut john = Person {
        id: people.last().map_or(0, |p| p.id + 1),
        name: "John".to_string(),
        age: 50,
    }
    .insert(&mut conn)
    .await?;
    println!("{:?}", john);

    // After modifying the object, you can update all its fields.
    john.age += 1;
    john.update_all_fields(conn.as_mut()).await?;

    Ok(())
}
