use rocket::{http::Status, serde::json::Json};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionError, TransactionTrait};

use backend::{
    entities::{
        prelude::{Task, TaskGroup},
        task, task_group,
    },
    establish_db_connection,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[delete("/task-group/<task_group_id>")]
pub async fn handler(
    task_group_id: i32,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<()>>, ErrorResponse> {
    let server_error_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;

    if let Err(_) = db_res {
        return Err(server_error_response);
    }

    let db = db_res.unwrap();

    // Delete all task group and all related tasks
    let tx_res = db
        .transaction::<_, (), DbErr>(|tx| {
            Box::pin(async move {
                let delete_task_group_res = TaskGroup::delete_many()
                    .filter(task_group::Column::Id.eq(task_group_id))
                    .filter(task_group::Column::UserId.eq(user.user_id))
                    .exec(tx)
                    .await?;

                if delete_task_group_res.rows_affected == 0 {
                    return Err(DbErr::Custom(format!(
                        "Task group with id {} does not exist",
                        task_group_id
                    )));
                }

                Task::delete_many()
                    .filter(task::Column::TaskGroupId.eq(task_group_id))
                    .filter(task::Column::UserId.eq(user.user_id))
                    .exec(tx)
                    .await?;

                Ok(())
            })
        })
        .await;

    match tx_res {
        Ok(_) => Ok(Json(SuccessResponse::new(()))),
        Err(tx_err) => match tx_err {
            TransactionError::Transaction(DbErr::Custom(err)) => {
                Err(ErrorResponse::new(Some(err), Status::NotFound))
            }
            _ => Err(server_error_response),
        },
    }
}
