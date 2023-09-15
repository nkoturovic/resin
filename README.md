## ORMLite

See: https://github.com/kurtbuilds/ormlite

Relies on:

```
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

Create db
```
# Create the database if it doesn't exist. For postgres, that's:
# createdb <dbname>
ormlite init
```

* `ormlite migrate initial` -  Auto generates migrations based on rust structs
* `ormlite up` - Executes migrations
