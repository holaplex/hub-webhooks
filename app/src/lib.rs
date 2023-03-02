#![feature(async_closure)]
#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod dataloaders;
pub mod db;
#[allow(clippy::pedantic)]
pub mod entities;
pub mod events;
pub mod handlers;
pub mod mutations;
pub mod objects;
pub mod queries;
pub mod svix_client;

use async_graphql::{
    dataloader::DataLoader,
    extensions::{ApolloTracing, Logger},
    EmptySubscription, Schema,
};
use dataloaders::{WebhookLoader, WebhooksLoader};
use db::Connection;
use hub_core::{
    anyhow::{Error, Result},
    clap,
    consumer::RecvError,
    prelude::*,
    tokio,
    uuid::Uuid,
};
use mutations::Mutation;
use poem::{async_trait, FromRequest, Request, RequestBody};
use queries::Query;
use svix::api::Svix;

#[allow(clippy::pedantic)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/organization.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/customer.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/treasury.proto.rs"));
}

#[derive(Debug)]
pub enum Services {
    Organizations(proto::OrganizationEventKey, proto::OrganizationEvents),
    Customers(proto::CustomerEventKey, proto::CustomerEvents),
    Treasuries(proto::TreasuryEventKey, proto::TreasuryEvents),
}

impl hub_core::consumer::MessageGroup for Services {
    const REQUESTED_TOPICS: &'static [&'static str] =
        &["hub-orgs", "hub-customers", "hub-treasuries"];

    fn from_message<M: hub_core::consumer::Message>(msg: &M) -> Result<Self, RecvError> {
        let topic = msg.topic();
        let key = msg.key().ok_or(RecvError::MissingKey)?;
        let val = msg.payload().ok_or(RecvError::MissingPayload)?;
        info!(topic, ?key, ?val);

        match topic {
            "hub-orgs" => {
                let key = proto::OrganizationEventKey::decode(key)?;
                let val = proto::OrganizationEvents::decode(val)?;

                Ok(Services::Organizations(key, val))
            },
            "hub-customers" => {
                let key = proto::CustomerEventKey::decode(key)?;
                let val = proto::CustomerEvents::decode(val)?;

                Ok(Services::Customers(key, val))
            },
            "hub-treasuries" => {
                let key = proto::TreasuryEventKey::decode(key)?;
                let val = proto::TreasuryEvents::decode(val)?;

                Ok(Services::Treasuries(key, val))
            },
            t => Err(RecvError::BadTopic(t.into())),
        }
    }
}

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env, default_value_t = 3008)]
    pub port: u16,

    #[command(flatten)]
    pub db: db::DbArgs,

    #[command(flatten)]
    pub svix: svix_client::SvixArgs,
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Debug, Clone, Copy)]
pub struct UserID(Option<Uuid>);

impl TryFrom<&str> for UserID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let id = Uuid::from_str(value)?;

        Ok(Self(Some(id)))
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for UserID {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let id = req
            .headers()
            .get("X-USER-ID")
            .and_then(|value| value.to_str().ok())
            .map_or(Ok(Self(None)), Self::try_from)?;

        Ok(id)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub connection: Connection,
    pub svix_client: Svix,
}

impl AppState {
    #[must_use]
    pub fn new(schema: AppSchema, connection: Connection, svix_client: Svix) -> Self {
        Self {
            schema,
            connection,
            svix_client,
        }
    }
}

pub struct AppContext {
    pub db: Connection,
    pub user_id: Option<Uuid>,
    pub organization_webhooks_loader: DataLoader<WebhooksLoader>,
    pub webhook_loader: DataLoader<WebhookLoader>,
}

impl AppContext {
    #[must_use]
    pub fn new(db: Connection, user_id: Option<Uuid>, svix: Svix) -> Self {
        let organization_webhooks_loader =
            DataLoader::new(WebhooksLoader::new(db.clone(), svix.clone()), tokio::spawn);
        let webhook_loader = DataLoader::new(WebhookLoader::new(db.clone(), svix), tokio::spawn);

        Self {
            db,
            user_id,
            organization_webhooks_loader,
            webhook_loader,
        }
    }
}

/// Builds the GraphQL Schema, attaching the Database to the context
#[must_use]
pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(ApolloTracing)
        .extension(Logger)
        .enable_federation()
        .finish()
}
