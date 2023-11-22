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
    let user_perms = UserPerms {
        first_name: 0b0010,
        last_name: 0b1101,
        age: 31,
    };
    println!("{:?}", user_perms);
}

#[test]
fn test_entity_to_json() -> Result<(), serde_json::Error> {
    let user_perms = UserPerms {
        first_name: 0b0010,
        last_name: 0b1011,
        age: 0b1011,
    };
    println!("{:#?}", user_perms);
    Ok(())
}
