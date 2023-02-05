// pub enum List {
//     Empty,
//     Elem(i32, List), // recursive without indirection [E0072]
// }

// Tail of a list allocates extra junk.
// Elements are NOT uniformly allocated.
// No null pointer optimization.
//
// layout 1:
// [] = Stack
// () = Heap
//
// [Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// Ideal layout:
// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)

// layout: (not uniformly allocated)
// [Elem A, ptr] -> (Elem B, ptr) -> (Elem C *junk*)
//
// It avoids allocating the Empty case.
// But there is an invalid state: ElemThenNotEmpty(0, Box<Empty>)
//
// Null pointer optimization is not possible.
// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

// Null pointer optimization requires
// enum Foo {
//     A,
//     B(ContainsANonNullPtr),
// }
//
// It means &, &mut, Box, Rc, Arc, Vec, and several other important types in
// Rust have no overhead when put in an Option!

// Tail of a list never allocates extra junk: check!
// enum is in delicious null-pointer-optimized form: check!
// All elements are uniformly allocated: check!
// struct Node {
//     elem: i32,
//     next: List,
// }
// pub enum List {
//     Empty,
//     More(Box<Node>), // [E0446]: private type `Node` in public interface
// }

use std::mem;

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Node {
    //         elem: elem,
    //         next: self.head, // error[E0507]: cannot move out of `self.head` which is behind a mutable reference
    //     };
    // }

    // We want to move the value of self.head out of self, and then put something back.
    // Rust still cannot accept it, because of the exception safety.
    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Box::new(Node {
    //         elem: elem,
    //         next: self.head, // error[E0507]: cannot move out of `self.head` which is behind a mutable reference
    //     });
    //     self.head = Link::More(new_node);
    // }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        // assert_eq!(list.pop(), Some(1));
        // assert_eq!(list.pop(), None);
    }
}
