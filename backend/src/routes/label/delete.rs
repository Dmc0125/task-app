use rocket::{http::Status, serde::json::Json};

use backend::{
    entities::{
        prelude::{Label, TaskGroup, Workspace},
        task_group,
    },
    establish_db_connection,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbBackend, DbErr, EntityTrait, ModelTrait, QueryFilter,
    Statement, TransactionError, TransactionTrait,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[delete("/label/<label_id>")]
pub async fn handler(
    label_id: i32,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<()>>, ErrorResponse> {
    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;
    if let Err(_) = db_res {
        return Err(server_err_response);
    }
    let db = db_res.unwrap();

    let tx_res = db
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                let label_to_delete_with_workspace = Label::find_by_id(label_id)
                    .find_also_related(Workspace)
                    .one(tx)
                    .await?;

                let not_found_err_msg = format!(
                    "Could not delete label with id {}, label does not exist",
                    label_id
                );
                if None == label_to_delete_with_workspace {
                    return Err(DbErr::Custom(not_found_err_msg));
                }

                let (label_to_delete, related_workspace) = label_to_delete_with_workspace.unwrap();

                if label_to_delete.user_id != user.user_id {
                    return Err(DbErr::Custom(not_found_err_msg));
                }
                if None == related_workspace {
                    return Err(DbErr::Conn(sea_orm::RuntimeErr::Internal(String::from(""))));
                }

                label_to_delete.delete(tx).await?;

                // Update tasks containing label id
                let workspace_model = related_workspace.unwrap();
                let related_task_groups = TaskGroup::find()
                    .filter(task_group::Column::WorkspaceId.eq(workspace_model.id))
                    .filter(task_group::Column::UserId.eq(user.user_id))
                    .all(tx)
                    .await?;

                let mut task_groups_conditions: Vec<String> = vec![];
                for task_group_model in related_task_groups.iter() {
                    task_groups_conditions.push(
                        format!(r#""task"."task_group_id" = {}"#, task_group_model.id)
                    );
                }

                let update_tasks_stmt = format!(
                    r#"UPDATE "task" SET "labels_ids" = array_remove("task"."labels_ids", {}) WHERE "task"."user_id" = {} AND "task"."labels_ids" @> ARRAY[{}] AND ({})"#,
                    label_id,
                    user.user_id,
                    label_id,
                    task_groups_conditions.join(" OR ")
                );
                tx.execute(Statement::from_string(
                    DbBackend::Postgres,
                    update_tasks_stmt
                )).await?;

                Ok(())
            })
        })
        .await;

    match tx_res {
        Ok(_) => Ok(Json(SuccessResponse::new(()))),
        Err(tx_err) => {
            return match tx_err {
                TransactionError::Transaction(db_err) => match db_err {
                    DbErr::Custom(err) => Err(ErrorResponse::new(Some(err), Status::NotFound)),
                    _ => Err(server_err_response),
                },
                _ => Err(server_err_response),
            };
        }
    }
}
