use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use sea_orm::{sea_query::Expr, ColumnTrait, DbErr, EntityTrait, QueryFilter, UpdateResult, Value};

use super::lib::validate_len;
use backend::{
    entities::{prelude::Workspace, workspace},
    establish_db_connection,
};

use crate::routes::lib::{AuthenticatedUser, ErrorResponse, SuccessResponse};

#[patch("/workspace/<workspace_id>", data = "<data>")]
pub async fn handler(
    workspace_id: i32,
    data: Json<ModifiedWorkspaceData>,
    user: AuthenticatedUser,
) -> Result<Json<SuccessResponse<SavedModifiedWorkspace>>, ErrorResponse> {
    if None == data.title && None == data.description {
        return Err(ErrorResponse::new(
            Some("Either title or description has to be provided".into()),
            Status::BadRequest,
        ));
    }

    let mut updated_workspace_stmt = Workspace::update_many()
        .filter(workspace::Column::Id.eq(workspace_id))
        .filter(workspace::Column::UserId.eq(user.user_id));

    let title = data.title.as_ref().unwrap();
    let title_err = validate_len(title, 1, 50, "Title");
    if let Some(err) = title_err {
        return Err(err);
    }
    updated_workspace_stmt =
        updated_workspace_stmt.col_expr(workspace::Column::Title, Expr::value(title.clone()));

    if let Some(desc) = &data.description {
        let desc_err = validate_len(desc, 0, 255, "Description");
        if let Some(err) = desc_err {
            return Err(err);
        }
        updated_workspace_stmt = match desc.len() > 0 {
            true => updated_workspace_stmt.col_expr(
                workspace::Column::Description,
                Expr::value(Some(desc.clone())),
            ),
            false => updated_workspace_stmt.col_expr(
                workspace::Column::Description,
                Expr::value(Value::String(None)),
            ),
        };
    }

    let updated_workspace_res: Result<UpdateResult, DbErr> = async move {
        let db = establish_db_connection().await?;
        let updated_workspace = updated_workspace_stmt.exec(&db).await?;
        Ok(updated_workspace)
    }
    .await;

    match updated_workspace_res {
        Ok(update_result) => {
            if update_result.rows_affected == 0 {
                return Err(ErrorResponse::new(
                    format!("Workspace with id {} does not exist", workspace_id).into(),
                    Status::NotFound,
                ));
            }

            Ok(Json(SuccessResponse::new(SavedModifiedWorkspace {
                id: workspace_id,
                title: title.clone(),
                description: data.description.clone(),
            })))
        }
        Err(_) => Err(ErrorResponse::new(None, Status::InternalServerError)),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ModifiedWorkspaceData {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SavedModifiedWorkspace {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
}
