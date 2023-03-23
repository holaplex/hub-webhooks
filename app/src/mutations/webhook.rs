use std::ops::Add;

use async_graphql::{self, Context, Enum, Error, InputObject, Object, Result, SimpleObject};
use hub_core::{chrono::Utc, producer::Producer};
use sea_orm::{prelude::*, JoinType, QuerySelect, Set, TransactionTrait};
use svix::api::{EndpointIn, EndpointUpdate, Svix};

use crate::{
    entities::{organization_applications, webhook_projects, webhooks},
    objects::Webhook,
    proto::{self, webhook_events::Event, WebhookEventKey, WebhookEvents},
    AppContext,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mutation;

#[Object(name = "WebhookMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_webhook(
        &self,
        ctx: &Context<'_>,
        input: CreateWebhookInput,
    ) -> Result<CreateWebhookPayload> {
        let AppContext { db, user_id, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<WebhookEvents>>()?;
        let svix = ctx.data::<Svix>()?;

        let user_id = user_id.ok_or_else(|| Error::new("X-USER-ID header not found"))?;

        let org_app = organization_applications::Entity::find()
            .filter(organization_applications::Column::OrganizationId.eq(input.organization))
            .one(db.get())
            .await?
            .ok_or_else(|| Error::new("organization not found"))?;

        let app_id = org_app.svix_app_id;

        let create_endpoint = EndpointIn {
            channels: Some(input.projects.iter().map(ToString::to_string).collect()),
            filter_types: Some(input.filter_types.iter().map(|e| e.format()).collect()),
            version: 1,
            description: Some(input.description),
            disabled: Some(false),
            rate_limit: None,
            secret: None,
            url: input.url,
            uid: None,
        };

        let endpoint = svix
            .endpoint()
            .create(app_id.clone(), create_endpoint, None)
            .await?;

        let endpoint_secret = svix
            .endpoint()
            .get_secret(app_id, endpoint.clone().id)
            .await?;

        let webhook_active_model = webhooks::ActiveModel {
            endpoint_id: Set(endpoint.id.clone()),
            organization_id: Set(input.organization),
            updated_at: Set(None),
            created_by: Set(user_id),
            ..Default::default()
        };

        let webhook = webhook_active_model.insert(db.get()).await?;

        for project in input.projects {
            let webhook_project_active_model = webhook_projects::ActiveModel {
                webhook_id: Set(webhook.id),
                project_id: Set(project),
                ..Default::default()
            };

            webhook_project_active_model.insert(db.get()).await?;
        }

        // return the webhook object and endpoint secret
        let graphql_response = CreateWebhookPayload {
            webhook: Webhook::new(endpoint, webhook.clone()),
            secret: endpoint_secret.key,
        };

        let event = WebhookEvents {
            event: Some(Event::Created(proto::Webhook {
                organization_id: webhook.organization_id.to_string(),
                endpoint_id: webhook.endpoint_id.to_string(),
            })),
        };

        let key = WebhookEventKey {
            id: webhook.id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(graphql_response)
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn delete_webhook(
        &self,
        ctx: &Context<'_>,
        input: DeleteWebhookInput,
    ) -> Result<DeleteWebhookPayload> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;

        let svix = ctx.data::<Svix>()?;

        let (webhook, organization_application) = webhooks::Entity::find()
            .join(
                JoinType::InnerJoin,
                webhooks::Relation::OrganizationApplications.def(),
            )
            .select_also(organization_applications::Entity)
            .filter(webhooks::Column::Id.eq(input.webhook))
            .one(db.get())
            .await?
            .ok_or_else(|| Error::new("webhook not found"))?;

        let organization_application = organization_application
            .ok_or_else(|| Error::new("organization_application not found"))?;

        svix.endpoint()
            .delete(
                organization_application.svix_app_id,
                webhook.endpoint_id.clone(),
            )
            .await?;

        webhook.delete(db.get()).await?;

        Ok(DeleteWebhookPayload {
            webhook: input.webhook,
        })
    }

    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn edit_webhook(
        &self,
        ctx: &Context<'_>,
        input: EditWebhookInput,
    ) -> Result<EditWebhookPayload> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let svix = ctx.data::<Svix>()?;
        let conn = db.get();

        let webhook = webhooks::Entity::find()
            .filter(webhooks::Column::Id.eq(input.webhook))
            .one(conn)
            .await?
            .ok_or_else(|| Error::new("webhook not found"))?;

        let org_app = organization_applications::Entity::find()
            .filter(organization_applications::Column::OrganizationId.eq(webhook.organization_id))
            .one(conn)
            .await?
            .ok_or_else(|| Error::new("organization not found"))?;

        let app_id = org_app.svix_app_id;

        conn.transaction::<_, (), DbErr>(|tx| {
            let projects = input.projects.clone();

            Box::pin(async move {
                webhook_projects::Entity::delete_many()
                    .filter(webhook_projects::Column::WebhookId.eq(webhook.id))
                    .exec(tx)
                    .await?;

                let webhook_projects: Vec<webhook_projects::ActiveModel> = projects
                    .into_iter()
                    .map(|project| webhook_projects::ActiveModel {
                        webhook_id: Set(webhook.id),
                        project_id: Set(project),
                        ..Default::default()
                    })
                    .collect();

                webhook_projects::Entity::insert_many(webhook_projects)
                    .exec(tx)
                    .await?;

                Ok(())
            })
        })
        .await?;

        // get and update endpoint
        let current_endpoint = svix
            .endpoint()
            .get(app_id.clone(), webhook.endpoint_id.clone())
            .await?;

        let update_endpoint = EndpointUpdate {
            channels: Some(input.projects.iter().map(ToString::to_string).collect()),
            filter_types: Some(input.filter_types.iter().map(|e| e.format()).collect()),
            version: current_endpoint.version.add(1),
            description: Some(input.description),
            disabled: input.disabled,
            rate_limit: current_endpoint.rate_limit,
            url: input.url,
            uid: current_endpoint.uid,
        };

        let endpoint = svix
            .endpoint()
            .update(
                app_id.clone(),
                webhook.endpoint_id.clone(),
                update_endpoint,
                None,
            )
            .await?;

        let mut active_webhook: webhooks::ActiveModel = webhook.clone().into();
        active_webhook.updated_at = Set(Some(Utc::now().naive_utc()));

        active_webhook.update(conn).await?;

        Ok(EditWebhookPayload {
            webhook: Webhook::new(endpoint, webhook),
        })
    }
}

#[derive(Debug, InputObject, Clone)]
pub struct CreateWebhookInput {
    pub url: String,
    pub organization: Uuid,
    pub description: String,
    pub projects: Vec<Uuid>,
    pub filter_types: Vec<FilterType>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct CreateWebhookPayload {
    pub webhook: Webhook,
    pub secret: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Enum)]
pub enum FilterType {
    ProjectCreated,
    CustomerCreated,
    CustomerTreasuryCreated,
    ProjectWalletCreated,
    CustomerWalletCreated,
    DropCreated,
    DropMinted,
}

impl FilterType {
    #[must_use]
    pub fn format(self) -> String {
        match self {
            Self::ProjectCreated => "project.created".to_string(),
            Self::CustomerCreated => "customer.created".to_string(),
            Self::CustomerTreasuryCreated => "customer_treasury.created".to_string(),
            Self::CustomerWalletCreated => "customer_wallet.created".to_string(),
            Self::ProjectWalletCreated => "project_wallet.created".to_string(),
            Self::DropCreated => "drop.created".to_string(),
            Self::DropMinted => "drop.minted".to_string(),
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct DeleteWebhookInput {
    pub webhook: Uuid,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct DeleteWebhookPayload {
    webhook: Uuid,
}

#[derive(Debug, InputObject, Clone)]
pub struct EditWebhookInput {
    pub webhook: Uuid,
    pub url: String,
    pub description: String,
    pub projects: Vec<Uuid>,
    pub filter_types: Vec<FilterType>,
    pub disabled: Option<bool>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct EditWebhookPayload {
    pub webhook: Webhook,
}
