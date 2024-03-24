use std::sync::Arc;
use todo_adapter::modules::{RepositoriesModule, RepositoriesModuleExt};
use todo_adapter::persistence::postgres::Db;
use todo_adapter::repository::health_check::HealthCheckRepository;
use todo_app::usecase::health_check::HealthCheckUseCase;
use todo_app::usecase::todo::TodoUseCase;
use todo_app::usecase::user::UserUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
    todo_use_case: TodoUseCase<RepositoriesModule>,
    user_use_case: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase;
    fn todo_use_case(&self) -> &TodoUseCase<Self::RepositoriesModule>;
    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }

    fn todo_use_case(&self) -> &TodoUseCase<Self::RepositoriesModule> {
        &self.todo_use_case
    }

    fn user_use_case(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_use_case
    }
}

impl Modules {
    pub async fn new() -> Self {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new(db));
        let todo_use_case = TodoUseCase::new(repositories_module.clone());
        let user_use_case = UserUseCase::new(repositories_module.clone());

        Self {
            health_check_use_case,
            todo_use_case,
            user_use_case,
        }
    }
}
