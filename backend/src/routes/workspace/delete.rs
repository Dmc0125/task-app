use rocket::{http::Status, serde::json::Json};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionError, TransactionTrait};

use backend::{
    entities::{
        label,
        prelude::{Label, Task, TaskGroup, Workspace},
        task, task_group, workspace,
    },
    establish_db_connection,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[delete("/workspace/<workspace_id>")]
pub async fn handler(
    workspace_id: i32,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<()>>, ErrorResponse> {
    let server_error_response = ErrorResponse::new(None, Status::InternalServerError);
    let db = establish_db_connection().await;

    if let Err(_) = db {
        return Err(server_error_response);
    }

    let delete_result = db
        .unwrap()
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                // Delete workspace
                let workspace_to_delete = workspace::ActiveModel {
                    id: sea_orm::ActiveValue::Set(workspace_id),
                    user_id: sea_orm::ActiveValue::Set(user.user_id),
                    ..Default::default()
                };
                let delete_workspace_res = Workspace::delete(workspace_to_delete).exec(tx).await?;

                if delete_workspace_res.rows_affected == 0 {
                    return Err(DbErr::Custom(format!(
                        "Workspace with id {} does not exist",
                        workspace_id
                    )));
                }

                // Find all related task groups
                let task_groups = TaskGroup::find()
                    .filter(task_group::Column::UserId.eq(user.user_id))
                    .filter(task_group::Column::WorkspaceId.eq(workspace_id))
                    .all(tx)
                    .await?;

                // Delete related tasks
                if task_groups.len() > 0 {
                    let mut delete_tasks_stmt =
                        Task::delete_many().filter(task::Column::UserId.eq(user.user_id));
                    for task_group in task_groups.iter() {
                        let task_group_id = task_group.id;
                        delete_tasks_stmt =
                            delete_tasks_stmt.filter(task::Column::Id.eq(task_group_id));
                    }
                    delete_tasks_stmt.exec(tx).await?;
                }

                // Delete task groups
                TaskGroup::delete_many()
                    .filter(task_group::Column::WorkspaceId.eq(workspace_id))
                    .filter(task_group::Column::UserId.eq(user.user_id))
                    .exec(tx)
                    .await?;

                // Delete labels
                Label::delete_many()
                    .filter(label::Column::WorkspaceId.eq(workspace_id))
                    .filter(label::Column::UserId.eq(user.user_id))
                    .exec(tx)
                    .await?;

                Ok(())
            })
        })
        .await;

    match delete_result {
        Ok(_) => Ok(Json(SuccessResponse::new(()))),
        Err(delete_err) => match delete_err {
            TransactionError::Transaction(DbErr::Custom(err)) => {
                Err(ErrorResponse::new(Some(err), Status::NotFound))
            }
            _ => Err(ErrorResponse::new(None, Status::InternalServerError)),
        },
    }
}
