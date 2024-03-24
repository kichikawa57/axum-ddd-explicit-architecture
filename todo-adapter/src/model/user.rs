use chrono::{DateTime, Utc};
use sqlx::FromRow;
use todo_kernel::model::users::User;

#[derive(FromRow, Debug)]
pub struct StoredUser {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<StoredUser> for User {
    type Error = anyhow::Error;

    fn try_from(t: StoredUser) -> Result<Self, Self::Error> {
        Ok(User {
            id: t.id.try_into()?,
            name: t.name.try_into()?,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
    }
}
