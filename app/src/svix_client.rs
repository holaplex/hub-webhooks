use std::collections::HashMap;

use hub_core::{clap, prelude::*};
use serde::Serialize;
use svix::api::{EventTypeIn, Svix, SvixOptions};

use crate::mutations::webhook::FilterType;

/// Arguments for establishing a database connection
#[derive(Debug, clap::Args)]
pub struct SvixArgs {
    #[arg(long, env, default_value = "http://127.0.0.1:8071")]
    svix_base_url: String,
    #[arg(long, env)]
    svix_auth_token: String,
}

impl SvixArgs {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn build_client(&self) -> Result<Svix> {
        let SvixArgs {
            svix_base_url,
            svix_auth_token,
        } = self;

        let svix_options = SvixOptions {
            debug: true,
            server_url: Some(svix_base_url.to_string()),
        };

        let svix_client = Svix::new(svix_auth_token.to_string(), Some(svix_options));

        customer_created_event(svix_client.clone()).await?;
        customer_treasury_created_event(svix_client.clone()).await?;
        customer_wallet_created_event(svix_client.clone()).await?;
        drop_created_event(svix_client.clone()).await?;
        drop_minted_event(svix_client.clone()).await?;

        Ok(svix_client)
    }
}

async fn customer_created_event(svix_client: Svix) -> Result<()> {
    let schema = Schema {
        title: "Customer created event".to_string(),
        description: "Customer was created in hub-customers service".to_string(),
        r#type: "object".to_string(),
        properties: Property {
            fields: HashMap::from([
                ("project_id".to_string(), PropertyFields {
                    description: "Project id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("customer_id".to_string(), PropertyFields {
                    description: "Customer id".to_string(),
                    r#type: "string".to_string(),
                }),
            ]),
        },
        required: vec!["project_id".to_string(), "customer_id".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema)?,
                )])),
                archived: Some(false),
                name: FilterType::CustomerCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn customer_treasury_created_event(svix_client: Svix) -> Result<()> {
    let schema = Schema {
        title: "Customer treasury created event".to_string(),
        description: "Customer treasury was created in hub-treasuries service".to_string(),
        r#type: "object".to_string(),
        properties: Property {
            fields: HashMap::from([
                ("project_id".to_string(), PropertyFields {
                    description: "Project id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("customer_id".to_string(), PropertyFields {
                    description: "Customer id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("treasury_id".to_string(), PropertyFields {
                    description: "Treasury id".to_string(),
                    r#type: "string".to_string(),
                }),
            ]),
        },
        required: vec![
            "project_id".to_string(),
            "customer_id".to_string(),
            "treasury_id".to_string(),
        ],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer treasury was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema)?,
                )])),
                archived: Some(false),
                name: FilterType::CustomerTreasuryCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn customer_wallet_created_event(svix_client: Svix) -> Result<()> {
    let schema = Schema {
        title: "Customer treasury wallet event".to_string(),
        description: "Customer treasury wallet was created in hub-treasuries service".to_string(),
        r#type: "object".to_string(),
        properties: Property {
            fields: HashMap::from([
                ("project_id".to_string(), PropertyFields {
                    description: "Project id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("customer_id".to_string(), PropertyFields {
                    description: "Customer id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("wallet_id".to_string(), PropertyFields {
                    description: "Wallet id".to_string(),
                    r#type: "string".to_string(),
                }),
            ]),
        },
        required: vec![
            "project_id".to_string(),
            "customer_id".to_string(),
            "wallet_id".to_string(),
        ],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer treasury was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema)?,
                )])),
                archived: Some(false),
                name: FilterType::CustomerWalletCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn drop_created_event(svix_client: Svix) -> Result<()> {
    let schema = Schema {
        title: "Drop created".to_string(),
        description: "A Drop was created in hub-nfts service".to_string(),
        r#type: "object".to_string(),
        properties: Property {
            fields: HashMap::from([
                ("project_id".to_string(), PropertyFields {
                    description: "Project id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("drop_id".to_string(), PropertyFields {
                    description: "Drop id".to_string(),
                    r#type: "string".to_string(),
                }),
            ]),
        },
        required: vec!["project_id".to_string(), "drop_id".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A drop was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema)?,
                )])),
                archived: Some(false),
                name: FilterType::DropCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn drop_minted_event(svix_client: Svix) -> Result<()> {
    let schema = Schema {
        title: "Drop mint created".to_string(),
        description: "A collection was minted in hub-nfts service".to_string(),
        r#type: "object".to_string(),
        properties: Property {
            fields: HashMap::from([
                ("project_id".to_string(), PropertyFields {
                    description: "Project id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("drop_id".to_string(), PropertyFields {
                    description: "Drop id".to_string(),
                    r#type: "string".to_string(),
                }),
                ("mint_id".to_string(), PropertyFields {
                    description: "Mint id".to_string(),
                    r#type: "string".to_string(),
                }),
            ]),
        },
        required: vec![
            "project_id".to_string(),
            "drop_id".to_string(),
            "mint_id".to_string(),
        ],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A collection minted event created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema)?,
                )])),
                archived: Some(false),
                name: FilterType::DropMinted.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct Schema {
    title: String,
    description: String,
    r#type: String,
    properties: Property,
    required: Vec<String>,
}

#[derive(Serialize)]
struct Property {
    #[serde(flatten)]
    fields: HashMap<String, PropertyFields>,
}

#[derive(Serialize)]
struct PropertyFields {
    description: String,
    r#type: String,
}
