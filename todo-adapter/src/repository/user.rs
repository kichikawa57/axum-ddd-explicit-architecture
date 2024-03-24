use crate::model::user::StoredUser;
use crate::repository::DatabaseRepositoryImpl;
use async_trait::async_trait;
use sqlx::{query, query_as};
use todo_kernel::model::users::{NewUser, User, UserId};
use todo_kernel::repository::user::UserRepository;
use tracing::info;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn get(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        info!("In repository, run `get` by `{}`.", id);

        let pool = self.db.0.clone();
        let sql = r#"
            select
                u.id as id,
                u.name as name,
                u.created_at as created_at,
                u.updated_at as updated_at
            from
                (select * from users where id = $1) as u
            where
                u.id = $1
        "#;
        let stored_user = query_as::<_, StoredUser>(sql)
            .bind(id.to_string())
            .fetch_one(&*pool)
            .await
            .ok();

        match stored_user {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, new: &NewUser) -> anyhow::Result<User> {
        info!("In repository, run `insert`.");

        let pool = self.db.0.clone();
        let _ = query("insert into users (id, name) values ($1, $2)")
            .bind(new.id.to_string())
            .bind(new.name.to_string())
            .execute(&*pool)
            .await?;

        let sql = r#"
            select
                u.id as id,
                u.name as name,
                u.created_at as created_at,
                u.updated_at as updated_at
            from
                (select * from users where id = $1) as u
            where
                u.id = $1
        "#;

        info!("In repository, run selecting by {}", new.id.to_string());

        let stored_user = query_as::<_, StoredUser>(sql)
            .bind(new.id.to_string())
            .fetch_one(&*pool)
            .await?;

        Ok(stored_user.try_into()?)
    }
}
