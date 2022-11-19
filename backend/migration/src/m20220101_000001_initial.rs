use sea_orm_migration::{
    prelude::*,
    sea_query::{extension::postgres::Type, Iden},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(SocialProviderType::Type)
                    .values([SocialProviderType::Discord, SocialProviderType::Google])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::DefaultSocialProfile)
                            .enumeration(
                                SocialProviderType::Type,
                                [SocialProviderType::Discord, SocialProviderType::Google],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SocialProfile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SocialProfile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SocialProfile::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_social_profile_id_user_id")
                            .from(SocialProfile::Table, SocialProfile::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(SocialProfile::ProviderType)
                            .enumeration(
                                SocialProviderType::Type,
                                [SocialProviderType::Discord, SocialProviderType::Google],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SocialProfile::ProviderId)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SocialProfile::ProviderUsername)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SocialProfile::ProviderAvatar)
                            .string()
                            .string_len(255),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Workspace::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Workspace::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_id_user_id")
                            .from(Workspace::Table, Workspace::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Workspace::Title)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Workspace::Description)
                            .string()
                            .string_len(255), // .default(Value::String(None))
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                table_with_id(
                    Label::Id,
                    Table::create()
                        .table(Label::Table)
                        .col(ColumnDef::new(Label::WorkspaceId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_label_id_workspace_id")
                                .from(Label::Table, Label::WorkspaceId)
                                .to(Workspace::Table, Workspace::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(ColumnDef::new(Label::UserId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_label_id_user_id")
                                .from(Label::Table, Label::UserId)
                                .to(User::Table, User::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(
                            ColumnDef::new(Label::Color)
                                .string()
                                .string_len(30)
                                .not_null(),
                        )
                        .col(ColumnDef::new(Label::Description).string().string_len(30)),
                )
                .to_owned(),
            )
            .await?;

        manager
            .create_table(
                table_with_id(
                    TaskGroup::Id,
                    Table::create()
                        .table(TaskGroup::Table)
                        .col(ColumnDef::new(TaskGroup::WorkspaceId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_task_group_id_workspace_id")
                                .from(TaskGroup::Table, TaskGroup::WorkspaceId)
                                .to(Workspace::Table, Workspace::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(ColumnDef::new(TaskGroup::UserId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_task_group_id_user_id")
                                .from(TaskGroup::Table, TaskGroup::UserId)
                                .to(User::Table, User::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(
                            ColumnDef::new(TaskGroup::Title)
                                .string()
                                .string_len(50)
                                .not_null(),
                        ),
                )
                .to_owned(),
            )
            .await?;

        manager
            .create_table(
                table_with_id(
                    Task::Id,
                    Table::create()
                        .table(Task::Table)
                        .col(ColumnDef::new(Task::TaskGroupId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_task_id_task_group_id")
                                .from(Task::Table, Task::TaskGroupId)
                                .to(TaskGroup::Table, TaskGroup::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(ColumnDef::new(Task::UserId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_task_id_user_id")
                                .from(Task::Table, Task::UserId)
                                .to(User::Table, User::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(
                            ColumnDef::new(Task::Title)
                                .string()
                                .string_len(50)
                                .not_null(),
                        )
                        .col(
                            ColumnDef::new(Task::Description)
                                .string()
                                .string_len(255)
                                .not_null(),
                        )
                        .col(ColumnDef::new(Task::LabelsIds).array(ColumnType::Integer(None))),
                )
                .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Task::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(TaskGroup::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Label::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Workspace::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(SocialProfile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(SocialProviderType::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

enum SocialProviderType {
    Type,
    Discord,
    Google,
}

impl Iden for SocialProviderType {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "social_provider_type",
                Self::Google => "google",
                Self::Discord => "discord",
            }
        )
        .unwrap();
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    DefaultSocialProfile,
}

#[derive(Iden)]
enum SocialProfile {
    Table,
    Id,
    UserId,

    ProviderType,
    ProviderId,
    ProviderUsername,
    ProviderAvatar,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
    UserId,

    Title,
    Description,
    //   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    //   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
}

#[derive(Iden)]
enum Label {
    Table,
    Id,
    WorkspaceId,
    UserId,

    Description,
    Color,
    //   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    //   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
}

#[derive(Iden)]
enum TaskGroup {
    Table,
    Id,
    WorkspaceId,
    UserId,

    Title,
    //   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    //   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
}

#[derive(Iden)]
enum Task {
    Table,
    Id,
    TaskGroupId,
    UserId,
    LabelsIds,

    Title,
    Description,
    //   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    //   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
}

pub fn table_with_id<T>(id_column: T, stmt: &mut TableCreateStatement) -> &mut TableCreateStatement
where
    T: IntoIden,
{
    stmt.col(
        ColumnDef::new(id_column)
            .integer()
            .auto_increment()
            .not_null()
            .primary_key(),
    )
}
