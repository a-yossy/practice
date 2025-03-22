use async_graphql::{Enum, Subscription};
use futures::{Stream, StreamExt};

use crate::{photo::Photo, simple_broker::SimpleBroker};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_photo(&self) -> impl Stream<Item = Photo> {
        SimpleBroker::<Photo>::subscribe()
    }
}
