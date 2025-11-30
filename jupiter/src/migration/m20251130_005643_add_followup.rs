use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum FollowUps {
    Table,
    Id,
    PublicId,
    OrganizationMembershipId,
    SubjectId,
    SubjectType,
    ShowAt,
    ShownAt,
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
                    .table(FollowUps::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FollowUps::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FollowUps::PublicId)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FollowUps::OrganizationMembershipId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FollowUps::SubjectId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(FollowUps::SubjectType).string().not_null())
                    .col(ColumnDef::new(FollowUps::ShowAt).date_time().not_null())
                    .col(ColumnDef::new(FollowUps::ShownAt).date_time().null())
                    .col(
                        ColumnDef::new(FollowUps::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(FollowUps::UpdatedAt)
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
                    .name("index_followups_on_public_id")
                    .table(FollowUps::Table)
                    .col(FollowUps::PublicId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_followups_on_subject")
                    .table(FollowUps::Table)
                    .col(FollowUps::SubjectId)
                    .col(FollowUps::SubjectType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_followups_on_organization_membership_id")
                    .table(FollowUps::Table)
                    .col(FollowUps::OrganizationMembershipId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FollowUps::Table).to_owned())
            .await
    }
}
