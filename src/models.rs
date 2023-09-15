use ormlite::model::*;

#[derive(Model, Debug)]
#[ormlite(insertable = InsertPerson)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
