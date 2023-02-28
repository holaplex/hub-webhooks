use hub_core::{prelude::*, uuid::Uuid};
use sea_orm::{prelude::*, JoinType, QuerySelect, Set};
use serde::Serialize;
use svix::api::{ApplicationIn, MessageIn, Svix};

use crate::{
    db::Connection,
    entities::{organization_applications, webhook_projects, webhooks},
    mutations::webhook::FilterType,
    proto::{
        customer_events, organization_events, Customer, CustomerEventKey, CustomerEvents,
        Organization, OrganizationEventKey,
    },
    Services,
};

/// Res
///
/// # Errors
/// This function fails if ...
pub async fn process(msg: Services, db: Connection, svix: Svix) -> Result<()> {
    // match topics
    match msg {
        Services::Organizations(k, e) => match e.event {
            Some(organization_events::Event::OrganizationCreated(org)) => {
                create_svix_application(db, svix, k, org).await
            },
            Some(_) | None => Ok(()),
        },
        Services::Customers(k, e) => match e.event {
            Some(customer_events::Event::Created(customer)) => {
                broadcast_customer_created_event(db, svix, k, customer).await
            },
            None => Ok(()),
        },
    }
}

async fn create_svix_application(
    db: Connection,
    svix: Svix,
    k: OrganizationEventKey,
    org: Organization,
) -> Result<()> {
    let app = svix
        .application()
        .create(
            ApplicationIn {
                name: org.name,
                rate_limit: None,
                uid: Some(k.id),
            },
            None,
        )
        .await
        .context("failed to create svix application for org")?;

    let org_id = Uuid::parse_str(&org.id)?;

    let org_app = organization_applications::ActiveModel {
        svix_app_id: Set(app.id),
        organization_id: Set(org_id),
        ..Default::default()
    };

    org_app.insert(db.get()).await?;

    Ok(())
}

async fn broadcast_customer_created_event(
    db: Connection,
    svix: Svix,
    key: CustomerEventKey,
    customer: Customer,
) -> Result<()> {
    let message = MessageIn {
        channels: Some(vec![customer.project_id.clone()]),
        event_id: None,
        event_type: FilterType::CustomerCreated.format(),
        payload: serde_json::to_value(CustomerCreatedEvent {
            project_id: customer.project_id.clone(),
            customer_id: key.id,
        })?,
        payload_retention_period: None,
    };

    let (_, app) = webhook_projects::Entity::find()
        .select_also(organization_applications::Entity)
        .join(
            JoinType::InnerJoin,
            webhooks::Relation::OrganizationApplications.def(),
        )
        .filter(webhook_projects::Column::ProjectId.eq(customer.project_id))
        .one(db.get())
        .await?
        .context("failed to get svix app_id")?;

    let app_model = app.context("no application found")?;

    svix.message()
        .create(app_model.svix_app_id, message, None)
        .await
        .context("failed to broadcast customer.created message")?;

    Ok(())
}

#[derive(Serialize)]
pub struct CustomerCreatedEvent {
    project_id: String,
    customer_id: String,
}
