use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

struct Item {
    contents: i64,
}

#[derive(Debug, Clone)]
pub struct Guest {
    name: String,
    address: String,
}

#[derive(Debug, Clone)]
pub struct Error(String);

#[derive(Debug, Default)]
pub struct GuestRegister(Vec<Guest>);
impl GuestRegister {
    pub fn register(&mut self, guest: Guest) {
        self.0.push(guest);
    }

    pub fn nth(&self, idx: usize) -> Option<&Guest> {
        self.0.get(idx)
    }
}

mod cloned {
    use super::Guest;

    #[derive(Default, Debug)]
    pub struct GuestRegister {
        by_arrival: Vec<Guest>,
        by_name: std::collections::BTreeMap<String, Guest>,
    }

    impl GuestRegister {
        pub fn register(&mut self, guest: Guest) {
            self.by_arrival.push(guest.clone());
            self.by_name.insert(guest.name.clone(), guest);
        }
        pub fn named(&self, name: &str) -> Option<&Guest> {
            self.by_name.get(name)
        }
        pub fn nth(&self, idx: usize) -> Option<&Guest> {
            self.by_arrival.get(idx)
        }
    }
}

mod indexed {
    use super::Guest;

    #[derive(Default, Debug)]
    pub struct GuestRegister {
        by_arrival: Vec<Guest>,
        by_name: std::collections::BTreeMap<String, usize>,
    }

    impl GuestRegister {
        pub fn register(&mut self, guest: Guest) {
            self.by_name
                .insert(guest.name.clone(), self.by_arrival.len());
            self.by_arrival.push(guest);
        }
        pub fn deregister(&mut self, idx: usize) -> Result<(), super::Error> {
            if idx >= self.by_arrival.len() {
                return Err(super::Error("out of bounds".to_string()));
            }
            self.by_arrival.remove(idx);

            Ok(())
        }
        pub fn named(&self, name: &str) -> Option<&Guest> {
            let idx = *self.by_name.get(name)?;
            self.nth(idx)
        }
        pub fn named_mut(&mut self, name: &str) -> Option<&mut Guest> {
            let idx = *self.by_name.get(name)?;
            self.nth_mut(idx)
        }
        pub fn nth(&self, idx: usize) -> Option<&Guest> {
            self.by_arrival.get(idx)
        }
        pub fn nth_mut(&mut self, idx: usize) -> Option<&mut Guest> {
            self.by_arrival.get_mut(idx)
        }
    }
}

mod rc {
    use super::{Error, Guest};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Default, Debug)]
    pub struct GuestRegister {
        by_arrival: Vec<Rc<RefCell<Guest>>>,
        by_name: std::collections::BTreeMap<String, Rc<RefCell<Guest>>>,
    }

    impl GuestRegister {
        pub fn register(&mut self, guest: Guest) {
            let name = guest.name.clone();
            let guest = Rc::new(RefCell::new(guest));
            self.by_arrival.push(guest.clone());
            self.by_name.insert(name, guest);
        }
        pub fn deregister(&mut self, idx: usize) -> Result<(), Error> {
            if idx >= self.by_arrival.len() {
                return Err(Error("out of bounds".to_string()));
            }
            let guest: Rc<RefCell<Guest>> = self.by_arrival.remove(idx);
            self.by_name.remove(&guest.borrow().name);

            Ok(())
        }
        pub fn named(&self, name: &str) -> Option<Rc<RefCell<Guest>>> {
            self.by_name.get(name).cloned()
        }
    }
}

struct TreeId(String);
struct BranchId(String);
struct LeafId(String);

struct Tree {
    id: TreeId,
    branches: Vec<Rc<RefCell<Branch>>>,
}

struct Branch {
    id: BranchId,
    leaves: Vec<Rc<RefCell<Leaf>>>,
    owner: Option<Weak<RefCell<Tree>>>,
}

struct Leaf {
    id: LeafId,
    owner: Option<Weak<RefCell<Branch>>>,
}

impl Branch {
    fn add_leaf(branch: Rc<RefCell<Self>>, mut leaf: Leaf) {
        leaf.owner = Some(Rc::downgrade(&branch));
        branch.borrow_mut().leaves.push(Rc::new(RefCell::new(leaf)));
    }

    fn location(&self) -> String {
        match &self.owner {
            None => format!("<unowned>.{}", self.id.0),
            Some(owner) => {
                let tree = owner.upgrade().expect("owner gone!");
                format!("{}.{}", tree.borrow().id.0, self.id.0)
            }
        }
    }
}

struct SelfRef {
    text: String,
    title: Option<std::ops::Range<usize>>,
}

fn main() {
    pub fn find<'a, 'b>(haystack: &'a str, needle: &'b str) -> Option<&'a str> {
        haystack
            .find(needle)
            .map(|i| &haystack[i..i + needle.len()])
    }

    let haystack = format!("{} to search", "Text");
    let found = find(&haystack, "ex");
    if let Some(text) = found {
        println!("Found '{text}'!");
    }

    fn check_item(item: Option<&Item>) {
        println!("called");
    }
    let x = Some(Rc::new(RefCell::new(Item { contents: 42 })));
    match x.as_ref().map(|r| r.borrow()) {
        None => check_item(None),
        Some(r) => {
            check_item(Some(r.deref()));
        }
    }

    let new_address = "123 Bigger House St";
    let mut guests = indexed::GuestRegister::default();
    let bob = Guest {
        name: "Bob".to_string(),
        address: "123 Small House St".to_string(),
    };
    guests.register(bob);
    let new_address = "123 Bigger House St";
    guests.named_mut("Bob").unwrap().address = new_address.to_string();
    assert_eq!(guests.named("Bob").unwrap().address, new_address,);

    // let mut ledger = indexed::GuestRegister::default();
    // let alice = Guest {
    //     name: "Alice".to_string(),
    //     address: "123 Small House St".to_string(),
    // };
    // let bob = Guest {
    //     name: "Bob".to_string(),
    //     address: "123 Small House St".to_string(),
    // };
    // let charlie = Guest {
    //     name: "Charlie".to_string(),
    //     address: "123 Small House St".to_string(),
    // };
    // ledger.register(alice);
    // ledger.register(bob);
    // ledger.register(charlie);
    // println!("Registered starts as:{ledger:?}");
    // ledger.deregister(0).unwrap();
    // println!("Register after deregister(0):{ledger:?}");

    // let also_alice = ledger.named("Alice");
    // println!("Found Alice: {also_alice:?}");

    // let also_bob = ledger.named("Bob");
    // println!("Found Bob: {also_bob:?}");

    // let also_charlie = ledger.named("Charlie");
    // println!("Found Charlie: {also_charlie:?}");

    let mut ledger = rc::GuestRegister::default();
    let alice = Guest {
        name: "Alice".to_string(),
        address: "123 Small House St".to_string(),
    };
    let bob = Guest {
        name: "Bob".to_string(),
        address: "123 Small House St".to_string(),
    };
    let charlie = Guest {
        name: "Charlie".to_string(),
        address: "123 Small House St".to_string(),
    };
    ledger.register(alice);
    ledger.register(bob);
    ledger.register(charlie);
    println!("Registered starts as:{ledger:?}");
    ledger.deregister(0).unwrap();
    println!("Register after deregister(0):{ledger:?}");

    let also_alice = ledger.named("Alice");
    println!("Found Alice: {also_alice:?}");

    let also_bob = ledger.named("Bob");
    println!("Found Bob: {also_bob:?}");

    let also_charlie = ledger.named("Charlie");
    println!("Found Charlie: {also_charlie:?}");
}
