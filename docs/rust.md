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
