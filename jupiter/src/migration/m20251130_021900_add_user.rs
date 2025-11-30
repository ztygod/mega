use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum UserTable {
    Table,
    Id,
    Name,
    Email,
    AvatarUrl,
    IsGithub,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 user 表
        manager
            .create_table(
                Table::create()
                    .table(UserTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTable::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTable::Name).text().not_null())
                    .col(ColumnDef::new(UserTable::Email).text().not_null())
                    .col(ColumnDef::new(UserTable::AvatarUrl).text().not_null())
                    .col(ColumnDef::new(UserTable::IsGithub).boolean().not_null())
                    .col(ColumnDef::new(UserTable::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(UserTable::UpdatedAt).date_time().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_user_on_email")
                    .table(UserTable::Table)
                    .col(UserTable::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    /// 回滚迁移
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTable::Table).to_owned())
            .await
    }
}
