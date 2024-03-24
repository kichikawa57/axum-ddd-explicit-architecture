use std::fmt;

use crate::model::Id;
use anyhow::anyhow;
use chrono::{DateTime, Utc};

pub struct UserId(Id<User>);
pub struct UserName(String);

pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewUser {
    pub id: UserId,
    pub name: UserName,
}

impl NewUser {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
}

pub struct UpdateUser {
    pub id: UserId,
    pub name: Option<UserName>,
}

impl UpdateUser {
    pub fn new(id: UserId, name: Option<UserName>) -> Self {
        Self { id, name }
    }
}

pub struct UpsertUser {
    pub id: UserId,
    pub name: UserName,
}

impl UpsertUser {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
}

impl TryFrom<String> for UserId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(UserId(value.try_into()?))
    }
}

impl TryFrom<String> for UserName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.is_empty() {
            Ok(UserName(value))
        } else {
            Err(anyhow!("Missing attribute"))
        }
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
