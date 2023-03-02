use std::collections::HashMap;

use async_graphql::{
    dataloader::Loader as DataLoader, futures_util::future::join_all, FieldError, Result,
};
use poem::async_trait;
use sea_orm::{prelude::*, JoinType, QuerySelect};
use svix::api::{EndpointOut, Svix};

use crate::{
    db::Connection,
    entities::{organization_applications, webhooks},
    objects::Webhook,
};

#[derive(Clone)]
pub struct WebhookLoader {
    pub db: Connection,
    pub svix: Svix,
}

impl WebhookLoader {
    #[must_use]
    pub fn new(db: Connection, svix: Svix) -> Self {
        Self { db, svix }
    }
}

#[async_trait]
impl DataLoader<Uuid> for WebhookLoader {
    type Error = FieldError;
    type Value = Webhook;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let webhooks_and_organization_application = webhooks::Entity::find()
            .join(
                JoinType::InnerJoin,
                webhooks::Relation::OrganizationApplications.def(),
            )
            .select_also(organization_applications::Entity)
            .filter(webhooks::Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        let endpoint_fetchs_with_webhooks = webhooks_and_organization_application
            .into_iter()
            .filter_map(|(webhook, organization_application)| {
                organization_application.map(|organization_application| {
                    let svix = self.svix.clone();

                    fetch_endpoint(organization_application.svix_app_id, webhook, svix)
                })
            });

        let collected_endpoints_with_metadata = join_all(endpoint_fetchs_with_webhooks)
            .await
            .into_iter()
            .collect::<Result<Vec<(webhooks::Model, EndpointOut)>, _>>()?;

        Ok(collected_endpoints_with_metadata
            .into_iter()
            .map(|(model, endpoint)| (model.id, Webhook::new(endpoint, model)))
            .collect())
    }
}

#[derive(Clone)]
pub struct WebhooksLoader {
    pub db: Connection,
    pub svix: Svix,
}

impl WebhooksLoader {
    #[must_use]
    pub fn new(db: Connection, svix: Svix) -> Self {
        Self { db, svix }
    }
}

#[async_trait]
impl DataLoader<Uuid> for WebhooksLoader {
    type Error = FieldError;
    type Value = Vec<Webhook>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let organization_applications_and_webhooks = organization_applications::Entity::find()
            .join(
                JoinType::InnerJoin,
                organization_applications::Relation::Webhooks.def(),
            )
            .select_with(webhooks::Entity)
            .filter(
                organization_applications::Column::OrganizationId
                    .is_in(keys.iter().map(ToOwned::to_owned)),
            )
            .all(self.db.get())
            .await?;

        let endpoint_fetchs_with_webhooks = organization_applications_and_webhooks
            .into_iter()
            .flat_map(|(organization_application, webhooks)| {
                webhooks.into_iter().map(move |webhook| {
                    let svix = self.svix.clone();

                    fetch_endpoint(organization_application.svix_app_id.clone(), webhook, svix)
                })
            });

        let collected_endpoints_with_metadata = join_all(endpoint_fetchs_with_webhooks)
            .await
            .into_iter()
            .collect::<Result<Vec<(webhooks::Model, EndpointOut)>, _>>()?;

        Ok(collected_endpoints_with_metadata.into_iter().fold(
            HashMap::<Uuid, Vec<Webhook>>::new(),
            |mut acc, (webhook, endpoint)| {
                acc.entry(webhook.organization_id).or_insert_with(Vec::new);

                acc.entry(webhook.organization_id)
                    .and_modify(|webhooks| webhooks.push(Webhook::new(endpoint, webhook)));

                acc
            },
        ))
    }
}

async fn fetch_endpoint(
    svix_app_id: String,
    webhook: webhooks::Model,
    svix: Svix,
) -> Result<(webhooks::Model, EndpointOut)> {
    let svix_endpoint = svix.endpoint();
    let endpoint = svix_endpoint
        .get(svix_app_id, webhook.endpoint_id.clone())
        .await?;

    Ok((webhook, endpoint))
}
