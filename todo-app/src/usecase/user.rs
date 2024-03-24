use crate::model::user::UserView;
use std::sync::Arc;
use todo_adapter::modules::RepositoriesModuleExt;
use todo_kernel::repository::user::UserRepository;
use tracing::log::info;

pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn get_by_id(&self, id: String) -> anyhow::Result<Option<UserView>> {
        info!("In usecase, run `get_by_id` by `{}`.", id);

        let resp = self
            .repositories
            .user_repository()
            .get(&id.try_into()?)
            .await?;

        match resp {
            Some(t) => Ok(Some(t.into())),
            None => Ok(None),
        }
    }
}
