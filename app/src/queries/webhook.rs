use async_graphql::{Context, Error, Object, Result, SimpleObject, Value};
use hub_core::serde_json;
use sea_orm::prelude::*;
use svix::api::{EventTypeOut, Svix};

use crate::{objects::Webhook, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "WebhookQuery")]
impl Query {
    /// Returns a list of event types that an external service can subscribe to.
    ///
    /// # Returns
    ///
    /// A vector of EventType objects representing the different event types that can be subscribed to.
    ///
    /// # Errors
    ///
    /// This function returns an error if there was a problem with retrieving the event types.
    async fn event_types(&self, ctx: &Context<'_>) -> Result<Vec<EventType>> {
        let svix = ctx.data::<Svix>()?;

        let event_types = svix.event_type().list(None).await?;

        event_types
            .data
            .iter()
            .map(|d| d.clone().try_into())
            .collect::<_>()
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_webhook_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Option<Webhook>> {
        let AppContext { webhook_loader, .. } = ctx.data::<AppContext>()?;

        webhook_loader.load_one(id).await
    }
}

/// An event to which an external service can subscribe.
#[derive(Clone, Debug, PartialEq, SimpleObject)]
#[graphql(concrete(name = "EventType", params()))]
pub struct EventType {
    /// Whether the event is archived or not.
    pub archived: Option<bool>,
    /// The date and time when the event was created, in string format.
    pub created_at: String,
    /// A description of the event.
    pub description: String,
    /// The name of the event.
    pub name: String,
    /// The JSON schema for the event payload.
    pub schemas: Json,
    /// The date and time when the event was last updated, in string format.
    pub updated_at: String,
}

impl TryFrom<EventTypeOut> for EventType {
    type Error = Error;

    fn try_from(
        EventTypeOut {
            archived,
            created_at,
            description,
            name,
            schemas,
            updated_at,
        }: EventTypeOut,
    ) -> Result<Self> {
        let schema: Value = serde_json::to_string(&schemas)?.into();
        let json = schema.into_json()?;

        Ok(Self {
            archived,
            created_at,
            description,
            name,
            schemas: json,
            updated_at,
        })
    }
}
