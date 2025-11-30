use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Comments {
    Table,
    Id,
    SubjectId,
    SubjectType,
    OrganizationMembershipId,
    BodyHtml,
    ParentId,
    ResolvedAt,
    ResolvedById,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 comments 表
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comments::SubjectId).big_integer().not_null())
                    .col(ColumnDef::new(Comments::SubjectType).string().not_null())
                    .col(
                        ColumnDef::new(Comments::OrganizationMembershipId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comments::BodyHtml).text().not_null())
                    .col(ColumnDef::new(Comments::ParentId).big_integer().null())
                    .col(ColumnDef::new(Comments::ResolvedAt).date_time().null())
                    .col(ColumnDef::new(Comments::ResolvedById).big_integer().null())
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Comments::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("index_comments_on_subject")
                    .table(Comments::Table)
                    .col(Comments::SubjectId)
                    .col(Comments::SubjectType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_comments_on_organization_membership_id")
                    .table(Comments::Table)
                    .col(Comments::OrganizationMembershipId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_comments_on_parent_id")
                    .table(Comments::Table)
                    .col(Comments::ParentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}
