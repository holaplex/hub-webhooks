//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "webhooks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub endpoint_id: String,
    pub organization_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub created_by: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization_applications::Entity",
        from = "Column::OrganizationId",
        to = "super::organization_applications::Column::OrganizationId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OrganizationApplications,
    #[sea_orm(has_many = "super::webhook_projects::Entity")]
    WebhookProjects,
}

impl Related<super::organization_applications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrganizationApplications.def()
    }
}

impl Related<super::webhook_projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebhookProjects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}