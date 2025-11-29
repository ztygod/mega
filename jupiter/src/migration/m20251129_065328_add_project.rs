use sea_orm_migration::prelude::*;

// Step 1: Define table + columns enum
#[derive(Iden)]
enum Projects {
    Table,
    Id,
    PublicId,
    Name,
    Description,
    Private,
    ArchivedAt,
    LastActivityAt,
    OrganizationId,
    CreatorId,
    ArchivedById,
    SlackChannelId,
    MessageThreadId,
    Accessory,
    CoverPhotoPath,
    IsGeneral,
    IsDefault,
    InviteToken,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::PublicId).string().not_null())
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Description).text().null())
                    .col(
                        ColumnDef::new(Projects::Private)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Projects::ArchivedAt).date_time().null())
                    .col(
                        ColumnDef::new(Projects::LastActivityAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Projects::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Projects::CreatorId).big_integer().not_null())
                    .col(ColumnDef::new(Projects::ArchivedById).big_integer().null())
                    .col(ColumnDef::new(Projects::SlackChannelId).string().null())
                    .col(
                        ColumnDef::new(Projects::MessageThreadId)
                            .big_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(Projects::Accessory).text().null())
                    .col(ColumnDef::new(Projects::CoverPhotoPath).text().null())
                    .col(
                        ColumnDef::new(Projects::IsGeneral)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Projects::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Projects::InviteToken).text().null())
                    .col(
                        ColumnDef::new(Projects::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Projects::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("index_projects_on_public_id")
                    .table(Projects::Table)
                    .col(Projects::PublicId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_projects_on_organization_id")
                    .table(Projects::Table)
                    .col(Projects::OrganizationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_projects_on_creator_id")
                    .table(Projects::Table)
                    .col(Projects::CreatorId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_projects_on_last_activity_at")
                    .table(Projects::Table)
                    .col(Projects::LastActivityAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}
