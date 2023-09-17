use ormlite::model::*;

#[derive(Model, Debug)]
#[ormlite(table = "people", insertable = InsertPerson)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
