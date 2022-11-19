use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use sea_orm::{ColumnTrait, DbErr, DeleteResult, EntityTrait, QueryFilter};

use backend::{
    entities::{prelude::Workspace, workspace},
    establish_db_connection,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[delete("/workspace/<workspace_id>")]
pub async fn handler(
    workspace_id: i32,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<DeletedWorkspace>>, ErrorResponse> {
    // TODO: delete all labels, task-group and tasks related to this workspace
    let delete_workspace_res: Result<DeleteResult, DbErr> = async move {
        let db = establish_db_connection().await?;
        let delete_result = Workspace::delete_many()
            .filter(workspace::Column::Id.eq(workspace_id))
            .filter(workspace::Column::UserId.eq(user.user_id))
            .exec(&db)
            .await?;

        Ok(delete_result)
    }
    .await;

    match delete_workspace_res {
        Ok(delete_res) => {
            if delete_res.rows_affected == 0 {
                return Err(ErrorResponse::new(
                    format!("Workspace with id {} does not exist", workspace_id).into(),
                    Status::NotFound,
                ));
            }

            Ok(Json(SuccessResponse::new(DeletedWorkspace {
                id: workspace_id,
            })))
        }
        Err(_) => Err(ErrorResponse::new(None, Status::InternalServerError)),
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeletedWorkspace {
    pub id: i32,
}
