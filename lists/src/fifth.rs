//----------------------------------------------------------------
// SELF REFERENCING SOLUTION
//----------------------------------------------------------------
// Each part of the code is technically correct in isolation (we can call push and pop once) but then the absurdity of what we created takes affect and everything just locks up.

// I'm sure there is some use for what we've written, but as far as I'm concerned it's just syntatically valid gibberish. We're saying we contain something with lifetime 'a, and that push and pop borrows self for that lifetime. That's weird but Rust can look at each part of our code individually and it doesn't see any rules being broken.

// But as soon as we try to actually use the list, the compiler quickly goes "yep you've borrowed self mutably for 'a, so you can't use self anymore until the end of 'a" but also "because you contain 'a, it must be valid for the entire list's existence".

// It's nearly a contradiction but there is one solution: as soon as you push or pop, the list "pins" itself in place and can't be accessed anymore. It has swallowed its own proverbial tail, and ascended to a world of dreams.

// error[E0499]: cannot borrow `list` as mutable more than once at a time
//   --> src/fifth.rs:78:9
//    |
// 75 |         assert_eq!(list.pop(), None);
//    |                    ---------- first mutable borrow occurs here
// ...
// 78 |         list.push(1);
//    |         ^^^^^^^^^^^^
//    |         |
//    |         second mutable borrow occurs here
//    |         first borrow later used here

// pub struct List<'a, T> {
//     head: Link<T>,
//     tail: Option<&'a mut Node<T>>,
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<'a, T> Default for List<'a, T> {
//     fn default() -> Self {
//         List::new()
//     }
// }

// impl<'a, T> List<'a, T> {
//     pub fn new() -> Self {
//         List {
//             head: None,
//             tail: None,
//         }
//     }

//     pub fn push(&'a mut self, elem: T) {
//         let new_tail = Box::new(Node {
//             elem,
//             // When you push onto the tail, your next is always None
//             next: None,
//         });

//         let new_tail = match self.tail.take() {
//             Some(old_tail) => {
//                 // If the old tail existed, update it to point to the new tail
//                 old_tail.next = Some(new_tail);
//                 old_tail.next.as_deref_mut()
//             }
//             None => {
//                 // Otherwise, update the head to point to it
//                 self.head = Some(new_tail);
//                 self.head.as_deref_mut()
//             }
//         };

//         self.tail = new_tail
//     }

//     pub fn pop(&'a mut self) -> Option<T> {
//         // Grab the list's current head
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;

//             // If we're out of `head`, make sure to set the tail to `None`.
//             if self.head.is_none() {
//                 self.tail = None;
//             }

//             head.elem
//         })
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::List;

//     #[test]
//     fn basics() {
//         let mut list = List::default();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), None);
//     }
// }
//----------------------------------------------------------------
// END SELF REFERENCING SOLUTION
//----------------------------------------------------------------

//----------------------------------------------------------------
// RAW POINTER AND BOX SOLUTION
//----------------------------------------------------------------

// Unsafe Rust is a superset of Safe Rust. It's completely the same as Safe
// Rust in all its semantics and rules, you're just allowed to do a few extra
// things that are wildly unsafe and can cause the dreaded Undefined Behaviour
// that haunts C.

// Raw pointers are basically C's pointers. They have no inherent aliasing
// rules. They have no lifetimes. They can be null. They can be
// misaligned. They can be dangling. They can point to uninitialized
// memory. They can be cast to and from integers. They can be cast to point to
// a different type. Mutability? Cast it. Pretty much everything goes, and that
// means pretty much anything can go wrong.

// There are two kinds of raw pointer: *const T and *mut T.

// You can only dereference a *const T to an &T, but much like the mutability
// of a variable, this is just a lint against incorrect usage. At most it just
// means you have to cast the *const to a *mut first. Although if you don't
// actually have permission to mutate the referent of the pointer, you're gonna
// have a bad time.

// For now, *mut T == &unchecked mut T!
// use std::ptr;

// pub struct List<T> {
//     head: Link<T>,
//     tail: *mut Node<T>, // DANGER DANGER
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List {
//             head: None,
//             tail: ptr::null_mut(), // 0 as *mut _
//         }
//     }

//     //     |
//     // 497 | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
//     //     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     //     | |
//     //     | trying to retag from <92962> for Unique permission at alloc24416[0x0], but that tag does not exist in the borrow stack for this location
//     //     | this error occurs as part of retag at alloc24416[0x0..0x8]
//     //     |
//     //     = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
//     //     = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
//     // help: <92962> was created by a SharedReadWrite retag at offsets [0x0..0x10]
//     //    --> src/fifth.rs:176:32
//     //     |
//     // 176 |         let raw_tail: *mut _ = &mut *new_tail;
//     //     |                                ^^^^^^^^^^^^^^
//     // help: <92962> was later invalidated at offsets [0x0..0x10] by a Unique retag
//     //    --> src/fifth.rs:185:30
//     //     |
//     // 185 |             self.head = Some(new_tail);
//     //     |                              ^^^^^^^^
//     pub fn push(&mut self, elem: T) {
//         let mut new_tail = Box::new(Node { elem, next: None });

//         let raw_tail: *mut _ = &mut *new_tail;

//         if !self.tail.is_null() {
//             // Hello Compiler, I Know I Am Doing Something Dangerous And
//             // I Promise To Be A Good Programmer Who Never Makes Mistakes.
//             unsafe {
//                 (*self.tail).next = Some(new_tail);
//             }
//         } else {
//             self.head = Some(new_tail);
//         }

//         self.tail = raw_tail;
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;

//             if self.head.is_none() {
//                 self.tail = ptr::null_mut()
//             }

//             head.elem
//         })
//     }
// }

//----------------------------------------------------------------
// END RAW POINTER AND BOX SOLUTION
//----------------------------------------------------------------

//----------------------------------------------------------------
// PURE RAW POINTER SOLUTION
//----------------------------------------------------------------
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>; // much better

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            // Immediately convert the Box into a raw pointer
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                // RISE FROM THE GRAVE
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

//----------------------------------------------------------------
// END PURE RAW POINTER SOLUTION
//----------------------------------------------------------------

#[cfg(test)]
mod test {
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
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }

    #[test]
    fn test_option_as_deref_mut() {
        let mut x: Option<String> = Some("hey".to_owned());
        let y = x.as_deref_mut().map(|x| {
            x.make_ascii_uppercase();
            x
        });
        let mut tmp = "HEY".to_owned();
        let z = Some(tmp.as_mut_str());
        assert_eq!(y, z);

        let xx = x.as_deref().map(|x| x.to_ascii_uppercase());
        assert_eq!(xx, Some(tmp));
    }
}
