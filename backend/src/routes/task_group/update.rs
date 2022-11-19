use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::routes::lib::{validate_len, AuthenticatedUser, ErrorResponse, SuccessResponse};

use backend::{
    entities::{prelude::TaskGroup, task_group},
    establish_db_connection,
};

#[patch("/task-group/<task_group_id>", data = "<data>")]
pub async fn handler(
    task_group_id: i32,
    data: Json<ModifiedTaskGroupData>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<ModifiedTaskGroupData>>, ErrorResponse> {
    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;

    if let Err(_) = db_res {
        return Err(server_err_response);
    }

    let db = db_res.unwrap();

    // Validate title len
    let title_len_err = validate_len(&data.title, 1, 50, "Title");
    if let Some(err) = title_len_err {
        return Err(err);
    }

    let select_res = TaskGroup::find()
        .filter(task_group::Column::Id.eq(task_group_id))
        .filter(task_group::Column::UserId.eq(user.user_id))
        .one(&db)
        .await;

    if let Err(_) = select_res {
        return Err(server_err_response);
    }

    match select_res.unwrap() {
        Some(task_group) => {
            let mut task_group_active_model: task_group::ActiveModel = task_group.into();
            task_group_active_model.title = sea_orm::Set(data.title.clone());
            let update_res = task_group_active_model.update(&db).await;

            match update_res {
                Ok(_) => Ok(Json(SuccessResponse::new(ModifiedTaskGroupData {
                    title: data.title.clone(),
                }))),
                Err(_) => Err(server_err_response),
            }
        }
        None => Err(ErrorResponse::new(
            Some(format!("Task group with id {} does not exist", task_group_id).into()),
            Status::NotFound,
        )),
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ModifiedTaskGroupData {
    title: String,
}
