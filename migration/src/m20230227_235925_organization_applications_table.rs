use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationApplications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationApplications::SvixAppId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrganizationApplications::OrganizationId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(OrganizationApplications::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("default now()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("organization_apps_organization_id_idx")
                    .table(OrganizationApplications::Table)
                    .col(OrganizationApplications::OrganizationId)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OrganizationApplications::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum OrganizationApplications {
    Table,
    SvixAppId,
    OrganizationId,
    CreatedAt,
}
