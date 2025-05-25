use std::task::Waker;

struct MyPrimitiveFuture {
    waker: Option<Waker>,
}

impl Future for MyPrimitiveFuture {
    type Output = String;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if true {
            return std::task::Poll::Ready("ok".to_string());
        }

        self.waker = Some(cx.waker().clone());
        std::task::Poll::Pending
    }
}

fn main() {
    println!("Hello, world!");
}
