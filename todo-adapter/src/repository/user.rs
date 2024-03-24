use crate::model::user::StoredUser;
use crate::repository::DatabaseRepositoryImpl;
use async_trait::async_trait;
use sqlx::query_as;
use todo_kernel::model::users::{User, UserId};
use todo_kernel::repository::user::UserRepository;
use tracing::info;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn get(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        info!("In repository, run `get` by `{}`.", id);

        let pool = self.db.0.clone();
        let sql = r#"
            select
                current_user.id as id,
                current_user.name as title,
                current_user.created_at as created_at,
                current_user.updated_at as updated_at
            from
                (select * from users where id = $1) as current_user
            where
                current_user.id = $1
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
}
