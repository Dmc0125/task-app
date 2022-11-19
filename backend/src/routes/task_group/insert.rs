use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};

use backend::{
    entities::{prelude::Workspace, task_group, workspace},
    establish_db_connection,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

use crate::routes::lib::{validate_len, AuthenticatedUser, ErrorResponse, SuccessResponse};

#[post("/task-group", data = "<data>")]
pub async fn handler(
    data: Json<NewTaskGroup>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<InsertedTaskGroup>>, ErrorResponse> {
    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;

    if let Err(_) = db_res {
        return Err(server_err_response);
    }

    let db = db_res.unwrap();
    // Validate workspace_id
    let get_workspace_res = Workspace::find_by_id(data.workspace_id)
        .filter(workspace::Column::UserId.eq(user.user_id))
        .one(&db)
        .await;

    if let Err(_) = get_workspace_res {
        return Err(server_err_response);
    }

    // Validate title
    let title_err = validate_len(&data.title, 1, 50, "Title");

    if let Some(err) = title_err {
        return Err(err);
    }

    match &get_workspace_res.unwrap() {
        Some(_) => {
            let insert_res = task_group::ActiveModel {
                user_id: ActiveValue::Set(user.user_id),
                workspace_id: ActiveValue::Set(data.workspace_id),
                title: ActiveValue::Set(data.title.clone()),
                ..Default::default()
            }
            .insert(&db)
            .await;

            match insert_res {
                Ok(inserted_task_group) => Ok(Json(SuccessResponse::new(InsertedTaskGroup {
                    title: inserted_task_group.title,
                }))),
                Err(_) => Err(server_err_response),
            }
        }
        None => Err(ErrorResponse::new(
            Some(format!("Workspace with id {} does not exist", data.workspace_id).into()),
            Status::NotFound,
        )),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewTaskGroup {
    pub workspace_id: i32,
    pub title: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InsertedTaskGroup {
    pub title: String,
}
