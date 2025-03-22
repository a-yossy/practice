use async_graphql::Subscription;
use futures::Stream;

use crate::{photo::Photo, simple_broker::SimpleBroker, user::User};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_photo(&self) -> impl Stream<Item = Photo> {
        SimpleBroker::<Photo>::subscribe()
    }

    async fn new_user(&self) -> impl Stream<Item = User> {
        SimpleBroker::<User>::subscribe()
    }
}
