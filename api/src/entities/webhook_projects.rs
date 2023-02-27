//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "webhook_projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub webhook_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub project_id: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::webhooks::Entity",
        from = "Column::WebhookId",
        to = "super::webhooks::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Webhooks,
}

impl Related<super::webhooks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Webhooks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
