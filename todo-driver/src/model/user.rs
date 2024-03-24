use serde::Serialize;
use todo_app::model::user::UserView;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonUser {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserView> for JsonUser {
    fn from(uv: UserView) -> Self {
        Self {
            id: uv.id,
            name: uv.name,
            created_at: uv.created_at.to_string(),
            updated_at: uv.updated_at.to_string(),
        }
    }
}
