use std::{future::Future, pin::Pin, rc::Rc, sync::{mpsc, Arc, Mutex}, task::{Context, Poll}, thread};

use futures::{executor, future::join_all};

pub fn main() {
  trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
      self.tweet();
      self.tweet();
    }

    fn shout(&self) {
      println!("Uooooooooohhh!!!!!!");
    }
  }

  struct Dove;
  struct Duck;

  impl Tweet for Dove {
    fn tweet(&self) {
      println!("Coo!");
    }
  }

  impl Tweet for Duck {
    fn tweet(&self) {
      println!("Quack!");
    }
  }

  let dove = Dove {};
  dove.tweet();
  dove.tweet_twice();
  dove.shout();
  let duck = Duck {};

  let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
  for bird in bird_vec {
    bird.tweet();
  }

  fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
  }

  let t1 = make_tuple(1, 2);
  let t2 = make_tuple("Hello", "World!");
  let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
  let t4 = make_tuple(3, "years old");

  struct Color {
    r: i32,
    g: i32,
    b: i32
  }
  let a = Color { r: 255, g: 255, b: 255 };
  let b = a;
  println!("{} {} {}", b.r, b.g, b.b);

  fn calc_data(data: String) -> String {
    println!("{}", data);
    data
  }
  let mut important_data = "Hello, World!".to_string();
  important_data = calc_data(important_data);
  println!("{}", important_data);

  fn calc_data2(data: &String) {
    println!("{}", data);
  }
  let important_data = "Hello, World!".to_string();
  calc_data2(&important_data);
  println!("{}", &important_data);

  let x = 5;
  let y = &x;
  let z = &x;
  dbg!(x);
  dbg!(y);
  dbg!(z);

  let mut x = 5;
  // {
  //   let y = &mut x;
  //   let z = &mut x;
  //   dbg!(y);
  //   dbg!(z);
  // }

  // {
  //   let y = &x;
  //   let z = &mut x;
  //   dbg!(y);
  //   dbg!(z);
  // }

  // let y;
  // {
  //   let x = 5;
  //   y = &x;
  //   dbg!(x);
  // }
  // dbg!(y);

  let mut x = 5;
  let y = &x;
  let z = &mut x;
  dbg!(z);
  dbg!(x);

  struct Droppable;
  impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
  }
  {
    let d = Droppable;
  }
  println!("The Droppable should be released at the end of block.");

  let handle = thread::spawn(|| {
    println!("Hello, world");
  });
  dbg!(handle.join());

  let mut handles = Vec::new();
  for x in 0..10 {
    handles.push(thread::spawn(move || {
      println!("Hello, world!: {}", x);
    }));
  }
  for handle in handles {
    let _ = handle.join();
  }

  let mut handles = Vec::new();
  let mut data = Arc::new(Mutex::new(vec![1; 10]));
  for x in 0..10 {
    let data_ref = data.clone();
    handles.push(thread::spawn(move || {
      let mut data = data_ref.lock().unwrap();
      data[x] += 1
    }));
  }
  for handle in handles {
    let _ = handle.join();
  }
  dbg!(data);

  let (tx, rx) = mpsc::channel();
  let handle = thread::spawn(move || {
    let data = rx.recv().unwrap();
    println!("{}", data);
  });
  let _ = tx.send("Hello, world!");
  let _ = handle.join();

  let mut handles = Vec::new();
  let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut snd_channels = Vec::new();
  let mut rcv_channels = Vec::new();
  for _ in 0..10 {
    let (snd_tx, snd_rx) = mpsc::channel();
    let (rcv_tx, rcv_rx) = mpsc::channel();
    snd_channels.push(snd_tx);
    rcv_channels.push(rcv_rx);

    handles.push(thread::spawn(move || {
      let mut thread_data = snd_rx.recv().unwrap();
      thread_data += 1;
      let _ = rcv_tx.send(thread_data);
    }));
  }

  for x in 0..10 {
    let _ = snd_channels[x].send(data[x]);
  }

  for x in 0..10 {
    data[x] = rcv_channels[x].recv().unwrap();
  }

  for handle in handles {
    let _ = handle.join();
  }

  dbg!(data);

  struct User {
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

  // executor::block_on(find_user_by_id(Db {}, UserId(1)));

  struct CountDown(u32);

  impl Future for CountDown {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
      if self.0 == 0 {
        Poll::Ready("Zero!!!".to_string())
      } else {
        println!("{}", self.0);
        self.0 -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
      }
    }
  }

  let countdown_future1 = CountDown(10);
  let countdown_future2 = CountDown(20);
  let cd_set = join_all(vec![countdown_future1, countdown_future2]);
  let res = executor::block_on(cd_set);
  for (i, s) in res.iter().enumerate() {
    println!("{}: {}", i, s);
  }

  async fn async_add(left: i32, right: i32) -> i32 {
    left + right
  }

  async fn something_great_async_function() -> i32 {
    let ans = async_add(2, 3).await;
    println!("{}", ans);
    ans
  }

  fn something_great_async_function2() -> impl Future<Output = i32> {
    async {
      let ans = async_add(2, 3).await;
      println!("{}", ans);
      ans
    }
  }

  async fn something_great_async_function3() -> i32 {
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(3, 4).await;
    let ans3 = async_add(4, 5).await;
    let result = ans1 + ans2 + ans3;
    result
  }

  executor::block_on(something_great_async_function());

  async fn print_result(value: i32) {
    println!("{}", value);
  }

  async fn calculate() -> i32 {
    let add1 = async_add(2, 3).await;
    print_result(add1);
    let add2 = async_add(3, 4).await;
    print_result(add2);
    async_add(add1, add2).await
  }
}
