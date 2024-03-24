use std::fmt;

use crate::model::DateTimeRfc3339;
use todo_kernel::model::users::{NewUser, User, UserId};

pub struct UserView {
    pub id: String,
    pub name: String,
    pub created_at: DateTimeRfc3339,
    pub updated_at: DateTimeRfc3339,
}

pub struct CreateUser {
    pub name: String,
}

impl CreateUser {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<User> for UserView {
    fn from(t: User) -> Self {
        Self {
            id: t.id.to_string(),
            name: t.name.to_string(),
            created_at: t.created_at.into(),
            updated_at: t.updated_at.into(),
        }
    }
}

impl From<CreateUser> for NewUser {
    fn from(t: CreateUser) -> Self {
        Self {
            id: UserId::new(),
            name: t.name.try_into().unwrap(),
        }
    }
}

impl fmt::Display for CreateUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
