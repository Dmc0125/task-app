use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};

use backend::{
    entities::{
        label,
        prelude::{Label, TaskGroup},
        task,
    },
    establish_db_connection,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Condition};

use crate::routes::lib::{validate_len, AuthenticatedUser, ErrorResponse, SuccessResponse};

#[post("/task", data = "<data>")]
pub async fn handler(
    data: Json<NewTask>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<NewTask>>, ErrorResponse> {
    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;
    if let Err(_) = db_res {
        return Err(server_err_response);
    }
    let db = db_res.unwrap();
    // Validate task group id
    let select_existing_task_group_res = TaskGroup::find_by_id(data.task_group_id).one(&db).await;
    if let Err(_) = select_existing_task_group_res {
        return Err(server_err_response);
    }
    let existing_task_group = select_existing_task_group_res.unwrap();
    let not_found_err_msg = ErrorResponse::new(
        Some(format!(
            "Could not create task in task group with id {}, task group does not exist",
            data.task_group_id
        )),
        Status::NotFound,
    );
    match existing_task_group {
        Some(task_group) => {
            if task_group.user_id != user.user_id {
                return Err(not_found_err_msg);
            }

            // Validate lengths
            let title_len_err = validate_len(&data.title, 1, 50, "Title");
            if let Some(err) = title_len_err {
                return Err(err);
            }
            let desc_len_err = validate_len(&data.description, 1, 255, "Description");
            if let Some(err) = desc_len_err {
                return Err(err);
            }

            // Validate labels ids
            match &data.labels_ids {
                Some(li) => {
                    if li.len() > 0 {
                        let select_labels_stmt = Label::find()
                            .filter(label::Column::UserId.eq(user.user_id))
                            .filter(label::Column::WorkspaceId.eq(task_group.workspace_id));
                        let mut select_labels_condition = Condition::any();
                        for label_id in li.iter() {
                            select_labels_condition = select_labels_condition.add(
                                label::Column::Id.eq(*label_id)
                            );
                        }
                        let select_labels_res = select_labels_stmt.filter(select_labels_condition).all(&db).await;
                        if let Err(_) = select_labels_res {
                            return Err(server_err_response);
                        }
                        let selected_labels = select_labels_res.unwrap();
                        let mut not_found_labels_ids = String::from("");
                        for label_id in li.iter() {
                            if selected_labels.iter().all(|sl| &sl.id != label_id) {
                                not_found_labels_ids.push_str(&format!("{}, ", label_id));
                            }
                        }
                        if not_found_labels_ids.len() > 0 {
                            return Err(ErrorResponse::new(
                                Some(format!(
                                    "Could not create task with labels ids {}labels do not exist",
                                    not_found_labels_ids
                                )),
                                Status::Conflict,
                            ));
                        }
                    }
                }
                None => (),
            }

            let mut task_to_insert = task::ActiveModel {
                user_id: sea_orm::ActiveValue::Set(user.user_id),
                task_group_id: sea_orm::ActiveValue::Set(data.task_group_id),
                title: sea_orm::ActiveValue::Set(data.title.clone()),
                description: sea_orm::ActiveValue::Set(data.description.clone()),
                ..Default::default()
            };

            if let Some(labels_ids) = &data.labels_ids {
                if labels_ids.len() > 0 {
                    // Insert only if provided, if not leave as default (None)
                    task_to_insert.labels_ids = sea_orm::ActiveValue::Set(data.labels_ids.clone())
                }
            }

            let insert_res = task_to_insert.save(&db).await;

            if let Err(_) = insert_res {
                return Err(server_err_response);
            }
            let inserted_task = insert_res.unwrap();
            Ok(Json(SuccessResponse::new(NewTask {
                task_group_id: inserted_task.task_group_id.unwrap(),
                title: inserted_task.title.unwrap(),
                description: inserted_task.description.unwrap(),
                labels_ids: inserted_task.labels_ids.unwrap(),
            })))
        }
        None => Err(not_found_err_msg),
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewTask {
    pub task_group_id: i32,
    pub title: String,
    pub description: String,
    pub labels_ids: Option<Vec<i32>>,
}
