use async_graphql::Object;
use hub_core::{chrono::NaiveDateTime, uuid::Uuid};
use svix::api::EndpointOut;

use crate::{entities::webhooks::Model, mutations::webhook::FilterType};

#[derive(Debug, Clone)]
pub struct Webhook {
    pub endpoint: EndpointOut,
    pub model: Model,
}

impl Webhook {
    #[must_use]
    pub fn new(endpoint: EndpointOut, model: Model) -> Self {
        Self { endpoint, model }
    }
}

#[Object]
impl Webhook {
    async fn id(&self) -> Uuid {
        self.model.id
    }

    async fn endpoint_id(&self) -> &str {
        &self.endpoint.id
    }

    async fn url(&self) -> &str {
        &self.endpoint.url
    }

    async fn events(&self) -> Vec<FilterType> {
        let filter_types = self.endpoint.filter_types.clone();

        filter_types
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.parse())
            .collect::<Result<Vec<FilterType>, _>>()
            .unwrap_or_default()
    }

    async fn description(&self) -> String {
        let description = self.endpoint.description.clone();

        description.unwrap_or_default()
    }

    async fn created_at(&self) -> NaiveDateTime {
        self.model.created_at
    }

    async fn organization_id(&self) -> Uuid {
        self.model.organization_id
    }

    async fn updated_at(&self) -> Option<NaiveDateTime> {
        self.model.updated_at
    }

    async fn created_by_id(&self) -> Uuid {
        self.model.created_by
    }

    async fn channels(&self) -> Vec<String> {
        let channels = self.endpoint.channels.clone();

        channels.unwrap_or_default()
    }
}
