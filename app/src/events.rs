use hub_core::{prelude::*, uuid::Uuid};
use sea_orm::{prelude::*, JoinType, QuerySelect, Set};
use serde::Serialize;
use serde_json::Value;
use svix::api::{ApplicationIn, MessageIn, Svix};

use crate::{
    db::Connection,
    entities::{organization_applications, webhook_projects, webhooks},
    mutations::webhook::FilterType,
    proto::{
        customer_events, nft_events, organization_events, treasury_events, CreationStatus,
        Organization, OrganizationEventKey,
    },
    Services,
};

/// Res
///
/// # Errors
/// This function fails if ...
#[allow(clippy::too_many_lines)]
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
                let payload = serde_json::to_value(Event {
                    event_type: FilterType::CustomerCreated.format(),
                    payload: EventPayload::CustomerCreated(CustomerCreatedPayload {
                        project_id: customer.project_id.clone(),
                        customer_id: k.id.clone(),
                    }),
                })?;

                broadcast(
                    db,
                    svix,
                    customer.project_id,
                    FilterType::CustomerCreated,
                    payload,
                )
                .await
            },
            Some(customer_events::Event::Blocked(_)) | None => Ok(()),
        },
        Services::Nfts(k, e) => match e.event {
            Some(nft_events::Event::DropCreated(drop_creation)) => {
                let creation_status = CreationStatus::from_i32(drop_creation.status)
                    .context("no creation status on the message")?;

                let payload = serde_json::to_value(Event {
                    event_type: FilterType::DropCreated.format(),
                    payload: EventPayload::DropCreated(DropCreatedPayload {
                        project_id: k.project_id.clone(),
                        drop_id: k.id,
                        creation_status: creation_status.as_str_name().to_string(),
                    }),
                })?;

                broadcast(db, svix, k.project_id, FilterType::DropCreated, payload).await
            },
            Some(nft_events::Event::DropMinted(mint_creation)) => {
                let creation_status = CreationStatus::from_i32(mint_creation.status)
                    .context("no creation status on the message")?;

                let payload = serde_json::to_value(Event {
                    event_type: FilterType::DropMinted.format(),
                    payload: EventPayload::DropMinted(DropMintedPayload {
                        project_id: k.project_id.clone(),
                        drop_id: mint_creation.drop_id,
                        mint_id: k.id,
                        creation_status: creation_status.as_str_name().to_string(),
                    }),
                })?;

                broadcast(db, svix, k.project_id, FilterType::DropMinted, payload).await
            },
            Some(nft_events::Event::MintedToCollection(payload)) => {
                let creation_status = CreationStatus::from_i32(payload.status)
                    .context("no creation status on the message")?;

                let payload = serde_json::to_value(Event {
                    event_type: FilterType::MintedToCollection.format(),
                    payload: EventPayload::MintedToCollection(MintedToCollectionPayload {
                        mint_id: k.id,
                        collection_id: payload.collection_id,
                        project_id: k.project_id.clone(),
                        status: creation_status.as_str_name().to_string(),
                    }),
                })?;

                broadcast(
                    db,
                    svix,
                    k.project_id,
                    FilterType::MintedToCollection,
                    payload,
                )
                .await
            },

            Some(nft_events::Event::CollectionCreated(payload)) => {
                let creation_status = CreationStatus::from_i32(payload.status)
                    .context("no creation status on the message")?;

                let payload = serde_json::to_value(Event {
                    event_type: FilterType::CollectionCreated.format(),
                    payload: EventPayload::CollectionCreated(CollectionCreatedPayload {
                        collection_id: k.id,
                        project_id: k.project_id.clone(),
                        status: creation_status.as_str_name().to_string(),
                    }),
                })?;

                broadcast(
                    db,
                    svix,
                    k.project_id,
                    FilterType::CollectionCreated,
                    payload,
                )
                .await
            },
            Some(_) | None => Ok(()),
        },
        Services::Treasuries(k, e) => match e.event {
            Some(treasury_events::Event::CustomerTreasuryCreated(customer)) => {
                let payload = serde_json::to_value(Event {
                    event_type: FilterType::CustomerTreasuryCreated.format(),
                    payload: EventPayload::CustomerTreasuryCreated(
                        CustomerTreasuryCreatedPayload {
                            project_id: customer.project_id.clone(),
                            customer_id: customer.customer_id,
                            treasury_id: k.id,
                        },
                    ),
                })?;

                broadcast(
                    db,
                    svix,
                    customer.project_id,
                    FilterType::CustomerTreasuryCreated,
                    payload,
                )
                .await
            },
            Some(treasury_events::Event::CustomerWalletCreated(customer)) => {
                let payload = serde_json::to_value(Event {
                    event_type: FilterType::CustomerWalletCreated.format(),
                    payload: EventPayload::CustomerWalletCreated(CustomerWalletCreatedPayload {
                        project_id: customer.project_id.clone(),
                        customer_id: customer.customer_id,
                        treasury_id: k.id,
                    }),
                })?;

                broadcast(
                    db,
                    svix,
                    customer.project_id,
                    FilterType::CustomerWalletCreated,
                    payload,
                )
                .await
            },
            Some(treasury_events::Event::ProjectWalletCreated(p)) => {
                let payload = serde_json::to_value(Event {
                    event_type: FilterType::ProjectWalletCreated.format(),
                    payload: EventPayload::ProjectWalletCreated(ProjectWalletCreatedPayload {
                        treasury_id: k.id,
                        project_id: p.project_id.clone(),
                    }),
                })?;

                broadcast(
                    db,
                    svix,
                    p.project_id,
                    FilterType::ProjectWalletCreated,
                    payload,
                )
                .await
            },
            Some(treasury_events::Event::MintTransfered(payload)) => {
                let event_payload = serde_json::to_value(Event {
                    event_type: FilterType::MintTransfered.format(),
                    payload: EventPayload::MintTransfered(MintTransferedPayload {
                        project_id: payload.project_id.clone(),
                        sender: payload.sender,
                        recipient: payload.recipient,
                        mint_id: k.id,
                    }),
                })?;

                broadcast(
                    db,
                    svix,
                    payload.project_id,
                    FilterType::MintTransfered,
                    event_payload,
                )
                .await
            },
            Some(_) | None => Ok(()),
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

async fn broadcast(
    db: Connection,
    svix: Svix,
    project_id: String,
    event_type: FilterType,
    payload: Value,
) -> Result<()> {
    let message = MessageIn {
        channels: Some(vec![project_id.clone()]),
        event_id: None,
        event_type: event_type.format(),
        payload,
        payload_retention_period: None,
    };

    let project_id = Uuid::parse_str(&project_id)?;

    let app = organization_applications::Entity::find()
        .join(
            JoinType::InnerJoin,
            organization_applications::Relation::Webhooks.def(),
        )
        .join(
            JoinType::InnerJoin,
            webhooks::Relation::WebhookProjects.def(),
        )
        .filter(webhook_projects::Column::ProjectId.eq(project_id))
        .one(db.get())
        .await?
        .context("failed to get svix app_id")?;

    svix.message()
        .create(app.svix_app_id, message, None)
        .await
        .context("failed to broadcast message")?;

    Ok(())
}

#[derive(Serialize)]
pub struct Event {
    event_type: String,
    payload: EventPayload,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum EventPayload {
    CustomerCreated(CustomerCreatedPayload),
    CustomerTreasuryCreated(CustomerTreasuryCreatedPayload),
    CustomerWalletCreated(CustomerWalletCreatedPayload),
    ProjectWalletCreated(ProjectWalletCreatedPayload),
    DropCreated(DropCreatedPayload),
    DropMinted(DropMintedPayload),
    MintTransfered(MintTransferedPayload),
    CollectionCreated(CollectionCreatedPayload),
    MintedToCollection(MintedToCollectionPayload),
}

#[derive(Serialize)]
pub struct CollectionCreatedPayload {
    collection_id: String,
    project_id: String,
    status: String,
}

#[derive(Serialize)]
pub struct MintedToCollectionPayload {
    mint_id: String,
    collection_id: String,
    project_id: String,
    status: String,
}

#[derive(Serialize)]
pub struct CustomerCreatedPayload {
    customer_id: String,
    project_id: String,
}

#[derive(Serialize)]
pub struct CustomerTreasuryCreatedPayload {
    treasury_id: String,
    project_id: String,
    customer_id: String,
}

#[derive(Serialize)]
pub struct CustomerWalletCreatedPayload {
    treasury_id: String,
    project_id: String,
    customer_id: String,
}

#[derive(Serialize)]
pub struct ProjectWalletCreatedPayload {
    treasury_id: String,
    project_id: String,
}

#[derive(Serialize)]
pub struct DropCreatedPayload {
    drop_id: String,
    project_id: String,
    creation_status: String,
}

#[derive(Serialize)]
pub struct DropMintedPayload {
    mint_id: String,
    project_id: String,
    drop_id: String,
    creation_status: String,
}

#[derive(Serialize)]
pub struct MintTransferedPayload {
    project_id: String,
    sender: String,
    recipient: String,
    mint_id: String,
}
