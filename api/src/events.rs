use hub_core::{prelude::*, uuid::Uuid};
use sea_orm::{prelude::*, Set};
use svix::api::{ApplicationIn, Svix};

use crate::{
    db::Connection,
    entities::organization_applications,
    proto::{organization_events, Organization, OrganizationEventKey},
    Services,
};

/// Res
///
/// # Errors
/// This function fails if ...
pub async fn process(msg: Services, db: Connection, svix: Svix) -> Result<()> {
    // match topics
    match msg {
        Services::Organizations(k, e) => match e.event {
            Some(organization_events::Event::OrganizationCreated(org)) => {
                create_svix_application(db, svix, k, org).await
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
