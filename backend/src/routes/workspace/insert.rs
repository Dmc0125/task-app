use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use backend::{entities::workspace, establish_db_connection};
use sea_orm::{ActiveModelTrait, ActiveValue};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[post("/workspace", data = "<data>")]
pub async fn handler(
    data: Json<NewWorkspace>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<InsertedWorkspace>>, ErrorResponse> {
    if data.title.len() > 50 && data.title.len() < 1 {
        return Err(ErrorResponse::new(
            "Title length must be between 50 and 1 characters",
            Status::BadRequest,
        ));
    }

    match &data.description {
        Some(desc) => {
            if desc.len() > 255 || desc.len() < 1 {
                return Err(ErrorResponse::new(
                    "Description length must be between 255 and 1 characters",
                    Status::BadRequest,
                ));
            }
        }
        None => (),
    }

    let db_res = establish_db_connection().await;
    if let Err(_) = db_res {
        return Err(ErrorResponse::new(
            "Unknown Error",
            Status::InternalServerError,
        ));
    }
    let db = db_res.unwrap();

    let inserted_workspace_res = workspace::ActiveModel {
        user_id: ActiveValue::Set(user.user_id),
        title: ActiveValue::Set(data.title.clone()),
        description: ActiveValue::Set(data.description.clone()),
        ..Default::default()
    }
    .save(&db)
    .await;

    if let Err(_) = inserted_workspace_res {
        return Err(ErrorResponse::new("", Status::InternalServerError));
    }

    let inserted_workspace = inserted_workspace_res.unwrap();
    Ok(Json(SuccessResponse::new(InsertedWorkspace {
        id: inserted_workspace.id.unwrap(),
        title: inserted_workspace.title.unwrap(),
        description: inserted_workspace.description.unwrap(),
    })))
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewWorkspace {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InsertedWorkspace {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
}
