use ormlite::model::*;
use ormlite::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use uuid::Uuid;

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

#[derive(Default)]
struct UserPermissions {
    pub model: u16,
    pub id: u16,
    pub username: u16,
    pub email: u16,
    pub password: u16,
    pub first_name: u16,
    pub last_name: u16,
    pub date_of_birth: u16,
    pub country: u16,
    pub language: u16,
    pub created_at: u16,
    pub updated_at: u16,
}

#[derive(PartialEq)]
enum Group {
    Guest,
    User,
    Moderator,
    Admin,
    Owner,
}

trait Permitable {
    type PermissionsType;
    fn get_permissions(group: Group) -> Self::PermissionsType;
}

impl Permitable for User {
    type PermissionsType = UserPermissions;
    fn get_permissions(group: Group) -> Self::PermissionsType {
        if group == Group::Guest {
            return UserPermissions::default()
        }

        let user_permissions = UserPermissions {
            model: 0x8,
            username: 0x8,
            email: 0x8,
            first_name: 0x8,
            last_name: 0x8,
            date_of_birth: 0x8,
            country: 0x8,
            language: 0x8,
            ..UserPermissions::default()
        };

        if group == Group::User {
            return user_permissions
        }

        if group == Group::Moderator {
            return UserPermissions {
                model: 0xC,
                email: 0xC,
                password: 0xC,
                ..user_permissions
            };
        }
        UserPermissions::default()
    }
}

//struct Post;
//struct PostPermissions;
//const fn get_user_permissions<User>(group: Group) -> UserPermissions {
//    match group {
//        Group::Moderator => UserPermissions {
//            model: 0x0,
//            id: 0x0,
//            username: 0x0,
//            email: 0x0,
//            password: 0x0,
//            first_name: 0x0,
//            last_name: 0x0,
//            date_of_birth: 0x0,
//            country: 0x0,
//            language: 0x0,
//            created_at: 0x0,
//            updated_at: 0x0,
//        },
//        _ => UserPermissions {
//            model: 0x0,
//            id: 0x0,
//            username: 0x0,
//            email: 0x0,
//            password: 0x0,
//            first_name: 0x0,
//            last_name: 0x0,
//            date_of_birth: 0x0,
//            country: 0x0,
//            language: 0x0,
//            created_at: 0x0,
//            updated_at: 0x0,
//        },
//    }
//}
//

// struct Moderator;
// struct Admin;
//
// // TODO: Can be derived with a macro
// trait Permissions<M> {
//     type PermissionsType;
//     const PERMISSIONS : Self::PermissionsType;
// }
//
// impl Permissions<User> for Moderator {
//     type PermissionsType = UserPermissions;
//     const PERMISSIONS : Self::PermissionsType =
//         UserPermissions {
//             model: 0x0,
//             id: 0x0,
//             username: 0x0,
//             email: 0x0,
//             password: 0x0,
//             first_name: 0x0,
//             last_name: 0x0,
//             date_of_birth: 0x0,
//             country: 0x0,
//             language: 0x0,
//             created_at: 0x0,
//             updated_at: 0x0,
//     };
// }

// #[derive(Model, Debug, Clone, Serialize, Deserialize, Validate)]
// #[ormlite(table="roles", insertable=InsertGroup)]
// pub struct Role {
//     pub id: Option<Uuid>,
//     pub name: Option<String>,
// }
//
// pub struct Permission {
//     pub id: Option<Uuid>,
//     pub name: Option<String>,
//     pub table_name:
//     pub permissions: Option<HashMap<String, u16>>,
// }
