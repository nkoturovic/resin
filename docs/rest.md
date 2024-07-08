# Rest server

## Models

In terms of the current proposal, models are building blocks for representing data types, validation serialization/deserialization, and potentially
additional characteristics such as authorization, permissions, etc.

### C++ examples

```c++
/* Base class Model will inject json(), get_unsatisfied_constraints() and more similar methods */
struct User : Model<User> {
    Field<int,cnstr::Unique> id;
    Field<std::string, {cnstr::Unique{}, cnstr::Length{1,10}, cnstr::Required}> username;
    Field<std::string, {cnstr::Required{}, cnstr::Length{6,255}}> password;
    Field<std::string, {cnstr::Unique{},cnstr::Required, cnstr::NotEmpty{}, cnstr::Length{2,32}}> email;
    Field<std::string, {cnstr::Required{}, cnstr::Length{2,64}}> firstname;
    Field<std::string, {cnstr::Required{}, cnstr::Length{2,64}}> lastname;
    Field<int, {cnstr::Required{}}> born; // linux time (since epoch)
    Field<std::string> status;
};
```

### Zig examples

```zig
const User = struct { 
  firstname: Valid([]const u8, .{Length{.from = 2, .to = 4}, Capitalized{}}),
  lastname: Valid([]const u8, .{Capitalized}),
};
```


```zig
const std = @import("std");

const Field = struct {
    const Self = @This();
    name : []const u8,
    typ : type,
};

const Model = struct {
  model_name : [] const u8,
  fields: []const Field,
};


const models = &[1]Model {
    Model {
        .model_name = "User",
        .fields = &[1]Field {
            Field {
                .name = "username",
                .typ = [] const u8,
         }},
    }
};

fn generateDocs(ms : []const Model) [2048] u8 {
    var buf : [2048]u8 = undefined;
    for (ms) |m| {
        _ = try std.fmt.bufPrint(&buf, "<h1>{s}</h1>", .{m.model_name});
    }
    return buf;
}

pub fn main() u8 {
    const docs = comptime generateDocs(models);
    std.debug.print("{s}", .{docs});
    return 0;
}
```


### Rust examples


```rust
#[derive(Model, Debug, Clone, Serialize, Deserialize, Validate)]
#[ormlite(table="users", insertable=InsertUser)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    #[validate(email(message = "Invalid e-mail address format"), required)]
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub country: Option<String>,
    pub language: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
```
