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


## TODO:

* [ ] Separate nix files for resin and postgres database
  * [ ] Create a separate project for PG database to be used with different apps
  * [ ] It should be configurable and it could run via shell script or directly through nix
* [ ] Fix rust analyzer running inside the nix

## Models

Eventually, resin can be a framework for easily constructing REST APIs

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
