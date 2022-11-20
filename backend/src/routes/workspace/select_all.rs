use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use backend::{
    entities::{prelude::Workspace, workspace},
    establish_db_connection,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[get("/workspace")]
pub async fn handler(
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<Vec<FoundWorkspace>>>, ErrorResponse> {
    let select_res: Result<Vec<workspace::Model>, DbErr> = async move {
        let db = establish_db_connection().await?;
        let select_workspaces = Workspace::find()
            .filter(workspace::Column::UserId.eq(user.user_id))
            .all(&db)
            .await?;
        Ok(select_workspaces)
    }
    .await;

    match select_res {
        Ok(workspaces_models) => {
            let mut workspaces: Vec<FoundWorkspace> = vec![];
            for workspace_model in workspaces_models.iter() {
                workspaces.push(FoundWorkspace {
                    id: workspace_model.id,
                    title: workspace_model.title.clone(),
                    description: workspace_model.description.clone(),
                })
            }
            Ok(Json(SuccessResponse::new(workspaces)))
        }
        Err(_) => Err(ErrorResponse::new(None, Status::InternalServerError)),
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FoundWorkspace {
    id: i32,
    title: String,
    description: Option<String>,
}
