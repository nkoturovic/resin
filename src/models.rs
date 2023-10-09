use ormlite::model::*;

#[derive(Debug, Model)]
#[ormlite(table="users", insertable=InsertUser)]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub country: Option<String>,
    pub language: Option<String>,
}
