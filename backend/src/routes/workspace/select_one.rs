use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use backend::{
    entities::{
        prelude::{Label, Task, TaskGroup, Workspace},
        task, workspace,
    },
    establish_db_connection,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[get("/workspace/<workspace_id>")]
pub async fn handler(
    workspace_id: i32,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<FoundWorkspace>>, ErrorResponse> {
    let server_err_response = ErrorResponse::new(None, Status::InternalServerError);
    let db_res = establish_db_connection().await;

    if let Err(_) = db_res {
        return Err(server_err_response);
    }

    let db = db_res.unwrap();

    // Find workspace
    let select_workspace_res = Workspace::find_by_id(workspace_id)
        .filter(workspace::Column::UserId.eq(user.user_id))
        .find_with_related(TaskGroup)
        .all(&db)
        .await;

    if let Err(_) = select_workspace_res {
        return Err(server_err_response);
    }

    let found_models = select_workspace_res.unwrap();
    if found_models.len() == 0 {
        return Err(ErrorResponse::new(
            Some(format!("Workspace with id {} does not exist", workspace_id)),
            Status::NotFound,
        ));
    }

    let (found_workspace, found_related_task_groups) = &found_models[0];

    // Find related labels
    let related_labels = found_workspace.find_related(Label).all(&db).await;
    if let Err(_) = related_labels {
        return Err(server_err_response);
    }
    let labels: Vec<FoundLabel> = related_labels
        .unwrap()
        .iter()
        .map(|label_model| FoundLabel {
            id: label_model.id,
            color: label_model.color.clone(),
            description: label_model.description.clone(),
        })
        .collect();

    // Find tasks related tasks to related task groups
    let mut related_tasks: Vec<task::Model> = vec![];
    if found_related_task_groups.len() > 0 {
        let mut find_related_tasks_stmt =
            Task::find().filter(task::Column::UserId.eq(user.user_id));
        for task_group_model in found_related_task_groups.clone().iter() {
            find_related_tasks_stmt =
                find_related_tasks_stmt.filter(task::Column::TaskGroupId.eq(task_group_model.id))
        }
        let found_related_tasks_models = find_related_tasks_stmt.all(&db).await;
        match found_related_tasks_models {
            Err(_) => {
                return Err(server_err_response);
            }
            Ok(tasks_models) => {
                related_tasks = tasks_models;
            }
        }
    }

    // Merge task groups with related tasks
    let mut task_groups: Vec<FoundTaskGroup> = vec![];
    for task_group_model in found_related_task_groups.iter() {
        let mut current_related_tasks: Vec<FoundTask> = vec![];
        for task_model in related_tasks.iter() {
            if task_model.task_group_id == task_group_model.id {
                current_related_tasks.push(FoundTask {
                    id: task_model.id,
                    title: task_model.title.clone(),
                    description: task_model.description.clone(),
                    labels_ids: task_model.labels_ids.clone(),
                })
            }
        }

        task_groups.push(FoundTaskGroup {
            id: task_group_model.id,
            title: task_group_model.title.clone(),
            tasks: current_related_tasks,
        })
    }

    Ok(Json(SuccessResponse::new(FoundWorkspace {
        title: found_workspace.title.clone(),
        description: found_workspace.description.clone(),
        labels,
        task_groups,
    })))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FoundWorkspace {
    pub title: String,
    pub description: Option<String>,
    pub labels: Vec<FoundLabel>,
    pub task_groups: Vec<FoundTaskGroup>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FoundLabel {
    pub id: i32,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FoundTaskGroup {
    pub id: i32,
    pub title: String,
    pub tasks: Vec<FoundTask>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FoundTask {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub labels_ids: Option<Vec<i32>>,
}
