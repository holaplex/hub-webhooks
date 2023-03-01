pub use sea_orm_migration::prelude::*;

mod m20230227_235925_organization_applications_table;
mod m20230227_235932_webhooks_table;
mod m20230227_235936_webhook_projects_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230227_235925_organization_applications_table::Migration),
            Box::new(m20230227_235932_webhooks_table::Migration),
            Box::new(m20230227_235936_webhook_projects_table::Migration),
        ]
    }
}
