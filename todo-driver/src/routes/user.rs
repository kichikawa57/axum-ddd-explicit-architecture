use crate::model::user::JsonUser;
use crate::module::{Modules, ModulesExt};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use tracing::log::{error, info};

pub async fn get_by_id(
    Path(id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("In routse, run `get_by_id` by `{}`.", id);

    let resp = modules.user_use_case().get_by_id(id).await;

    match resp {
        Ok(tv) => tv
            .map(|tv| {
                info!("Found user `{}`.", tv.id);
                let json: JsonUser = tv.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("User is not found.");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
