use ormlite::model::*;
use ormlite::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use bitflags::bitflags;

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

bitflags! {
    #[derive(Default)]
    pub struct CrudPermissions: u16 {
        const CREATE = 0x1;
        const READ = 0x2;
        const UPDATE = 0x4;
        const DELETE = 0x8;
    }
}

#[derive(Default)]
struct UserPermissions {
    pub model: CrudPermissions,
    pub id: CrudPermissions,
    pub username: CrudPermissions,
    pub email: CrudPermissions,
    pub password: CrudPermissions,
    pub first_name: CrudPermissions,
    pub last_name: CrudPermissions,
    pub date_of_birth: CrudPermissions,
    pub country: CrudPermissions,
    pub language: CrudPermissions,
    pub created_at: CrudPermissions,
    pub updated_at: CrudPermissions,
}

#[derive(PartialEq)]
enum Group {
    Guest,
    User,
    Moderator,
    Admin,
    Owner,
}

trait ModelPermissions {
    type Permissions;
    fn permissions() -> &'static [(Group, Self::Permissions)];
}

impl ModelPermissions for User {
    type Permissions = UserPermissions;
    fn permissions() -> &'static [(Group, Self::Permissions)] {
        let guest_permissions = UserPermissions::default();

        let user_permissions = UserPermissions {
            model: CrudPermissions::READ,
            username: CrudPermissions::READ,
            email: CrudPermissions::READ,
            first_name: CrudPermissions::READ,
            last_name: CrudPermissions::READ,
            date_of_birth: CrudPermissions::READ,
            country: CrudPermissions::READ,
            language: CrudPermissions::READ,
            ..UserPermissions::default()
        };

        let moderator_permissions = UserPermissions {
            model: CrudPermissions::UPDATE | CrudPermissions::READ,
            email: CrudPermissions::UPDATE | CrudPermissions::READ,
            password: CrudPermissions::UPDATE | CrudPermissions::READ,
            ..user_permissions
        };

        [guest_permissions, user_permissions, moderator_permissions]
    }
}

// For requests

// Request ->
// (CREATE, READ, UPDATE, DELETE)
// Which resource is being targeted?
// I: For each handler, when given a Opt Model<bool>, it should be able to check wether all
// required permissions are matched up front + in general, without opts (default list of fields)

// * Model1: ModelPermissions

trait Handler {
    type ModelBool;
    type Error;
    type ClientInfo; // Holds information for deducing who we're talking to
    fn handle();

    fn check_permissions(
        client: Self::ClientInfo,
        requested: Self::ModelBool,
    ) -> Result<(), Self::Error>; // the error here should have contextual info
                                  // about which permissions aren't meet exactly
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
