pub use sea_orm_migration::prelude::*;

mod m20230124_165007_webhooks_table;
mod m20230227_194931_organization_applications;
mod m20230227_202311_create_webhook_projects_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230124_165007_webhooks_table::Migration),
            Box::new(m20230227_194931_organization_applications::Migration),
            Box::new(m20230227_202311_create_webhook_projects_table::Migration),
        ]
    }
}
