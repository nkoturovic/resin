## ORMLite

See: https://github.com/kurtbuilds/ormlite

Relies on:

```sh
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

Create db
```sh
# Create the database if it doesn't exist
# For postgres, that is: createdb <dbname>
ormlite init
```

* `ormlite migrate initial` -  Auto generates migrations based on rust structs
* `ormlite up` - Executes migrations

## Permission system

* Field level permissions

Q: Entity vs model permissions?

Built-in permissions + extended

Groups: Owner, Admin, User, Guest
         CRUD,  CRUD, CRUD, CRUD

Actually, maybe both should have permissions?
But, hmm, ... -> Model | check_permissions
I think model only needs permissions...
Export usual permissions???

Base permissions are required for each model defined.
Base permissions are known at compile time
Therefore, those permissions can easily get exported by the API

## Strongly typed clients

* Not part of the initial version
* Should be doable after backend is fully complete, probably better then

src/api
* import export types
* export builtin permissions
* export manip functions

use resin::api::{User}
api.create(User {
    firstname : "John",
    lastname : "Doe"
})

with macros, easily can do:
api.CreateUser(User{
    ...
})

---

# Deprecated

## TODO:

* [ ] Separate nix files for resin and postgres database
  * [ ] Create a separate project for PG database to be used with different apps
  * [ ] It should be configurable and it could run via shell script or directly through nix
* [ ] Fix rust analyzer running inside the nix

## Models

Eventually, resin can be a framework for easily constructing REST APIs

See: [Proof of concept implementation](./src/resin-macros/tests/model_test.rs)

**NOTE:** This is done through boilermates now

```rust
use resin::{Entity, Model};

#[derive(Entity, Model)]
#[resin(table = "people", insertable = InsertPerson)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}
```

Which would generate

Entity:

```rust
#[derive(Serialize, Deserialize)]
pub struct PersonEntity {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
```

Model:

```rust
#[derive(Model, Debug)]
#[ormlite(table = "people", insertable = InsertPerson)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
```

## Posrgres

* `psql` - Open postgres shell
* `\dt` - Show tables
* `SELECT * FROM <table-name>;` - Show table


## Thoughts and ideas

* If the direction is that the resin library should be crated
  * Then, we can create resin (resin-core) folder, and also have resin-macros and maybe some more


More idea on having a single model for Model and Entity

```rust
#derive(Debug)
#entity_derive(Serialize,Deserialize)
struct User {
    id: i32,
    name: String,
    age: i32,

    #[entity_field]
    age: i32,

    #[model_field]
    born: Date,
}
```

Goals:

* Reduce boilerplate code
* Lower the possibility for error 


It would be good to consider making this even more generic

```rust
#[multi_struct(UserModel, UserEntity)]
// Optionally the union struct can be kept
#[multi_struct_keep_original]
#derive(Debug)
struct User {
    id: i32,
    name: String,
    age: i32,

    #[only(UserEntity)]
    age: i32,

    #[only(UserModel)]
    born: Date,
}
```
