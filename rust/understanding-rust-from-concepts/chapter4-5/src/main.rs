use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    data: i32,
    child: Option<Weak<RefCell<Node>>>,
}

fn print_link(start_node: Rc<RefCell<Node>>) {
    let mut p = Rc::clone(&start_node);
    loop {
        println!("{}", p.borrow().data);
        if p.borrow().child.is_none()
            || Weak::upgrade(p.borrow().child.as_ref().unwrap())
                .unwrap()
                .as_ptr()
                == start_node.as_ptr()
        {
            println!("reached the first node");
            break;
        }
        let ptmp = Weak::upgrade(p.borrow().child.as_ref().unwrap()).unwrap();
        p = ptmp;
    }
}

fn main() {
    let node1 = Rc::new(RefCell::new(Node {
        data: 1,
        child: None,
    }));
    let node2 = Rc::new(RefCell::new(Node {
        data: 2,
        child: None,
    }));

    node1.borrow_mut().child = Some(Rc::downgrade(&node2));
    node2.borrow_mut().child = Some(Rc::downgrade(&node1));

    println!("link from node1");
    print_link(Rc::clone(&node1));

    println!("link from node2");
    print_link(Rc::clone(&node2));
}
