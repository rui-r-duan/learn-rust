#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

use crate::List::{Cons, Nil};
use RcList::{Cons as RcCons, Nil as RcNil};
use std::rc::Rc;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    let b = RcCons(3, Rc::clone(&a));
    let c = RcCons(4, Rc::clone(&a));

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
