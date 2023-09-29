use boilermates::boilermates;
use ormlite::model::*;

#[boilermates("PersonModel", "PersonEntity")]
#[boilermates(attr_for("PersonModel", "#[derive(Model)]"))]
#[boilermates(attr_for(
    "PersonModel",
    "#[ormlite(table = \"people\", insertable = InsertPerson)]"
))]
#[boilermates(attr_for("PersonEntity", "#[derive(Debug)]"))]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
