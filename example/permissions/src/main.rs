use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bitflags::bitflags;

#[derive( Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub country: Option<String>,
    pub language: Option<String>,
}

bitflags! {
    #[derive(Default,Debug,Clone)]
    pub struct CrudPermissions: u16 {
        const CREATE = 0x1;
        const READ = 0x2;
        const UPDATE = 0x4;
        const DELETE = 0x8;
    }
}

#[derive(Default, Clone)]
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
}


enum Group {
    Guest,
    User,
    Moderator,
    Admin,
    Owner,
}


#[derive(Default)]
struct GroupPermissions<P: Default> {
    guest : P,
    user : P,
    moderator : P,
    admin : P,
    owner : P,
}

trait ModelPermissions {
    type Permissions : Default;
    fn permissions() -> GroupPermissions<Self::Permissions>;
}

impl ModelPermissions for User {
    type Permissions = UserPermissions;
    fn permissions() ->  GroupPermissions<UserPermissions> {
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
            ..user_permissions.clone()
        };

        GroupPermissions {
            guest: guest_permissions,
            user: user_permissions,
            moderator: moderator_permissions,
            ..GroupPermissions::default()
        }

    }
}

fn check_permissions<M : ModelPermissions>(requested_permissions : M::Permissions, groups : GroupPermissions<bool>) -> bool {
    let model_permissions = M::permissions();
    let mut given_permissions : M::Permissions = M::Permissions::default();
    if groups.user {
        given_permissions = model_permissions.user
    }

    false
}



fn main() {
}
