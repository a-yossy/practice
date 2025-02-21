use futures::executor;

struct User {
    name: String
}

struct UserId(u32);

struct Db {}

impl Db {
    async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {
        todo!()
    }
}

async fn find_user_by_id(db: Db, user_id: UserId) -> Option<User> {
    db.find_by_user_id(user_id).await
}

fn main() {
    executor::block_on(find_user_by_id(Db {}, UserId(1)));
}
