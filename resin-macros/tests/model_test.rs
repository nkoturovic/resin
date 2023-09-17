// use resin_macros::*;
use resin_macros::resin_model;

// macro resin_model crates a model from this struct
#[resin_model]
struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

#[test]
fn test_macro() {
    let user_model = UserModel {
        first_name: String::from("Jane"),
        last_name: String::from("Doe"),
        age: 31,

    };
    println!("{:?}", user_model);
}
