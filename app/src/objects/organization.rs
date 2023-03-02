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
    pub async fn webhooks(&self, ctx: &Context<'_>) -> Result<Option<Vec<Webhook>>> {
        let AppContext {
            organization_webhooks_loader,
            ..
        } = ctx.data::<AppContext>()?;

        organization_webhooks_loader.load_one(self.id).await
    }

    pub async fn webhook(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Webhook>> {
        let AppContext { webhook_loader, .. } = ctx.data::<AppContext>()?;

        webhook_loader.load_one(id).await
    }
}
