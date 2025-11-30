use sea_orm_migration::prelude::*;

// Step 1: Define table + columns enum
#[derive(Iden)]
enum OrganizationMemberships {
    Table,
    Id,
    PublicId,
    Role,
    OrganizationId,
    UserId,
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
                    .table(OrganizationMemberships::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationMemberships::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMemberships::PublicId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMemberships::Role)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMemberships::OrganizationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMemberships::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMemberships::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrganizationMemberships::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Indexes
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("index_organization_memberships_on_public_id")
                    .table(OrganizationMemberships::Table)
                    .col(OrganizationMemberships::PublicId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_organization_memberships_on_organization_id")
                    .table(OrganizationMemberships::Table)
                    .col(OrganizationMemberships::OrganizationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("index_organization_memberships_on_user_id")
                    .table(OrganizationMemberships::Table)
                    .col(OrganizationMemberships::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OrganizationMemberships::Table)
                    .to_owned(),
            )
            .await
    }
}
