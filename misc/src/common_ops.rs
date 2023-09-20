// How to get reference?
// How to get mutable reference?
// How to dereference?
// How to get the wrapped data?
// How to set the wrapped data?
// How to get a wrapped struct's data member?
// How to set a wrapped struct's data member?
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct SNode<T> {
    elem: T,
    next: Option<Rc<SNode<T>>>,
}

#[derive(Debug)]
struct DNode<T> {
    elem: T,
    next: Option<Rc<RefCell<DNode<T>>>>,
    prev: Option<Rc<RefCell<DNode<T>>>>,
}

pub struct SList<T> {
    head: Option<Rc<SNode<T>>>,
}

pub struct DList<T> {
    head: Option<Rc<RefCell<DNode<T>>>>,
    tail: Option<Rc<RefCell<DNode<T>>>>,
}

impl<T> SList<T> {
    pub fn prepend(&self, elem: T) -> Self {
        SList {
            head: Some(Rc::new(SNode {
                elem,
                // next: self.head.as_ref().map(|rcnode| Rc::clone(rcnode)),
                next: self.head.clone(),
            })),
        }
    }
}

fn print_slist(head: &Option<Rc<SNode<&str>>>) {
    println!("{head:?}");
}

fn main() {
    let node1 = Some(Rc::new(SNode {
        elem: "A",
        next: None,
    }));
    println!("{node1:?}");
    let node2 = Some(Rc::new(SNode {
        elem: "B",
        next: node1, // data moved
    }));
    println!("{node2:?}");
    print_slist(&node2);
    let node3 = Some(Rc::new(SNode {
        elem: "C",
        next: node2,
    }));
    print_slist(&node3);
    let mut node4 = Some(Rc::new(SNode {
        elem: "D",
        next: node3.as_ref().unwrap().next.clone(),
    }));
    print_slist(&node4);
    let p = node3
        .as_ref()
        .map(|n| n.next.as_ref().map(|n| n.next.as_ref().map(|n| n.elem)));
    let q = node3
        .as_deref()
        .map(|n| n.next.as_deref().map(|n| n.next.as_deref().map(|n| n.elem)));
    println!("{p:?}, {q:?}");

    // How to get reference?
    // How to get mutable reference?
    // How to dereference?
    // How to get the wrapped data?
    // How to set the wrapped data?
    // How to get a wrapped struct's data member?
    // How to set a wrapped struct's data member?

    // let node2 = Some(DNode {
    //     elem: "B",
    //     next: None,
    //     prev: None,
    // });
    // println!("{node2:?}");
}
