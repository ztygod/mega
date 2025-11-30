use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Permissions {
    Table,
    Id,
    PublicId,
    UserId,
    SubjectId,
    SubjectType,
    Action,
    DiscardedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 permissions 表
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permissions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permissions::PublicId)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Permissions::UserId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Permissions::SubjectId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Permissions::SubjectType).string().not_null())
                    .col(ColumnDef::new(Permissions::Action).integer().not_null())
                    .col(ColumnDef::new(Permissions::DiscardedAt).date_time().null())
                    .col(
                        ColumnDef::new(Permissions::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Permissions::UpdatedAt)
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
                    .name("index_permissions_on_public_id")
                    .table(Permissions::Table)
                    .col(Permissions::PublicId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_permissions_on_subject")
                    .table(Permissions::Table)
                    .col(Permissions::SubjectId)
                    .col(Permissions::SubjectType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_permissions_on_user_id")
                    .table(Permissions::Table)
                    .col(Permissions::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await
    }
}
