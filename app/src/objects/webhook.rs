use async_graphql::{Object, Result};
use hub_core::{chrono::NaiveDateTime, uuid::Uuid};
use svix::api::EndpointOut;

use crate::{entities::webhooks::Model, mutations::webhook::FilterType};

/// A webhook represents an endpoint registered to receive notifications for specific events within a project.
#[derive(Debug, Clone)]
pub struct Webhook {
    /// The endpoint that the webhook is registered to.
    pub endpoint: EndpointOut,
    /// The database model for the webhook.
    pub model: Model,
}

impl Webhook {
    #[must_use]
    pub fn new(endpoint: EndpointOut, model: Model) -> Self {
        Self { endpoint, model }
    }
}

/// A webhook represents an endpoint registered to receive notifications for specific events within a project.
#[Object]
impl Webhook {
    /// Retrieves the ID of the webhook.
    async fn id(&self) -> Uuid {
        self.model.id
    }

    /// Retrieves the ID of the webhook's endpoint.
    async fn endpoint_id(&self) -> &str {
        &self.endpoint.id
    }

    /// Retrieves the URL of the webhook's endpoint.
    async fn url(&self) -> &str {
        &self.endpoint.url
    }

    /// Retrieves the events the webhook is subscribed to.
    async fn events(&self) -> Result<Vec<FilterType>> {
        let filter_types = self.endpoint.filter_types.clone();

        filter_types
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.parse())
            .collect::<Result<Vec<FilterType>, _>>()
            .map_err(Into::into)
    }

    /// Retrieves the webhook's description.
    async fn description(&self) -> String {
        let description = self.endpoint.description.clone();

        description.unwrap_or_default()
    }

    /// Retrieves the creation datetime of the webhook.
    async fn created_at(&self) -> NaiveDateTime {
        self.model.created_at
    }

    /// Retrieves the ID of the organization the webhook belongs to.
    async fn organization_id(&self) -> Uuid {
        self.model.organization_id
    }

    /// Retrieves the last update datetime of the webhook.
    async fn updated_at(&self) -> Option<NaiveDateTime> {
        self.model.updated_at
    }

    /// Retrieves the ID of the user who created the webhook.
    async fn created_by_id(&self) -> Uuid {
        self.model.created_by
    }

    /// Retrieves the channels the webhook is subscribed to.
    async fn channels(&self) -> Vec<String> {
        let channels = self.endpoint.channels.clone();

        channels.unwrap_or_default()
    }
}
