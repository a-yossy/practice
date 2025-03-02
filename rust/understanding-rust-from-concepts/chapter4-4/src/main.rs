use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    data: i32,
    child: Option<Rc<RefCell<Node>>>,
}

fn print_link(start_node: Rc<RefCell<Node>>) {
    let mut p = start_node;
    loop {
        println!("{}", p.borrow().data);
        if p.borrow().child.is_none() {
            break;
        }
        let ptmp = Rc::clone(p.borrow().child.as_ref().unwrap());
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
    let node3 = Rc::new(RefCell::new(Node {
        data: 3,
        child: None,
    }));

    node1.borrow_mut().child = Some(Rc::clone(&node3));
    node2.borrow_mut().child = Some(Rc::clone(&node3));

    println!("link from node1");
    print_link(Rc::clone(&node1));

    println!("link from node2");
    print_link(Rc::clone(&node2));
}
