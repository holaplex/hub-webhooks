use std::collections::HashMap;

use hub_core::{clap, reqwest::StatusCode};
use serde::Serialize;
use svix::{
    api::{EventTypeIn, Svix, SvixOptions},
    error::Error,
};

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
    pub async fn build_client(&self) -> Result<Svix, Error> {
        let SvixArgs {
            svix_base_url,
            svix_auth_token,
        } = self;

        let svix_options = SvixOptions {
            debug: true,
            server_url: Some(svix_base_url.to_string()),
        };

        let svix_client = Svix::new(svix_auth_token.to_string(), Some(svix_options));

        match create_event_types(svix_client.clone()).await {
            Ok(_) => (),
            Err(Error::Http(e)) if e.status == StatusCode::CONFLICT => (),
            Err(e) => return Err(e),
        }

        Ok(svix_client)
    }
}

async fn create_event_types(svix_client: Svix) -> Result<(), Error> {
    drop_created_event(svix_client.clone()).await?;
    drop_minted_event(svix_client.clone()).await?;
    customer_created_event(svix_client.clone()).await?;
    customer_treasury_created_event(svix_client.clone()).await?;
    customer_wallet_created_event(svix_client.clone()).await?;
    project_wallet_created_event(svix_client.clone()).await
}

async fn customer_created_event(svix_client: Svix) -> Result<(), Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Customer created event".to_string()),
            description: "Customer was created in hub-customers service".to_string(),
            r#type: "object".to_string(),
            properties: Some(HashMap::from([
                ("event_type".to_string(), Fields {
                    description: "Event Type".to_string(),
                    r#type: "string".to_string(),
                    title: None,
                    properties: None,
                }),
                ("payload".to_string(), Fields {
                    description: "Event Payload".to_string(),
                    r#type: "object".to_string(),
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id".to_string(), Fields {
                            description: "Project id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("customer_id".to_string(), Fields {
                            description: "Customer id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type".to_string(), "payload".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn customer_treasury_created_event(svix_client: Svix) -> Result<(), Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Customer treasury created event".to_string()),
            description: "Customer treasury was created in hub-treasuries service".to_string(),
            r#type: "object".to_string(),
            properties: Some(HashMap::from([
                ("event_type".to_string(), Fields {
                    description: "Event Type".to_string(),
                    r#type: "string".to_string(),
                    title: None,
                    properties: None,
                }),
                ("payload".to_string(), Fields {
                    description: "Event Payload".to_string(),
                    r#type: "object".to_string(),
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id".to_string(), Fields {
                            description: "Project id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("customer_id".to_string(), Fields {
                            description: "Customer id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("treasury_id".to_string(), Fields {
                            description: "Treasury id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type".to_string(), "payload".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer treasury was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerTreasuryCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn customer_wallet_created_event(svix_client: Svix) -> Result<(), Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Customer treasury wallet event".to_string()),
            description: "Customer treasury wallet was created in hub-treasuries service"
                .to_string(),
            r#type: "object".to_string(),

            properties: Some(HashMap::from([
                ("event_type".to_string(), Fields {
                    description: "Event Type".to_string(),
                    r#type: "string".to_string(),
                    title: None,
                    properties: None,
                }),
                ("payload".to_string(), Fields {
                    description: "Event Payload".to_string(),
                    r#type: "object".to_string(),
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id".to_string(), Fields {
                            description: "Project id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("customer_id".to_string(), Fields {
                            description: "Customer id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("treasury_id".to_string(), Fields {
                            description: "Wallet id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type".to_string(), "payload".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer treasury wallet was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerWalletCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn project_wallet_created_event(svix_client: Svix) -> Result<(), Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Project treasury wallet event".to_string()),
            description: "Project treasury wallet was created in hub-treasuries service"
                .to_string(),
            r#type: "object".to_string(),
            properties: Some(HashMap::from([
                ("event_type".to_string(), Fields {
                    description: "Event Type".to_string(),
                    r#type: "string".to_string(),
                    title: None,
                    properties: None,
                }),
                ("payload".to_string(), Fields {
                    description: "Event Payload".to_string(),
                    r#type: "object".to_string(),
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id".to_string(), Fields {
                            description: "Project id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("treasury_id".to_string(), Fields {
                            description: "Wallet id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type".to_string(), "payload".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A project treasury wallet was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerWalletCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn drop_created_event(svix_client: Svix) -> Result<(), Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Drop created".to_string()),
            description: "A Drop was created in hub-nfts service".to_string(),
            r#type: "object".to_string(),
            properties: Some(HashMap::from([
                ("event_type".to_string(), Fields {
                    description: "Event Type".to_string(),
                    r#type: "string".to_string(),
                    title: None,
                    properties: None,
                }),
                ("payload".to_string(), Fields {
                    description: "Event Payload".to_string(),
                    r#type: "object".to_string(),
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id".to_string(), Fields {
                            description: "Project id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("drop_id".to_string(), Fields {
                            description: "Drop id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type".to_string(), "payload".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A drop was created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::DropCreated.format(),
            },
            None,
        )
        .await?;

    Ok(())
}

async fn drop_minted_event(svix_client: Svix) -> Result<(), Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Drop mint created".to_string()),
            description: "A collection was minted in hub-nfts service".to_string(),
            r#type: "object".to_string(),
            properties: Some(HashMap::from([
                ("event_type".to_string(), Fields {
                    description: "Event Type".to_string(),
                    r#type: "string".to_string(),
                    title: None,
                    properties: None,
                }),
                ("payload".to_string(), Fields {
                    description: "Event Payload".to_string(),
                    r#type: "object".to_string(),
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id".to_string(), Fields {
                            description: "Project id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("drop_id".to_string(), Fields {
                            description: "Drop id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                        ("mint_id".to_string(), Fields {
                            description: "Mint id".to_string(),
                            r#type: "string".to_string(),
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type".to_string(), "payload".to_string()],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A collection minted event created".to_string(),
                schemas: Some(HashMap::from([(
                    "1".to_string(),
                    serde_json::to_value(schema).expect("failed to build schema"),
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
    #[serde(flatten)]
    fields: Fields,
    required: Vec<String>,
}

#[derive(Serialize)]
struct Fields {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    description: String,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<HashMap<String, Fields>>,
}
