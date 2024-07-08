use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bitflags::bitflags;
use std::ops::BitOr;
use std::ops::BitXor;

#[derive( Default, Debug, Clone, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone)]
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

impl BitOr for UserPermissions {
    type Output = UserPermissions;
    fn bitor(self : Self, rhs : Self) -> Self {
        UserPermissions {
            model: self.model | rhs.model,
            id: self.id | rhs.id,
            username: self.username | rhs.username,
            email: self.email | rhs.email,
            password: self.password | rhs.password,
            first_name: self.first_name | rhs.first_name,
            last_name: self.last_name | rhs.last_name,
            date_of_birth: self.date_of_birth | rhs.date_of_birth,
            country: self.country | rhs.country,
            language: self.language | rhs.language
        }
    }
}

impl BitXor for UserPermissions {
    type Output = UserPermissions;
    fn bitxor(self : Self, rhs : Self) -> Self {
        UserPermissions {
            model: self.model ^ rhs.model,
            id: self.id ^ rhs.id,
            username: self.username ^ rhs.username,
            email: self.email ^ rhs.email,
            password: self.password ^ rhs.password,
            first_name: self.first_name ^ rhs.first_name,
            last_name: self.last_name ^ rhs.last_name,
            date_of_birth: self.date_of_birth ^ rhs.date_of_birth,
            country: self.country ^ rhs.country,
            language: self.language ^ rhs.language
        }
    }
}


// enum Group {
//     Guest,
//     User,
//     Moderator,
//     Admin,
//     Owner,
// }


#[derive(Default)]
struct GroupPermissions<P: Default + BitOr + BitXor> {
    guest : P,
    user : P,
    moderator : P,
    admin : P,
    owner : P,
}

impl <P> BitOr for GroupPermissions<P> where P: Default + BitOr<Output = P> + BitXor<Output = P> {
    type Output = GroupPermissions<P>;
    fn bitor(self : Self, rhs : GroupPermissions<P>) -> GroupPermissions<P> {
        GroupPermissions::<P> {
            guest: self.guest | rhs.guest,
            user: self.user | rhs.user,
            admin: self.admin | rhs.admin,
            moderator: self.moderator | rhs.moderator,
            owner: self.owner | rhs.owner,
        }
    }
}

trait ModelPermissions {
    type Permissions : Default + BitOr + BitXor;
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

fn check_permissions<M : ModelPermissions>(requested_permissions : M::Permissions, groups : GroupPermissions<bool>) -> M::Permissions
where <M as ModelPermissions>::Permissions: BitOr<Output=M::Permissions> + BitXor<Output=M::Permissions> {
    let model_permissions = M::permissions();
    let mut given_permissions : M::Permissions = M::Permissions::default();
    if groups.user {
        given_permissions = given_permissions | model_permissions.user
    }
    given_permissions ^ requested_permissions
}



fn main() {

    let u = User::default();
    let result = check_permissions::<User>(
        UserPermissions{ 
            first_name: CrudPermissions::READ,
            ..UserPermissions::default()
        },
        GroupPermissions {
            guest: false,
            user: true,
            ..GroupPermissions::default()
        }
    );

    println!("{:?}", result);

}
