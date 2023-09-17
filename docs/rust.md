# Rust 

Taken from:

[https://rust-classes.com/chapter_4_3.html](https://rust-classes.com/chapter_4_3.html)

* main.rs for applications
* lib.rs for a library
* mod.rs for sub-modules

Example:

```
|- Cargo.toml
|- src
    |- main.rs
    |- person.rs
```


src/person.rs
```rust
#[derive(Debug)]
pub struct Person {
    pub name: String,
}
```

src/main.rs
```rust
use crate::person::Person;

mod person;

fn main() {
    let me = Person {
        name: "Marcel".to_string(),
    };
    println!("{:?}", me);
}
```

Output
```
Person { name: "Marcel" }
```

## Macros


https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#attributelikemacros


## Tests

* Run tests with `cargo test`
* Files should end with `_test`
* Can be defined in the source file or in separate file under `src/tests`
* Panics are considered test failure
* Tests can return `Result<T, E>` as well
* Display stdout with: `cargo test -- --nocapture`


```rust
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]

// Wrapping tests with mod tests is not required if 
// tests are defined under: src/tests/some_file_test.rs
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }
}
```
