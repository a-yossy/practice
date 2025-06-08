// async fn forward<T>(rx: Receiver<T>, tx: Sender<T>) {
//     while let Some(t) = rx.next().await {
//         tx.send(t).await;
//     }
// }

// enum Forward<T> { // 1
//     WaitingForReceive(ReceiveFuture<T>, Option<Sender<T>>),
//     WaitingForSend(SendFuture<T>, Option<Receiver<T>>),
// }

// impl<T> Future for Forward<T> {
//     type Output = (); // 2
//     fn poll(&mut self) -> Poll<Self::Output> {
//         match self { // 3
//             Forward::WaitingForReceive(recv, tx) => {
//                 if let Poll::Ready((rx, v)) = recv.poll() {
//                     if let Some(v) = v {
//                         let tx = tx.take().unwrap(); // 4
//                         *self = Forward::WaitingForSend(tx.send(v), Some(rx)); // 5
//                         // Try to make progress on sending.
//                         return self.poll(); // 6
//                     } else {
//                         // No more items.
//                         Poll::Ready(())
//                     }
//                 } else {
//                     Poll::Pending
//                 }
//             }
//             Forward::WaitingForSend(send, rx) => {
//                 if let Poll::Ready(tx) = send.poll() {
//                     let rx = rx.take().unwrap();
//                     *self = Forward::WaitingForReceive(rx.receive(), Some(tx));
//                     // Try to make progress on receiving.
//                     return self.poll();
//                 } else {
//                     Poll::Pending
//                 }
//             }
//         }
//     }
// }

// async fn try_forward<T>(rx: Receiver<T>, tx: Sender<T>) -> Option<impl Future> {
//     let mut f = forward(rx, tx);
//     if f.poll().is_pending() { Some(f) } else { None }
// }

trait Future {
    type Output;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output>;
}

use std::{
    ops::{Deref, DerefMut},
    task::Context,
};

struct Pin<P> {
    pointer: P,
}

impl<P> Pin<P>
where
    P: Deref,
{
    pub unsafe fn new_unchecked(pointer: P) -> Self {
        todo!()
    }
}

impl<'a, T> Pin<&'a mut T> {
    pub unsafe fn get_unchecked_mut(self) -> &'a mut T {
        todo!()
    }
}

impl<P> Deref for Pin<P>
where
    P: Deref,
{
    type Target = P::Target;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<P> Pin<P>
where
    P: Deref,
    P::Target: Unpin,
{
    pub fn new(pointer: P) -> Self {
        todo!()
    }
}

impl<P> DerefMut for Pin<P>
where
    P: DerefMut,
    P::Target: Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

macro_rules! pin_mut {
    ($var:ident) => {
        let mut $var = $var;
        let mut $var = unsafe { Pin::new_unchecked(&mut $var) };
    };
}

struct MySelfRefStruct {
    value: i32,
    ptr_to_value: *const i32,
}

impl MySelfRefStruct {
    fn new(value: i32) -> Self {
        let mut s = MySelfRefStruct {
            value,
            ptr_to_value: std::ptr::null(),
        };
        s.ptr_to_value = &s.value;
        s
    }

    fn check_ptr(&self) {
        println!("Value: {}, Ptr points to: {}", self.value, unsafe {
            *self.ptr_to_value
        });
    }
}

async fn handle_client(socket: std::net::TcpStream) -> Result<(), std::io::Error> {
    Ok(())
}

async fn server(socket: std::net::TcpListener) -> Result<(), std::io::Error> {
    while let Some(stream) = socket.accept().await? {
        spawn(handle_client(stream));
    }
}

fn main() {
    println!("Hello, world!");
}
