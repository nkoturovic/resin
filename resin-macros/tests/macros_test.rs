// use resin_macros::*;
use resin_macros::resin_model;
use serde_json;

// macro resin_model crates a model from this struct
#[resin_model(ops = "CRUD")]
struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

#[test]
fn test_entity_print_debug() {
    let user_create = UserCreate {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        age: 31,
    };
    println!("{:?}", user_create);
}

#[test]
fn test_entity_to_json() -> Result<(), serde_json::Error> {
    let user_create = UserCreate {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        age: 31,
    };
    let str = serde_json::to_string(&user_create)?;
    assert_eq!(str, r#"{"first_name":"Jane","last_name":"Doe","age":31}"#);
    Ok(())
}
