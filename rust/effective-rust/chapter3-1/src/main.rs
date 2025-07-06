fn main() {
    fn caller() -> u64 {
        let x = 42u64;
        let y = 19u64;
        f(x) + g(y)
    }

    fn f(f_param: u64) -> u64 {
        let two = 2u64;
        f_param + two
    }

    fn g(g_param: u64) -> u64 {
        let arr = [2u64, 3u64];
        g_param + arr[1]
    }

    #[derive(Debug, Clone)]
    pub struct Item {
        contents: u32,
    }
    {
        let item1 = Item { contents: 1 };
        let item2 = Item { contents: 2 };
        println!("item1 = {item1:?}, item2 = {item2:?}");
    }

    let a = 10;
    let b = 20;
    let x = f((a + b) * 2);
    let x = {
        let temp1 = a + b;
        {
            let temp2 = temp1 * 2;
            f(temp2)
        }
    };

    // let r: &Item;
    // {
    //     let item = Item { contents: 42 };
    //     r = &item;
    // }
    // println!("r.contents = {}", r.contents);

    // fn fn_returning_ref(item: &mut Item) -> &Item {
    //     item
    // }
    // let r: &Item = fn_returning_ref(&mut Item { contents: 42 });
    // println!("r.contents = {}", r.contents);

    let mut s: String = "Hello".to_string();
    let greeting = &mut s[..5];
    greeting.make_ascii_uppercase();
    let r: &str = &s;
    println!("s = '{}'", r);

    pub fn first(data: &[Item]) -> Option<&Item> {
        if data.is_empty() {
            None
        } else {
            Some(&data[0])
        }
    }

    pub fn find<'a, 'b>(haystack: &'a [u8], needle: &'b [u8]) -> Option<&'a [u8]> {
        None
    }

    pub fn smaller<'a>(left: &'a Item, right: &'a Item) -> &'a Item {
        if left.contents < right.contents {
            left
        } else {
            right
        }
    }

    {
        let outer = Item { contents: 7 };
        {
            let inner = Item { contents: 8 };
            {
                let min = smaller(&inner, &outer);
                println!("smaller of {inner:?} ans {outer:?} is {min:?}");
            }
        }
    }

    {
        let haystack = b"123";
        let found = {
            let needle = b"234";
            find(haystack, needle)
        };
        println!("found = {:?}", found);
    }

    fn f2(x: &Item) -> (&Item, &Item) {
        todo!()
    }
    fn f3(x: &Item, y: &Item, z: &Item) -> i32 {
        todo!()
    }
    // fn f4(&self, y: &Item, z: &Item) -> &Item {
    //     todo!()
    // }

    static ANSWER: Item = Item { contents: 42 };
    pub fn the_answer() -> &'static Item {
        &ANSWER
    }

    pub struct Wrapper(pub i32);
    impl Drop for Wrapper {
        fn drop(&mut self) {}
    }

    // const ANSWER2: Wrapper = Wrapper(42);
    // pub fn the_answer2() -> &'static Wrapper {
    //     &ANSWER2
    // }

    // {
    //     let boxed = Box::new(Item { contents: 42 });
    //     let r: &'static Item = &boxed;
    //     println!("static item is {:?}", r);
    // }

    {
        let boxed = Box::new(Item { contents: 42 });
        let r: &'static Item = Box::leak(boxed);
        println!("static item is {:?}", r);
    }

    {
        let b: Box<Item> = Box::new(Item { contents: 42 });
    }

    {
        let b: Box<Item> = Box::new(Item { contents: 42 });
        let bb: Box<Box<Item>> = Box::new(b);
    }

    pub struct ReferenceHolder<'a> {
        pub index: usize,
        pub item: &'a Item,
    }

    pub struct RefHolderHolder<'a> {
        pub inner: ReferenceHolder<'a>,
    }

    pub struct LargestCommonSubstring<'a, 'b> {
        pub left: &'a str,
        pub right: &'b str,
    }
    pub fn find_common<'a, 'b>(
        left: &'a str,
        right: &'b str,
    ) -> Option<LargestCommonSubstring<'a, 'b>> {
        todo!()
    }

    pub struct RepeatedSubstring<'a> {
        pub first: &'a str,
        pub second: &'a str,
    }
    pub fn find_repeat<'a>(s: &'a str) -> Option<RepeatedSubstring<'a>> {
        todo!()
    }

    pub fn find_one_item(items: &[Item]) -> RefHolderHolder<'_> {
        todo!()
    }

    #[derive(Debug)]
    pub struct Item2 {
        pub contents: i64,
    }
    pub fn replace(item: &mut Option<Item2>, val: Item2) -> Option<Item2> {
        item.replace(val)
    }

    fn both_zero(left: &Item, right: &Item) -> bool {
        left.contents == 0 && right.contents == 0
    }
    let item = Item { contents: 0 };
    assert!(both_zero(&item, &item));

    fn zero_both(left: &mut Item, right: &mut Item) {
        left.contents = 0;
        right.contents = 0;
    }
    let mut item = Item { contents: 0 };
    // zero_both(&mut item, &mut item);

    fn copy_contents(left: &mut Item, right: &Item) {
        left.contents = right.contents;
    }
    let mut item = Item { contents: 0 };
    // copy_contents(&mut item, &item);

    // let mut item = Item {contents: 0 };
    // let r = &item;
    // item.contents = 42;
    // // (&mut item).contents = 0;
    // println!("reference to item is {:?}", r);

    let item = Item { contents: 42 };
    let r = &item;
    let contents = item.contents;
    // let contents = (&item).contents;
    println!("reference to item is {:?}", r);

    // let mut item = Item { contents: 42 };
    // let r = &mut item;
    // let contents = item.contents;
    // r.contents = 0;

    // let item = Item { contents: 42 };
    // let r = &item;
    // let new_item = item;
    // println!("reference to item is {:?}", r);

    let item = Item { contents: 42 };
    let r = &item;
    println!("reference to item is {:?}", r);
    let new_item = item;
}
