use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use crate::{objects::Webhook, AppContext};

#[derive(SimpleObject, Debug, Clone)]
#[graphql(complex)]
pub struct Organization {
    #[graphql(external)]
    pub id: Uuid,
}

#[ComplexObject]
impl Organization {
    /// Retrieves a list of all webhooks associated with the organization.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context object representing the current request.
    ///
    /// # Returns
    ///
    /// A vector of all Webhook objects associated with the Organization, or None if there are none.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data context cannot be retrieved.
    pub async fn webhooks(&self, ctx: &Context<'_>) -> Result<Option<Vec<Webhook>>> {
        let AppContext {
            organization_webhooks_loader,
            ..
        } = ctx.data::<AppContext>()?;

        organization_webhooks_loader.load_one(self.id).await
    }

    /// Retrieves a specific webhook associated with the organization, based on its ID.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context object representing the current request.
    /// * `id` - The UUID of the Webhook to retrieve.
    ///
    /// # Returns
    ///
    /// The specified Webhook object, or None if it does not exist.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data context cannot be retrieved.
    pub async fn webhook(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Webhook>> {
        let AppContext { webhook_loader, .. } = ctx.data::<AppContext>()?;

        webhook_loader.load_one(id).await
    }
}
