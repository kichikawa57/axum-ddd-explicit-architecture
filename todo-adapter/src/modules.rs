use crate::persistence::postgres::Db;
use crate::repository::DatabaseRepositoryImpl;
use todo_kernel::model::todo::status::TodoStatus;
use todo_kernel::model::todo::Todo;
use todo_kernel::model::users::User;
use todo_kernel::repository::todo::status::TodoStatusRepository;
use todo_kernel::repository::todo::TodoRepository;
use todo_kernel::repository::user::UserRepository;

pub struct RepositoriesModule {
    user_repository: DatabaseRepositoryImpl<User>,
    todo_repository: DatabaseRepositoryImpl<Todo>,
    todo_status_repository: DatabaseRepositoryImpl<TodoStatus>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type TodoRepo: TodoRepository;
    type TodoStatusRepo: TodoStatusRepository;

    fn todo_repository(&self) -> &Self::TodoRepo;
    fn user_repository(&self) -> &Self::UserRepo;
    fn todo_status_repository(&self) -> &Self::TodoStatusRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepositoryImpl<User>;
    type TodoRepo = DatabaseRepositoryImpl<Todo>;
    type TodoStatusRepo = DatabaseRepositoryImpl<TodoStatus>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn todo_repository(&self) -> &Self::TodoRepo {
        &self.todo_repository
    }

    fn todo_status_repository(&self) -> &Self::TodoStatusRepo {
        &self.todo_status_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let todo_repository = DatabaseRepositoryImpl::new(db.clone());
        let user_repository = DatabaseRepositoryImpl::new(db.clone());
        let todo_status_repository = DatabaseRepositoryImpl::new(db.clone());
        Self {
            user_repository,
            todo_repository,
            todo_status_repository,
        }
    }
}
