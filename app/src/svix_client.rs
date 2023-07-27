use std::collections::HashMap;

use hub_core::{clap, reqwest::StatusCode};
use serde::Serialize;
use svix::{
    api::{EventTypeIn, EventTypeOut, Svix, SvixOptions},
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
            server_url: Some(svix_base_url.into()),
        };

        let svix_client = Svix::new(svix_auth_token.into(), Some(svix_options));

        create_event_types(svix_client.clone()).await?;

        Ok(svix_client)
    }
}

macro_rules! event {
    ($event:expr, $svix:expr) => {
        match $event($svix.clone()).await {
            Ok(_) => (),
            Err(Error::Http(e)) if e.status == StatusCode::CONFLICT => (),
            Err(e) => return Err(e),
        }
    };
}

async fn create_event_types(svix_client: Svix) -> Result<(), Error> {
    event!(drop_created, svix_client);
    event!(drop_minted, svix_client);
    event!(customer_created, svix_client);
    event!(customer_treasury_created, svix_client);
    event!(customer_wallet_created, svix_client);
    event!(project_wallet_created, svix_client);
    event!(mint_transfered, svix_client);
    event!(minted_to_collection, svix_client);
    event!(collection_created, svix_client);
    Ok(())
}

async fn customer_created(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Customer created event"),
            description: "Customer was created in hub-customers service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("customer_id", Fields {
                            description: "Customer id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer was created".into(),
                schemas: Some(HashMap::from([(
                    "2".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerCreated.format(),
            },
            None,
        )
        .await
}

async fn customer_treasury_created(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Customer treasury created event"),
            description: "Customer treasury was created in hub-treasuries service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("customer_id", Fields {
                            description: "Customer id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("treasury_id", Fields {
                            description: "Treasury id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer treasury was created".into(),
                schemas: Some(HashMap::from([(
                    "2".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerTreasuryCreated.format(),
            },
            None,
        )
        .await
}

async fn customer_wallet_created(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Customer treasury wallet event"),
            description: "Customer treasury wallet was created in hub-treasuries service",
            r#type: "object",

            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("customer_id", Fields {
                            description: "Customer id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("treasury_id", Fields {
                            description: "Wallet id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A customer treasury wallet was created".into(),
                schemas: Some(HashMap::from([(
                    "2".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CustomerWalletCreated.format(),
            },
            None,
        )
        .await
}

async fn project_wallet_created(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Project treasury wallet event"),
            description: "Project treasury wallet was created in hub-treasuries service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("treasury_id", Fields {
                            description: "Wallet id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A project treasury wallet was created".into(),
                schemas: Some(HashMap::from([(
                    "2".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::ProjectWalletCreated.format(),
            },
            None,
        )
        .await
}

async fn drop_created(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Drop created"),
            description: "A Drop was created in hub-nfts service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("drop_id", Fields {
                            description: "Drop id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("creation_status", Fields {
                            description: "The status of the drop's creation",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A drop was created".into(),
                schemas: Some(HashMap::from([(
                    "3".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::DropCreated.format(),
            },
            None,
        )
        .await
}

async fn drop_minted(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Drop mint created"),
            description: "A collection was minted in hub-nfts service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("drop_id", Fields {
                            description: "Drop id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("mint_id", Fields {
                            description: "Mint id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("creation_status", Fields {
                            description: "The status of the mint creation",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A collection minted event created".into(),
                schemas: Some(HashMap::from([(
                    "3".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::DropMinted.format(),
            },
            None,
        )
        .await
}

async fn mint_transfered(svix_client: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Mint transfered event"),
            description: "A mint was transfered",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("sender", Fields {
                            description: "Sender wallet address",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("recipient", Fields {
                            description: "Recipient wallet address",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("mint_id", Fields {
                            description: "Mint id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix_client
        .event_type()
        .create(
            EventTypeIn {
                description: "A mint transfered event created".into(),
                schemas: Some(HashMap::from([(
                    "1".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::MintTransfered.format(),
            },
            None,
        )
        .await
}

async fn minted_to_collection(svix: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Collection Mint creation event"),
            description: "Status of collection mint creation in hub-nfts service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("mint_id", Fields {
                            description: "Collection Mint id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("collection_id", Fields {
                            description: "Collection id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("status", Fields {
                            description: "Collection status",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix.event_type()
        .create(
            EventTypeIn {
                description: "collection mint creation event".into(),
                schemas: Some(HashMap::from([(
                    "1".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::CollectionCreated.format(),
            },
            None,
        )
        .await
}

async fn collection_created(svix: Svix) -> Result<EventTypeOut, Error> {
    let schema = Schema {
        fields: Fields {
            title: Some("Collection creation event"),
            description: "Status of collection creation in hub-nfts service",
            r#type: "object",
            properties: Some(HashMap::from([
                ("event_type", Fields {
                    description: "Event Type",
                    r#type: "string",
                    title: None,
                    properties: None,
                }),
                ("payload", Fields {
                    description: "Event Payload",
                    r#type: "object",
                    title: None,
                    properties: Some(HashMap::from([
                        ("collection_id", Fields {
                            description: "Collection id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("project_id", Fields {
                            description: "Project id",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                        ("status", Fields {
                            description: "Collection status",
                            r#type: "string",
                            title: None,
                            properties: None,
                        }),
                    ])),
                }),
            ])),
        },
        required: vec!["event_type", "payload"],
    };

    svix.event_type()
        .create(
            EventTypeIn {
                description: "collection creation event".into(),
                schemas: Some(HashMap::from([(
                    "1".into(),
                    serde_json::to_value(schema).expect("failed to build schema"),
                )])),
                archived: Some(false),
                name: FilterType::MintedToCollection.format(),
            },
            None,
        )
        .await
}

#[derive(Serialize)]
struct Schema<'a> {
    #[serde(flatten)]
    fields: Fields<'a>,
    required: Vec<&'a str>,
}

#[derive(Serialize)]
struct Fields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    description: &'a str,
    r#type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<HashMap<&'a str, Fields<'a>>>,
}
