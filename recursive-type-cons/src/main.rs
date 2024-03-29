// Some implementations are bad although they just work.

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RefList<'a> {
    Cons(i32, &'a RefList<'a>),
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

#[derive(Debug)]
enum RefCellList {
    Cons(i32, RefCell<Rc<RefCellList>>),
    Nil,
}

impl RefCellList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCellList>>> {
        match self {
            RefCellCons(_, item) => Some(item),
            RefCellNil => None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};
use RcCellList::{Cons as RcCellCons, Nil as RcCellNil};
use RcList::{Cons as RcCons, Nil as RcNil};
use RefCellList::{Cons as RefCellCons, Nil as RefCellNil};
use RefList::{Cons as RefCons, Nil as RefNil};

fn main() {
    // List
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("-------- Cons(i32, Box<List>) --------");
    println!("list = {:?}", list);

    // RefList
    // List
    let list = RefCons(1, &RefCons(2, &(RefCons(3, &RefNil))));

    println!("-------- Cons(i32, &RefList) --------");
    println!("list = {:?}", list);

    // RcList
    println!("-------- Cons(i32, Rc<List>) --------");
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
    println!("-------- Cons(Rc<Refcell<i32>, Rc<RcCellList>) --------");
    let value = Rc::new(RefCell::new(5));

    let aa = Rc::new(RcCellCons(Rc::clone(&value), Rc::new(RcCellNil)));

    let bb = RcCellCons(Rc::new(RefCell::new(3)), Rc::clone(&aa));
    let cc = RcCellCons(Rc::new(RefCell::new(4)), Rc::clone(&aa));

    println!("aa before = {:?}", aa);
    println!("bb before = {:?}", bb);
    println!("cc before = {:?}", cc);

    // if the "car" part of "cons" were not an Rc<T>, Rust will not allow
    // `value` to borrow the ref below after its ownership is moved into `aa`.
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

    // RefCellList
    println!("-------- Cons(i32, RefCell<Rc<List>>) --------");
    let a = Rc::new(RefCellCons(5, RefCell::new(Rc::new(RefCellNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(RefCellCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
