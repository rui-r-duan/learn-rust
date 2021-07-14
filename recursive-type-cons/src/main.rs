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

#[derive(Debug)]
enum RcCellList {
    Cons(Rc<RefCell<i32>>, Rc<RcCellList>),
    Nil,
}

use crate::List::{Cons, Nil};
use RcList::{Cons as RcCons, Nil as RcNil};
use RcCellList::{Cons as RcCellCons, Nil as RcCellNil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // List
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("-------- Cons list using Box<T> --------");
    println!("list = {:?}", list);

    // RcList
    println!("-------- Cons list using Rc<T> --------");
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c = {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    // RcCellList
    // Among the above three versions of cons, RcCellList is the closest to
    // Lisp cons cells.
    println!("-------- Cons list using Rc<T> and RefCell<T> --------");
    let value = Rc::new(RefCell::new(5));

    let aa = Rc::new(RcCellCons(Rc::clone(&value), Rc::new(RcCellNil)));

    let bb = RcCellCons(Rc::new(RefCell::new(3)), Rc::clone(&aa));
    let cc = RcCellCons(Rc::new(RefCell::new(4)), Rc::clone(&aa));

    println!("aa before = {:?}", aa);
    println!("bb before = {:?}", bb);
    println!("cc before = {:?}", cc);

    *value.borrow_mut() += 10;

    println!("aa after = {:?}", aa);
    println!("bb after = {:?}", bb);
    println!("cc after = {:?}", cc);

    // similar things in Common Lisp
    // CL-USER> (defparameter *a* (cons 5 nil))
    // *A*
    // CL-USER> *a*
    // (5)
    // CL-USER> (defparameter *b* (cons 3 *a*))
    // *B*
    // CL-USER> *b*
    // (3 5)
    // CL-USER> (defparameter *c* (cons 4 *a*))
    // *C*
    // CL-USER> *c*
    // (4 5)
    // CL-USER> (incf (car *a*) 10)
    // 15
    // CL-USER> *a*
    // (15)
    // CL-USER> *b*
    // (3 15)
    // CL-USER> *c*
    // (4 15)
}
